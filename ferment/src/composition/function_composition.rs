use std::fmt::{Debug, Formatter};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{BareFnArg, FnArg, Generics, ItemFn, ParenthesizedGenericArguments, parse_quote, Pat, PathArguments, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn, TypePath};
use quote::{quote, ToTokens};
use syn::token::RArrow;
use crate::composer::{CommaPunctuated, Composer, Depunctuated};
use crate::composition::CfgAttributes;
use crate::context::ScopeContext;
use crate::conversion::{FieldTypeConversion, FieldTypeConversionKind, ObjectConversion, TypeCompositionConversion};
use crate::ext::{Conversion, FFIResolveExtended, FFITypeResolve, Mangle, Resolve};
use crate::holder::PathHolder;
use crate::naming::{DictionaryName, Name};
use crate::presentation::context::{FieldContext, OwnedItemPresentableContext};

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
    pub conversion: FieldContext,
}

#[derive(Clone)]
pub struct FnArgComposer {
    pub name: Option<TokenStream2>,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: FieldContext,
}

impl FnArgComposer {
    pub fn new(name: Option<TokenStream2>, original: OwnedItemPresentableContext, conversion: FieldContext) -> Self {
        Self { name, name_type_original: original, name_type_conversion: conversion }
    }
}

impl Debug for FnArgComposer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FnArgComposition")
            .field("name", &format!("{}", self.name.to_token_stream()))
            .field("name_type_original", &format!("{}", self.name_type_original))
            .field("name_type_conversion", &format!("{}", self.name_type_conversion))
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct FnSignatureComposition {
    pub is_async: bool,
    pub ident: Option<Ident>,
    pub scope: PathHolder,
    pub return_type: FnReturnTypeComposer,
    pub arguments: Depunctuated<FnArgComposer>,
    pub generics: Option<Generics>,
    pub impl_context: FnSignatureContext,
    // pub self_ty: Option<Type>,
}

impl<'a> Composer<'a> for ReturnType {
    type Source = (bool, &'a ScopeContext);
    type Result = FnReturnTypeComposer;

    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (bare, source) = source;
        match (bare, self) {
            (false, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: FieldContext::LineTermination
            },
            (false, ReturnType::Type(_, ty)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(RArrow::default(), Box::new(ty.ffi_full_dictionary_type_presenter(source))),
                conversion: ty.conversion_to(FieldContext::Obj)
            },
            (true, ReturnType::Type(token, field_type)) => FnReturnTypeComposer {
                presentation: ReturnType::Type(token.clone(), Box::new(field_type.ffi_full_dictionary_type_presenter(source))),
                conversion: FieldContext::Empty
            },
            (true, ReturnType::Default) => FnReturnTypeComposer {
                presentation: ReturnType::Default,
                conversion: FieldContext::Empty,
            }
        }
    }
}

impl<'a> Composer<'a> for CommaPunctuated<BareFnArg> {
    type Source = ScopeContext;
    type Result = Depunctuated<FnArgComposer>;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        self.iter()
            .map(|bare_fn_arg| bare_fn_arg.compose(source))
            .collect()
    }
}

impl<'a> Composer<'a> for BareFnArg {
    type Source = ScopeContext;
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let BareFnArg { ty, attrs, name, .. } = self;
        let name = name.clone().map(|(ident, _)| ident);
        println!("BareFnArg::compose: {}", ty.to_token_stream());
        FnArgComposer::new(
            name.as_ref().map(Ident::to_token_stream),
            OwnedItemPresentableContext::Named(
                FieldTypeConversion::Named(
                    Name::Optional(name),
                    FieldTypeConversionKind::Type(ty.ffi_full_dictionary_type_presenter(source)),
                    attrs.cfg_attributes_expanded()),
                false),
            FieldContext::Empty)
    }
}

impl<'a> Composer<'a> for PatType {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;
    fn compose(&self, source: &Self::Source) -> Self::Result {
        let (_ctx, source) = source;
        let PatType { ty, attrs, pat, .. } = self;
        let maybe_obj = source.maybe_object(ty);
        println!("PatType::compose: {}", maybe_obj.as_ref().map_or("Nothing".to_string(), |a| format!("{}", a)));
        let ident = match &**pat {
            Pat::Ident(PatIdent { ident, .. }) => {
                ident
                // println!("Compose PatType: {} --> {}", ty.to_token_stream(), ty.resolve(source).to_token_stream());
            },
            _ =>
                panic!("error: Arg conversion not supported: {}", quote!(#ty)),
        };

        let (conversion_kind, name_type_conversion) = match maybe_obj {
            Some(ObjectConversion::Item(composition, ..) |
                 ObjectConversion::Type(composition)) => {
                let ty = composition.ty();
                println!("PatType::compose({})", composition);
                match composition {
                    TypeCompositionConversion::Callback(..) => (
                        FieldTypeConversionKind::Type(ty.ffi_full_dictionary_type_presenter(source)),
                        {
                            let ffi_ty = ty.to_custom_or_ffi_type(source);
                            println!("TypeCompositionConversion::Callback:::: {}", ty.to_token_stream());
                            match ty {
                                Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                                    Some(last_segment) => match &last_segment.arguments {
                                        PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                                            let ret = match output {
                                                ReturnType::Default => quote!(()),
                                                ReturnType::Type(_, ty) => quote!(#ty),
                                            };
                                            FieldContext::Simple(quote!(<#ffi_ty as ferment_interfaces::FFICallback<(#inputs), #ret>>::get(&*#ident)))
                                        },
                                        _ => panic!("Non callback type: {}", ty.to_token_stream()),
                                    },
                                    None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                                },
                                _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
                            }
                            // let TypeBareFn { inputs, .. } = parse_quote!(#ty);
                            // let arg_names = inputs.iter().map(|BareFnArg { name, .. }| name.as_ref().map(|(ident, _)| ident).to_token_stream()).collect::<CommaPunctuated<TokenStream2>>();

                            // ::<(u32, [u8; 32]), Option<String>>::


                            // FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&#ident)))
                            // FieldContext::MapExpression(
                            //     FieldContext::Simple(arg_names.to_token_stream()).into(),
                            //     // FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&#ident, (#arg_names)))).into())
                            //     FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&#ident))).into())
                        }
                    ),
                    TypeCompositionConversion::Primitive(_) |
                    TypeCompositionConversion::Trait(_, _, _) |
                    TypeCompositionConversion::TraitType(_) |
                    TypeCompositionConversion::Object(_) |
                    TypeCompositionConversion::Optional(_) |
                    TypeCompositionConversion::Array(_) |
                    TypeCompositionConversion::Slice(_) |
                    TypeCompositionConversion::Tuple(_) |
                    TypeCompositionConversion::Unknown(_) |
                    TypeCompositionConversion::LocalOrGlobal(_) => (
                        FieldTypeConversionKind::Type(ty.ffi_full_dictionary_type_presenter(source)),
                        {
                            let conversion = ty.resolve(source).conversion_from(FieldContext::Simple(quote!(#ident)));
                            match ty {
                                Type::Reference(..) =>
                                    FieldContext::AsRef(conversion.into()),
                                _ => conversion
                            }
                        }
                    ),
                    TypeCompositionConversion::Imported(_, _) => panic!("error: Arg conversion (Imported) not supported: {}", quote!(#ty)),
                    TypeCompositionConversion::Bounds(bounds) => (
                        FieldTypeConversionKind::Type(bounds.ffi_full_dictionary_type_presenter(source)),

                        match bounds.bounds.len() {
                            0 => FieldContext::Simple(quote!(#ident)),
                            1 => {
                                println!("TypeCompositionConversion::Bounds:::: {}", bounds);
                                if let Some(ParenthesizedGenericArguments { inputs, .. }) = bounds.maybe_bound_is_callback(bounds.bounds.first().unwrap()) {
                                    let lambda_args = inputs.iter().enumerate().map(|(index, _ty)| Name::UnnamedArg(index)).collect::<CommaPunctuated<_>>();
                                    FieldContext::Simple(quote!(|#lambda_args| unsafe { (&*#ident).call(#lambda_args) }))
                                } else {
                                    FieldContext::From(FieldContext::Simple(quote!(#ident)).into())
                                }
                            }
                            _ => {
                                unimplemented!("Mixin as fn arg...")
                                // FieldContext::From(FieldContext::Simple(quote!(#ident)).into())
                            }
                        }

                        // match ty {
                        //     Type::Path(TypePath { qself: _, path }) => match path.segments.last() {
                        //         Some(last_segment) => match &last_segment.arguments {
                        //             PathArguments::Parenthesized(ParenthesizedGenericArguments { inputs, output, .. }) => {
                        //                 FieldContext::Simple(quote!(ferment_interfaces::FFICallback::<(#inputs), #output>::get(&#ident)))
                        //             },
                        //             _ => panic!("Non callback type: {}", ty.to_token_stream()),
                        //         },
                        //         None => unimplemented!("TODO: Non-supported optional type as generic argument (Empty last segment): {}", ty.to_token_stream()),
                        //     },
                        //     _ => unimplemented!("TODO: Non-supported optional type as generic argument (Type): {}", ty.to_token_stream()),
                        // }

                    // FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Get(quote!(&*#ident)))

                        // bounds.conversion_from(FieldContext::Simple(quote!(#ident)))
                    ),
                    TypeCompositionConversion::Fn(_) => panic!("error: Arg conversion (Fn) not supported: {}", quote!(#ty)),
                }
            }
            _ => panic!("ObjectConversion::None or Empty"),
        };
        // TODO: handle mut/const with pat
        // let full_ty = ;
        // let ty_conversion = TypeConversion::from(ty);
        // let (conversion_kind, name_type_conversion) = match ty_conversion {
        //     TypeConversion::Primitive(..) |
        //     TypeConversion::Complex(..) |
        //     TypeConversion::Generic(..) => (
        //         FieldTypeConversionKind::Type(ty.ffi_full_dictionary_type_presenter(source)),
        //         match &**pat {
        //             Pat::Ident(PatIdent { ident, .. }) => {
        //                 // println!("Compose PatType: {} --> {}", ty.to_token_stream(), ty.resolve(source).to_token_stream());
        //                 let full_ty = ty.resolve(source);
        //                 let conversion = full_ty.conversion_from(FieldContext::Simple(quote!(#ident)));
        //                 match &**ty {
        //                     Type::Reference(..) =>
        //                         FieldContext::AsRef(conversion.into()),
        //                     _ => conversion
        //
        //                 }
        //             },
        //             _ =>
        //                 panic!("error: Arg conversion not supported: {}", quote!(#ty)),
        //         }
        //     ),
        //     TypeConversion::Callback(..) => (
        //         FieldTypeConversionKind::Type(ty.ffi_full_dictionary_type_presenter(source)),
        //         match &**pat {
        //             Pat::Ident(PatIdent { ident, .. }) => {
        //                 let TypeBareFn { inputs, .. } = parse_quote!(#ty);
        //                 let arg_names = inputs.iter().map(|BareFnArg { name, .. }| name.as_ref().map(|(ident, _)| ident).to_token_stream()).collect::<CommaPunctuated<TokenStream2>>();
        //                 FieldContext::MapExpression(
        //                     FieldContext::Simple(arg_names.to_token_stream()).into(),
        //                     FieldContext::FFICallbackExpr(FFICallbackMethodExpr::Apply(quote!(&#ident, (#arg_names)))).into())
        //             },
        //             _ =>
        //                 panic!("error: Arg conversion not supported: {}", quote!(#ty)),
        //         }
        //
        //     )
        // };
        // println!("PatType: {}", name_type_conversion);
        FnArgComposer::new(
            Some(pat.to_token_stream()),
            OwnedItemPresentableContext::Named(
                FieldTypeConversion::Named(
                    Name::Pat(*pat.clone()),
                    conversion_kind,
                    attrs.cfg_attributes_expanded()),
                false),
            name_type_conversion)
    }
}

impl<'a> Composer<'a> for FnArg {
    type Source = (&'a FnSignatureContext, &'a ScopeContext);
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
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
    type Result = FnArgComposer;

    fn compose(&self, source: &'a Self::Source) -> Self::Result {
        let (ctx, source) = source;
        let Receiver { mutability, reference, attrs, .. } = self;
        match ctx {
            FnSignatureContext::ModFn(_) => panic!("receiver in mod fn"),
            FnSignatureContext::Bare(_, _) => panic!("receiver in bare fn"),
            FnSignatureContext::Impl(self_ty, maybe_trait_ty, _) |
            FnSignatureContext::TraitInner(self_ty, maybe_trait_ty, _) => {
                let (mangled_ident, name_type_conversion) = match maybe_trait_ty {
                    Some(trait_ty) => (
                        trait_ty.resolve(&source).mangle_ident_default(),
                        FieldContext::SelfAsTrait(self_ty.resolve(&source).to_token_stream())
                    ),
                    None => (
                        self_ty.resolve(&source).mangle_ident_default(),
                        FieldContext::From(FieldContext::Self_.into())
                    )
                };
                let access = mutability.as_ref().map_or(quote!(const), ToTokens::to_token_stream);
                let name_type_original = OwnedItemPresentableContext::Named(
                    FieldTypeConversion::Named(
                        Name::Dictionary(DictionaryName::Self_),
                        FieldTypeConversionKind::Type(parse_quote!(* #access #mangled_ident)),
                        attrs.cfg_attributes_expanded()),
                    false);
                let name_type_conversion = if reference.is_some() {
                    FieldContext::AsRef(name_type_conversion.into())
                } else {
                    name_type_conversion
                };
                FnArgComposer::new(None, name_type_original, name_type_conversion)
            },
        }
    }
}


