use std::fmt::{Debug, Formatter};
use proc_macro2::Ident;
use syn::{Attribute, FnArg, ItemFn, parse_quote, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn, Visibility};
use quote::ToTokens;
use crate::composable::{CfgAttributes, FieldComposer, FieldTypeKind};
use crate::composer::{Composer, FromConversionComposer, FromConversionFullComposer, VarComposer};
use crate::context::{ScopeContext, ScopeSearch, ScopeSearchKey};
use crate::ext::{Resolve, ToType};
use crate::lang::LangAttrSpecification;
use crate::presentable::{Expression, OwnedItemPresentableContext};
use crate::presentation::{DictionaryName, Name, RustFermentate};

#[derive(Clone)]
pub enum FnSignatureContext {
    ModFn(ItemFn),
    Impl(Type, Option<Type>, Signature),
    Bare(Ident, TypeBareFn),
    TraitInner(Type, Option<Type>, Signature)
}

impl Debug for FnSignatureContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            FnSignatureContext::ModFn(sig) =>
                format!("ModFn({})", sig.to_token_stream()),
            FnSignatureContext::Impl(self_ty, trait_ty, sig) =>
                format!("Impl(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::TraitInner(self_ty, trait_ty, sig) =>
                format!("TraitInner(self: {}, trait: {}, sig: {}", self_ty.to_token_stream(), trait_ty.to_token_stream(), sig.to_token_stream()),
            FnSignatureContext::Bare(ident, type_bare_fn) =>
                format!("Bare({}, {})", ident.to_token_stream(), type_bare_fn.to_token_stream()),
        }.as_str())
    }
}

impl FnSignatureContext {
    #[allow(unused)]
    pub fn is_trait_fn(&self) -> bool {
        match self {
            FnSignatureContext::Impl(_, Some(_), _) => true,
            _ => false
        }
    }
}
#[derive(Clone, Debug)]
pub struct FnReturnTypeComposer {
    pub presentation: ReturnType,
    pub conversion: Expression<RustFermentate, Vec<Attribute>>,
}

#[derive(Clone, Debug)]
pub struct FnArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub name_type_original: OwnedItemPresentableContext<LANG, SPEC>,
    pub name_type_conversion: Expression<RustFermentate, Vec<Attribute>>,
}

impl<LANG, SPEC> FnArgComposer<LANG, SPEC>
    where LANG: Clone,
          SPEC: LangAttrSpecification<LANG> {
    pub fn new(original: OwnedItemPresentableContext<LANG, SPEC>, conversion: Expression<RustFermentate, Vec<Attribute>>) -> Self {
        Self { name_type_original: original, name_type_conversion: conversion }
    }
}

impl<'a> Composer<'a> for PatType {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Output = FnArgComposer<RustFermentate, Vec<Attribute>>;
    fn compose(&self, source: &Self::Source) -> Self::Output {
        let (_ctx, source) = source;
        let PatType { ty, attrs, pat, .. } = self;
        FnArgComposer::new(
            OwnedItemPresentableContext::Named(FieldComposer::new(Name::Pat(*pat.clone()), FieldTypeKind::Type(ty.to_type()), true, attrs.cfg_attributes()), Visibility::Inherited),
            FromConversionFullComposer::expr_less(Name::Pat(*self.pat.clone()), ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(&self.ty).unwrap(), &source.scope)).compose(source))
    }
}

impl<'a> Composer<'a> for FnArg {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Output = FnArgComposer<RustFermentate, Vec<Attribute>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        match self {
            FnArg::Receiver(receiver) =>
                receiver.compose(source),
            FnArg::Typed(pat_ty) =>
                pat_ty.compose(source),
        }
    }
}

impl<'a> Composer<'a> for Receiver {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Output = FnArgComposer<RustFermentate, Vec<Attribute>>;

    fn compose(&self, source: &'a Self::Source) -> Self::Output {
        let (ctx, source) = source;
        let Receiver { mutability: _, reference, attrs, .. } = self;
        let (ty, name_type_conversion) = match ctx {
            FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
            FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => match maybe_trait_ty {
                Some(trait_ty) => (
                    trait_ty,
                    Expression::SelfAsTrait(<Type as Resolve<Type>>::resolve(self_ty, &source).to_token_stream())
                ),
                None => (
                    self_ty,
                    // FromConversionFullComposer::expr_less(Name::Dictionary(DictionaryName::Self_), ScopeSearch::KeyInScope(ScopeSearchKey::TypeRef(self_ty)))
                    FromConversionComposer::new(Name::Dictionary(DictionaryName::Self_), self_ty.clone(), None).compose(source)
                )
            }
            _ => panic!("Receiver in regular fn")
        };

        let var = VarComposer::new(ScopeSearch::KeyInScope(ScopeSearchKey::maybe_from_ref(ty).unwrap(), &source.scope)).compose(source);
        println!("Receiver Var: {}", var.to_token_stream());
        // let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);

        let name_type_original = OwnedItemPresentableContext::Named(FieldComposer::new(Name::Dictionary(DictionaryName::Self_), FieldTypeKind::Type(parse_quote!(#var)), true, attrs.cfg_attributes()), Visibility::Inherited);

        let name_type_conversion = if reference.is_some() {
            Expression::AsRef(name_type_conversion.into())
        } else {
            name_type_conversion
        };
        FnArgComposer::new(name_type_original, name_type_conversion)

    }
}


