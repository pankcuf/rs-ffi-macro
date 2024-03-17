use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{BareFnArg, FnArg, Generics, parse_quote, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Type, TypeBareFn};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren, RArrow};
use crate::composer::constants;
use crate::composition::Composition;
use crate::composition::context::FnSignatureCompositionContext;
use crate::context::ScopeContext;
use crate::conversion::FieldTypeConversion;
use crate::ext::{Conversion, Mangle};
use crate::holder::PathHolder;
use crate::naming::{DictionaryFieldName, Name};
use crate::presentation::context::{FieldTypePresentableContext, OwnedItemPresentableContext};
use crate::presentation::BindingPresentation;
use crate::presentation::context::name::{Aspect, Context};
use crate::presentation::ScopeContextPresentable;
use crate::wrapped::Wrapped;

#[derive(Clone, Debug)]
pub struct FnReturnTypeComposition {
    pub presentation: ReturnType,
    pub conversion: FieldTypePresentableContext,
}

#[derive(Clone, Debug)]
pub struct FnArgComposition {
    pub name: Option<TokenStream2>,
    pub name_type_original: OwnedItemPresentableContext,
    pub name_type_conversion: TokenStream2,
}

#[derive(Clone, Debug)]
pub struct FnSignatureComposition {
    pub is_async: bool,
    pub ident: Option<Ident>,
    pub scope: PathHolder,
    pub return_type: FnReturnTypeComposition,
    pub arguments: Vec<FnArgComposition>,
    pub generics: Option<Generics>,
    pub self_ty: Option<Type>,
}

impl Composition for FnSignatureComposition {
    type Context = FnSignatureCompositionContext;
    type Presentation = BindingPresentation;

    fn present(self, composition_context: Self::Context, source: &ScopeContext) -> Self::Presentation {
        match composition_context {
            FnSignatureCompositionContext::FFIObject => {
                let arguments = self.arguments
                    .iter()
                    .map(|arg| {
                        arg.name_type_original.clone()
                    })
                    .collect::<Punctuated<_, Comma>>();

                let fn_name = self.ident.unwrap();
                // println!("present_ffi_object_fn.0: {}: scope: {}", fn_name, self.scope);

                let mut full_fn_path = self.scope.joined(&fn_name);
                if self.scope.has_belong_to_current_crate() {

                    let crate_scope = source.scope.crate_scope();
                    let local_path: Vec<_> = full_fn_path.0.segments.iter().skip(1).collect();
                    full_fn_path.0 = parse_quote!(#crate_scope::#(#local_path)::*);
                }
                let argument_conversions = self.arguments
                    .iter()
                    .map(|arg|
                        OwnedItemPresentableContext::Conversion(arg.name_type_conversion.clone()))
                    .collect::<Punctuated<_, _>>();
                let name = Name::ModFn(full_fn_path.0.clone());

                // Aspect::Target()
                let input_conversions = constants::ROUND_BRACES_FIELDS_PRESENTER((Aspect::FFI(Context::Fn { path: full_fn_path.0 }), argument_conversions)).present(source);
                let return_type = self.return_type.presentation;
                let output_conversions = self.return_type.conversion.present(source);
                // println!("present_ffi_object_fn.1: {}", name);
                // println!("present_ffi_object_fn.2: {}", quote!(arguments));
                // // println!("present_ffi_object_fn.22: {}", quote!(н#(#argument_conversions),*));
                // println!("present_ffi_object_fn.3: {}", quote!(#input_conversions));
                // println!("present_ffi_object_fn.4: {}", quote!(#output_conversions));
                BindingPresentation::RegularFunction {
                    is_async: self.is_async,
                    name,
                    arguments: arguments.present(source),
                    input_conversions,
                    return_type,
                    output_conversions
                }
            },
            FnSignatureCompositionContext::FFIObjectCallback => {
                let arguments = self.arguments
                    .iter()
                    .map(|arg| arg.name_type_original.present(source))
                    .collect();
                let output_expression = self.return_type.presentation;
                BindingPresentation::Callback {
                    name: self.ident.clone().unwrap().to_token_stream(),
                    arguments,
                    output_expression
                }
            },
            FnSignatureCompositionContext::TraitVTableInner => {
                let arguments = self.arguments.iter()
                    .map(|arg|
                        arg.name_type_original.clone())
                    .collect::<Punctuated<_, Comma>>();
                println!("present_trait_vtable_inner_fn: {:#?}", arguments);
                let fn_name = self.ident.unwrap();
                let presentation = Wrapped::<_, Paren>::new(arguments.present(source));
                let name_and_args = quote!(unsafe extern "C" fn #presentation);
                let output_expression = self.return_type.presentation;

                BindingPresentation::TraitVTableInnerFn {
                    name: Name::VTableInnerFn(fn_name),
                    name_and_args,
                    output_expression
                }
            }
            // FnSignatureCompositionContext::StaticVTable(trait_decomposition) => {
            //     let item_full_ty = context.full_type_for(&parse_quote!(#item_name));
            //     let trait_full_ty = context.full_type_for(&parse_quote!(#trait_ident));
            //     // let (vtable_methods_implentations, vtable_methods_declarations): (Vec<TraitVTablePresentation>, Vec<TraitVTablePresentation>) = trait_decomposition.methods.into_iter()
            //     // let (vtable_methods_implentations, method_names, method_signatures): (Vec<TraitVTablePresentation>, Vec<Ident>, Vec<Ident>) = trait_decomposition.methods.into_iter()
            //     let methods_compositions: Vec<TraitVTableMethodComposition> = trait_decomposition.methods.into_iter()
            //         .map(|signature_decomposition| {
            //
            //             let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
            //             let fn_name = signature_decomposition.ident.unwrap();
            //             let ffi_method_ident = format_ident!("{}_{}", item_name, fn_name);
            //             // let arguments = signature_decomposition.arguments
            //             //     .iter()
            //             //     .map(|arg| arg.name_type_original.clone())
            //             //     .collect::<Vec<_>>();
            //             //
            //             // let name_and_args = ROUND_BRACES_FIELDS_PRESENTER((quote!(unsafe extern "C" fn #ffi_method_ident), arguments)).present(context);
            //             // let argument_names = IteratorPresentationContext::Round(
            //             //     signature_decomposition.arguments
            //             //         .iter()
            //             //         .map(|arg|
            //             //             arg.name.map_or(
            //             //                 OwnedItemPresenterContext::Conversion(quote!(cast_obj)),
            //             //                 |_| OwnedItemPresenterContext::Conversion(arg.name_type_conversion.clone())))
            //             //         .collect())
            //             //     .present(context);
            //
            //             TraitVTableMethodComposition {
            //                 fn_name,
            //                 ffi_fn_name: ffi_method_ident,
            //                 signature_composition: signature_decomposition.clone()
            //             }
            //             // (TraitVTablePresentation::Method {
            //             //     fn_name: fn_name.clone(),
            //             //     sig_name: ffi_method_ident.clone(),
            //             //     argument_names,
            //             //     name_and_args,
            //             //     output_expression,
            //             //     item_type: item_full_ty.clone(),
            //             //     trait_type: trait_full_ty.clone(),
            //             //     output_conversions,
            //             // }, fn_name, ffi_method_ident)
            //         }).collect();
            //
            //     // FFIObjectPresentation::StaticVTable {
            //     //     name: f,
            //     //     methods_names: vec![],
            //     //     methods_signatures: vec![],
            //     //     fq_trait_vtable: Default::default(),
            //     //     methods_implementations: vec![],
            //     //     methods_declarations: vec![],
            //     // }
            //     // FFIObjectPresentation::StaticVTable {
            //     //     name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
            //     //     fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
            //     //     methods_implementations: vtable_methods_implentations,
            //     //     methods_declarations: vtable_methods_declarations,
            //     // }
            //     FFIObjectPresentation::StaticVTable {
            //         name: Name::TraitImplVtable(item_name.clone(), trait_ident.clone()),
            //         fq_trait_vtable: if is_defined_in_same_scope { quote!(#trait_vtable_ident) } else { quote!(#trait_scope::#trait_vtable_ident) },
            //         // methods_implementations: vtable_methods_implentations,
            //         // methods_declarations: vtable_methods_declarations,
            //         methods_compositions,
            //     }
            // }
        }
    }
}


impl FnSignatureComposition {
    pub fn from_signature(sig: &Signature, self_ty: Option<Type>, scope: PathHolder, context: &ScopeContext) -> Self {
        let Signature { output, ident, inputs, generics, .. } = sig;
        // TODO: make a path
        let return_type = handle_fn_return_type(output, context);
        let ident = Some(ident.clone());
        let arguments = handle_fn_args(inputs, &self_ty, context);
        let is_async = sig.asyncness.is_some();

        FnSignatureComposition { is_async, ident, scope, return_type, arguments, generics: Some(generics.clone()), self_ty }
    }

    pub fn from_bare_fn(bare_fn: &TypeBareFn, ident: &Ident, scope: PathHolder, context: &ScopeContext) -> Self {
        let TypeBareFn { inputs, output, .. } = bare_fn;
        let arguments = handle_bare_fn_args(inputs, context);
        let return_type = handle_bare_fn_return_type(output, context);
        FnSignatureComposition {
            is_async: false,
            ident: Some(ident.clone()),
            scope,
            arguments,
            return_type,
            self_ty: None,
            generics: None
        }
    }
}

fn handle_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeComposition {
    match output {
        ReturnType::Default => FnReturnTypeComposition { presentation: ReturnType::Default, conversion: FieldTypePresentableContext::LineTermination },
        ReturnType::Type(_, field_type) => {
            let conversion = match &**field_type {
                Type::Path(type_path) =>
                    type_path.conversion_to(FieldTypePresentableContext::Obj),
                Type::Tuple(type_tuple) =>
                    type_tuple.conversion_to(FieldTypePresentableContext::Obj),
                _ => panic!("error: output conversion: {}", quote!(#field_type)),
            };
            FnReturnTypeComposition { presentation: ReturnType::Type(RArrow::default(), Box::new(context.ffi_full_dictionary_type_presenter(field_type))), conversion }
        },
    }
}
fn handle_arg_type(ty: &Type, pat: &Pat, context: &ScopeContext) -> TokenStream2 {
    match (ty, pat) {
        (Type::Path(type_path), Pat::Ident(PatIdent { ident, .. })) =>
            type_path.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident)))
                .present(context),
        (Type::Reference(type_reference), pat) => {
            let arg_type = handle_arg_type(&type_reference.elem, pat, context);
            if type_reference.mutability.is_some() {
                FieldTypePresentableContext::AsMutRef(arg_type)
            } else {
                FieldTypePresentableContext::AsRef(arg_type)
            }.present(context)
        },
        (Type::Array(type_array), Pat::Ident(PatIdent { ident, .. })) =>
            type_array.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident)))
                .present(context),
        (Type::Slice(type_slice), Pat::Ident(PatIdent { ident, .. })) =>
            type_slice.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident)))
                .present(context),
        (Type::Tuple(type_tuple), Pat::Ident(PatIdent { ident, .. })) =>
            type_tuple.conversion_from(FieldTypePresentableContext::Simple(quote!(#ident))).present(context),
        (Type::TraitObject(type_trait_object), Pat::Ident(PatIdent { ident: _, .. })) => {
            FieldTypePresentableContext::AsRef(type_trait_object.to_token_stream()).present(context)
        },
        // (Type::Ptr(TypePtr { star_token, const_token, mutability, elem }), Pat::Ident(PatIdent { ident, .. })) =>
        _ => panic!("error: Arg conversion not supported: {}", quote!(#ty)),
    }
}

fn handle_bare_fn_return_type(output: &ReturnType, context: &ScopeContext) -> FnReturnTypeComposition {
    FnReturnTypeComposition {
        presentation: if let ReturnType::Type(token, field_type) = output {
            ReturnType::Type(token.clone(), Box::new(context.ffi_full_dictionary_type_presenter(field_type)))
        } else {
            ReturnType::Default
        },
        conversion: FieldTypePresentableContext::Empty
    }
}

fn handle_fn_args(inputs: &Punctuated<FnArg, Comma>, self_ty: &Option<Type>, context: &ScopeContext) -> Vec<FnArgComposition> {
    // TODO: replace Fn arguments with crate::fermented::generics::#ident or #import



    inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(Receiver { mutability, reference, .. }) => {
                let reference = match reference {
                    Some((_, _lifetime)) => quote!(&),
                    None => quote!()
                };
                let (ffi_type, name_type_conversion) = match (mutability, self_ty) {
                    (Some(..), Some(ty)) => (
                        {
                            let full_ty = context.full_type_for(ty).to_mangled_ident_default();
                            parse_quote!(*mut #full_ty)
                        },
                        quote!(#reference ferment_interfaces::FFIConversion::ffi_from(obj))),
                    (None, Some(ty)) => ({
                                             let full_ty = context.full_type_for(ty).to_mangled_ident_default();
                                             parse_quote!(*const #full_ty)
                                         },
                        quote!(#reference ferment_interfaces::FFIConversion::ffi_from(obj))),
                    (Some(..), None) => (parse_quote!(*mut ()), quote!(#reference ferment_interfaces::FFIConversion::ffi_from_const(obj as *const _))),
                    (None, None) => (parse_quote!(*const ()), quote!(#reference ferment_interfaces::FFIConversion::ffi_from_const(obj as *const _))),
                };
                let name_type_original = OwnedItemPresentableContext::Named(FieldTypeConversion::Named(Name::Dictionary(DictionaryFieldName::Obj), ffi_type), false);
                FnArgComposition {
                    name: None,
                    name_type_original,
                    name_type_conversion
                }
            },
            FnArg::Typed(PatType { ty, pat, .. }) => {
                // TODO: handle mut/const with pat
                let full_type = context.ffi_full_dictionary_type_presenter(ty);
                let name_type_original = OwnedItemPresentableContext::Named(FieldTypeConversion::Named(Name::Pat(*pat.clone()), full_type), false);
                let name_type_conversion = handle_arg_type(ty, pat, context);
                FnArgComposition {
                    name: Some(pat.to_token_stream()),
                    name_type_original,
                    name_type_conversion
                }
            },
        })
        .collect()
}

fn handle_bare_fn_args(inputs: &Punctuated<BareFnArg, Comma>, context: &ScopeContext) -> Vec<FnArgComposition> {
    inputs
        .iter()
        .map(|BareFnArg { ty, name, .. }| {
            let name = name.clone().map(|(ident, _)| ident);
            let pres = context.ffi_full_dictionary_type_presenter(ty);
            FnArgComposition {
                name: name.clone().map(|g| g.to_token_stream()),
                name_type_original: OwnedItemPresentableContext::Named(FieldTypeConversion::Named(Name::Optional(name), pres), false),
                name_type_conversion: quote!()
            }
        })
        .collect::<Vec<_>>()
}
