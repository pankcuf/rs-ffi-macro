use std::rc::Rc;
use quote::ToTokens;
use syn::{Attribute, TypeTuple};
use syn::token::Comma;
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, ParenWrapped, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, Composer, ComposerLink, constants, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::conversion::dictionary_generic_arg_pair;
use crate::ext::Mangle;
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, ScopeContextPresentable};
use crate::presentation::{DictionaryName, InterfacePresentation, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct TupleComposer<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable> + 'static,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub type_tuple: TypeTuple,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC>,
}

impl<LANG, SPEC> TupleComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG, Expr: Clone + ScopeContextPresentable>,
          Aspect<SPEC::TYC>: ScopeContextPresentable,
          Self: AspectPresentable<SPEC::TYC> {
    pub fn new(type_tuple: &TypeTuple, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            type_tuple: type_tuple.clone(),
        }
    }
}

impl<'a, SPEC> Composer<'a> for TupleComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.type_tuple.mangle_ident_default();
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
                let (ty, args) = dictionary_generic_arg_pair::<RustFermentate, SPEC>(name.clone(), Name::Index(index), ty, source);
                args.iter().for_each(|item| {
                    from_conversions.push(item.from_conversion.present(source));
                    to_conversions.push(item.to_conversion.present(source));
                    destroy_conversions.push(item.destructor.present(source));
                });
                field_composers.push(FieldComposer::unnamed(name, FieldTypeKind::Type(ty)));
            });
        let attrs = self.compose_attributes();
        let interfaces = Depunctuated::from_iter([
            InterfacePresentation::conversion_from_root(&attrs, &types, ParenWrapped::<_, Comma>::new(from_conversions).to_token_stream(), &None),
            InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, to_conversions, &None),
            InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
            InterfacePresentation::drop(&attrs, ffi_type, destroy_conversions)
        ]);
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(ffi_name, &attrs, field_composers, interfaces))
    }
}