use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, Type, TypeTuple};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, SourceComposable, ComposerLink, constants, GenericComposerInfo, BasicComposerLink, FFIAspect};
use crate::context::{ScopeContext, ScopeContextLink};
use crate::conversion::{GenericArgPresentation, TypeKind};
use crate::ext::{Mangle, Resolve};
use crate::lang::{LangFermentable, RustSpecification, Specification};
use crate::presentable::{Aspect, ConversionExpressionKind, Expression, ScopeContextPresentable, TypeContext};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct TupleComposer<LANG, SPEC>
    where LANG: LangFermentable + 'static,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable> + 'static,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub type_tuple: TypeTuple,
    base: BasicComposerLink<Self, LANG, SPEC>,
}

impl<LANG, SPEC> TupleComposer<LANG, SPEC>
    where LANG: LangFermentable,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn new(type_tuple: &TypeTuple, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ScopeContextLink) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            type_tuple: type_tuple.clone(),
        }
    }
}

impl<SPEC> SourceComposable for TupleComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &Self::Source) -> Self::Output {
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());
        let mut from_conversions = CommaPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut to_conversions = CommaPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut destroy_conversions = SemiPunctuated::<<SPEC::Expr as ScopeContextPresentable>::Presentation>::new();
        let mut field_composers = Depunctuated::new();
        self.type_tuple
            .elems
            .iter()
            .enumerate()
            .for_each(|(index, ty)| {
                let name = Name::UnnamedArg(index);
                let field_name = Name::Index(index);
                let ty: Type = ty.resolve(source);
                let (kind, destroy_expr,
                    from_expr,
                    to_expr) = match TypeKind::from(&ty) {
                    TypeKind::Primitive(..) => (
                        ConversionExpressionKind::Primitive,
                        Expression::empty(),
                        Expression::ffi_ref_with_name(&name),
                        Expression::obj_name(&field_name),
                    ),
                    _ => (
                        ConversionExpressionKind::Complex,
                        Expression::dict_expr(DictionaryExpr::SelfProp(name.to_token_stream())),
                        Expression::ffi_ref_with_name(&name),
                        Expression::obj_name(&field_name)
                    ),
                };
                let item = GenericArgPresentation::<RustFermentate, SPEC>::new(
                    SPEC::Var::direct(ty.clone()),
                    Expression::ConversionExpr(FFIAspect::Destroy, kind, destroy_expr.into()),
                    Expression::ConversionExpr(FFIAspect::From, kind, from_expr.into()),
                    Expression::Named((name.to_token_stream(), Expression::ConversionExpr(FFIAspect::To, kind, to_expr.into()).into())));

                // let (ty, item) = dictionary_generic_arg_pair::<RustFermentate, SPEC>(name.clone(), Name::Index(index), ty, source);
                // args.iter().for_each(|item| {
                    from_conversions.push(item.from_conversion.present(source));
                    to_conversions.push(item.to_conversion.present(source));
                    destroy_conversions.push(item.destructor.present(source));
                // });
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions).to_token_stream(), &None),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
        ]);
        let aspect = Aspect::RawTarget(TypeContext::Struct { ident: self.type_tuple.mangle_ident_default(), attrs: vec![] });
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(aspect, &attrs, field_composers, interfaces))
    }
}
