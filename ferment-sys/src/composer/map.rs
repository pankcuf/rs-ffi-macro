use std::rc::Rc;
use quote::{quote, ToTokens};
use syn::{Attribute, parse_quote, Type};
use ferment_macro::ComposerBase;
use crate::ast::{CommaPunctuated, Depunctuated, SemiPunctuated};
use crate::composable::{AttrsModel, FieldComposer, FieldTypeKind, GenModel};
use crate::composer::{AspectPresentable, AttrComposable, BasicComposer, BasicComposerOwner, Composer, ComposerLink, constants, GenericComposerInfo};
use crate::context::ScopeContext;
use crate::conversion::{GenericArgComposer, GenericArgPresentation, GenericTypeKind, TypeKind};
use crate::ext::{Accessory, FFIVarResolve, GenericNestedArg, Mangle};
use crate::lang::{RustSpecification, Specification};
use crate::presentable::{Aspect, Expression, ScopeContextPresentable};
use crate::presentation::{DictionaryExpr, DictionaryName, InterfacePresentation, InterfacesMethodExpr, Name, RustFermentate};

#[derive(ComposerBase)]
pub struct MapComposer<LANG, SPEC>
    where LANG: Clone + 'static,
          SPEC: Specification<LANG> + 'static,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub ty: Type,
    base: BasicComposer<ComposerLink<Self>, LANG, SPEC>,
}

impl<LANG, SPEC> MapComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: Specification<LANG>,
          <SPEC as Specification<LANG>>::Expr: Clone + ScopeContextPresentable,
          Aspect<SPEC::TYC>: ScopeContextPresentable {
    pub fn new(ty: &Type, ty_context: SPEC::TYC, attrs: Vec<Attribute>, scope_context: &ComposerLink<ScopeContext>) -> Self {
        Self {
            base: BasicComposer::from(AttrsModel::from(&attrs), ty_context, GenModel::default(), constants::composer_doc(), Rc::clone(scope_context)),
            ty: ty.clone(),
        }
    }
}

impl<'a, SPEC> Composer<'a> for MapComposer<RustFermentate, SPEC>
    where SPEC: RustSpecification {
    type Source = ScopeContext;
    type Output = Option<GenericComposerInfo<RustFermentate, SPEC>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let ffi_name = self.ty.mangle_ident_default();
        let count = DictionaryName::Count;
        let keys = DictionaryName::Keys;
        let values = DictionaryName::Values;
        let count_name = Name::Dictionary(count.clone());
        let arg_0_name = Name::Dictionary(keys.clone());
        let arg_1_name = Name::Dictionary(values.clone());

        let arg_context = |arg_name: &Name| quote!(obj.#arg_name().cloned());
        let arg_args = |arg_name: &Name| CommaPunctuated::from_iter([
            DictionaryExpr::SelfProp(arg_name.to_token_stream()),
            DictionaryExpr::SelfProp(count_name.to_token_stream())]);

        let compose_arg = |arg_ty: Type, from_expr: SPEC::Expr, to_expr: SPEC::Expr, destroy_expr: SPEC::Expr|
            GenericArgPresentation::<RustFermentate, SPEC>::new(
                arg_ty,
                destroy_expr,
                Expression::map_expr(Expression::DictionaryName(DictionaryName::O), from_expr),
                to_expr);
        let compose = |arg_name: &Name, ty: &Type| match TypeKind::from(ty) {
            TypeKind::Primitive(arg_ty) =>
                GenericArgPresentation::<RustFermentate, SPEC>::new(
                    arg_ty,
                    Expression::destroy_primitive_group_tokens(arg_args(arg_name)),
                    Expression::map_tokens(DictionaryName::O, DictionaryName::O),
                    Expression::ffi_to_primitive_group_tokens(arg_context(arg_name))),
            TypeKind::Complex(arg_ty) => {
                let arg_composer = GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_group_tokens), Some(Expression::destroy_complex_group_tokens));
                compose_arg(
                    arg_ty.special_or_to_ffi_full_path_variable_type(source),
                    arg_composer.from(DictionaryName::O.to_token_stream()),
                    arg_composer.to(arg_context(arg_name)),
                    arg_composer.destroy(arg_args(arg_name).to_token_stream())
                )
            },
            TypeKind::Generic(generic_arg_ty) => {
                let (arg_composer, arg_ty) = if let GenericTypeKind::Optional(..) = generic_arg_ty {
                    match generic_arg_ty.ty() {
                        None => unimplemented!("Mixin inside generic: {}", generic_arg_ty),
                        Some(ty) => (match TypeKind::from(ty) {
                            TypeKind::Primitive(_) => GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::from_primitive_opt_tokens), Some(Expression::ffi_to_primitive_opt_group_tokens), Some(Expression::destroy_complex_group_tokens)),
                            _ => GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::from_complex_opt_tokens), Some(Expression::ffi_to_complex_opt_group_tokens), Some(Expression::destroy_complex_group_tokens)),
                        }, ty.special_or_to_ffi_full_path_variable_type(source))
                    }
                } else { (GenericArgComposer::<RustFermentate, SPEC>::new(Some(Expression::from_complex_tokens), Some(Expression::ffi_to_complex_group_tokens), Some(Expression::destroy_complex_group_tokens)), generic_arg_ty.special_or_to_ffi_full_path_variable_type(source)) };
                compose_arg(
                    arg_ty,
                    arg_composer.from(DictionaryName::O.to_token_stream()),
                    arg_composer.to(arg_context(arg_name)),
                    arg_composer.destroy(arg_args(arg_name).to_token_stream())
                )
            },
        };
        let ffi_type = self.present_ffi_aspect();
        let types = (ffi_type.clone(), self.present_target_aspect());

        let nested_types = self.ty.nested_types();
        let arg_0_presentation = compose(&arg_0_name, nested_types[0]);
        let arg_1_presentation = compose(&arg_1_name, nested_types[1]);
        let expr_from_iterator = [
            quote!(ffi_ref.#count),
            quote!(ffi_ref.#keys),
            quote!(ffi_ref.#values),
            <SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.from_conversion, source).to_token_stream(),
            <SPEC::Expr as ScopeContextPresentable>::present(&arg_1_presentation.from_conversion, source).to_token_stream(),
        ];
        let expr_to_iterator = [
            FieldComposer::<RustFermentate, SPEC>::named(count_name.clone(), FieldTypeKind::Conversion(DictionaryExpr::ObjLen.to_token_stream())),
            FieldComposer::<RustFermentate, SPEC>::named(arg_0_name.clone(), FieldTypeKind::Conversion(<SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.to_conversion, source).to_token_stream())),
            FieldComposer::<RustFermentate, SPEC>::named(arg_1_name.clone(), FieldTypeKind::Conversion(<SPEC::Expr as ScopeContextPresentable>::present(&arg_1_presentation.to_conversion, source).to_token_stream())),
        ];

        let expr_destroy_iterator = [
            <SPEC::Expr as ScopeContextPresentable>::present(&arg_0_presentation.destructor, source),
            <SPEC::Expr as ScopeContextPresentable>::present(&arg_1_presentation.destructor, source),
        ];
        let attrs = self.compose_attributes();
        Some(GenericComposerInfo::<RustFermentate, SPEC>::default(
            ffi_name,
            &attrs,
            Depunctuated::from_iter([
                FieldComposer::<RustFermentate, SPEC>::named(count_name, FieldTypeKind::Type(parse_quote!(usize))),
                FieldComposer::<RustFermentate, SPEC>::named(arg_0_name, FieldTypeKind::Type(arg_0_presentation.ty.joined_mut())),
                FieldComposer::<RustFermentate, SPEC>::named(arg_1_name, FieldTypeKind::Type(arg_1_presentation.ty.joined_mut()))
            ]),
            Depunctuated::from_iter([
                InterfacePresentation::conversion_from_root(&attrs, &types, InterfacesMethodExpr::FoldToMap(CommaPunctuated::from_iter(expr_from_iterator).to_token_stream()), &None),
                InterfacePresentation::conversion_to_boxed_self_destructured(&attrs, &types, CommaPunctuated::from_iter(expr_to_iterator), &None),
                InterfacePresentation::conversion_unbox_any_terminated(&attrs, &types, DictionaryName::Ffi, &None),
                InterfacePresentation::drop(&attrs, ffi_type, SemiPunctuated::from_iter(expr_destroy_iterator))
            ])
        ))
    }
}

