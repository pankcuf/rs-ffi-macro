use quote::{format_ident, quote, ToTokens};
use syn::{ItemTrait, parse_quote, Path};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use crate::composer::{Composer, LinkedComposer, Depunctuated, ParentComposer};
use crate::composer::trait_composer::TraitComposer;
use crate::composition::{AttrsComposition, FnReturnTypeComposer, TraitDecompositionPart2};
use crate::context::{ScopeChain, ScopeContext};
use crate::ext::{FFIResolveExtended, Mangle, Resolve, ToPath, ToType};
use crate::holder::EMPTY;
use crate::naming::Name;
use crate::presentation::context::OwnedItemPresentableContext;
use crate::presentation::{BindingPresentation, Expansion, ScopeContextPresentable};
use crate::shared::{ParentLinker, SharedAccess};

pub struct AttrsComposer<Parent: SharedAccess> {
    pub parent: Option<Parent>,
    pub attrs: AttrsComposition,
}
impl<'a, Parent: SharedAccess> LinkedComposer<'a, Parent> for AttrsComposer<Parent> {}
impl<Parent: SharedAccess> AttrsComposer<Parent> {
    pub fn new(attrs: AttrsComposition) -> AttrsComposer<Parent> {
        Self { parent: None, attrs }
    }
}

impl<Parent: SharedAccess> ParentLinker<Parent> for AttrsComposer<Parent> {
    fn link(&mut self, parent: &Parent) {
        self.parent = Some(parent.clone_container());
    }
}

impl<'a, Parent: SharedAccess> Composer<'a> for AttrsComposer<Parent> {
    type Source = ParentComposer<ScopeContext>;
    type Result = Depunctuated<Expansion>;
    fn compose(&self, context: &Self::Source) -> Self::Result {
        Depunctuated::new()
        // TODO: currently disable trait expansion via attributes,
        // TODO: migrate onto composable RefinedTree version
        // let attrs_composition = &self.attrs;
        // let source = context.borrow();
        // source.trait_items_from_attributes(&attrs_composition.attrs)
        //     .iter_mut()
        //     .map(|(composition, trait_scope)|
        //         implement_trait_for_item((&composition.item, trait_scope), attrs_composition, context))
        //     .collect()
    }
}

// pub fn implement_trait_for_item(item_trait: (&ItemTrait, &ScopeChain), attrs_composition: &AttrsComposition, context: &ParentComposer<ScopeContext>) -> TraitVTablePresentation {
//     let (item_trait, trait_scope) = item_trait;
//     let source = context.borrow();
//     let AttrsComposition { ident: item_name, scope: item_scope, .. } = attrs_composition;
//     let self_ty = item_name.to_type();
//     let trait_ident = &item_trait.ident;
//     let item_full_ty = self_ty.resolve(&source);
//     let trait_full_ty = trait_ident.to_type().resolve(&source);
//     let trait_decomposition = TraitDecompositionPart2::from_item_trait(&item_trait, self_ty, &EMPTY, context);
//     let methods_compositions: Vec<TraitVTableMethodComposition> = trait_decomposition.methods.into_iter()
//         .map(|signature_decomposition| {
//             let composer = signature_decomposition.borrow();
//             // TraitComposer::from_item_trait()
//             // composer.expand()
//             let FnReturnTypeComposition { presentation: output_expression, conversion: output_conversions } = signature_decomposition.return_type;
//             let fn_name = signature_decomposition.ident.unwrap();
//             let ffi_fn_name = format_ident!("{}_{}", item_name, fn_name);
//             let arguments_presentation = Wrapped::<_, Paren>::new(signature_decomposition
//                 .arguments
//                 .iter()
//                 .map(|arg| arg.name_type_original.clone())
//                 .collect::<Punctuated<_, Comma>>()
//                 .present(&source));
//             let argument_conversions =
//                 Wrapped::<_, Paren>::new(signature_decomposition
//                     .arguments
//                     .iter()
//                     .map(|arg|
//                         OwnedItemPresentableContext::FieldType(arg.name_type_conversion.clone()))
//                     .collect::<Punctuated<_, Comma>>()
//                     .present(&source));
//             let output_conversions = output_conversions.present(&source);
//             TraitVTableMethodComposition {
//                 name_and_args: quote!(unsafe extern "C" fn #ffi_fn_name #arguments_presentation),
//                 fn_name,
//                 ffi_fn_name,
//                 item_type: item_full_ty.clone(),
//                 trait_type: trait_full_ty.clone(),
//                 output_expression,
//                 output_conversions,
//                 argument_conversions
//             }
//         }).collect();
//     let trait_vtable_ident = Name::Vtable(trait_ident.clone());
//     let trait_object_ident = Name::TraitObj(trait_ident.clone());
//     let trait_self_scope = trait_scope.parent_path_holder();
//     let is_defined_in_same_scope = item_scope.has_same_parent(&trait_scope);
//
//     let full_trait_type: Path = if is_defined_in_same_scope { parse_quote!(#trait_object_ident) } else { parse_quote!(#trait_self_scope::#trait_object_ident) };
//     let fq_trait_ty: Path = if is_defined_in_same_scope { trait_vtable_ident.to_path() } else { parse_quote!(#trait_self_scope::#trait_ident) };
//     let mut fq_trait_vtable = fq_trait_ty.ffi_external_path_converted(&source).unwrap();
//     fq_trait_vtable.segments.last_mut().unwrap().ident = format_ident!("{}_VTable", fq_trait_ty.mangle_ident_default());
//     let vtable_name = Name::TraitImplVtable(item_name.clone(), trait_ident.clone());
//     let ffi_full_trait_ty = full_trait_type.ffi_external_path_converted(&source).unwrap();
//
//     TraitVTablePresentation::Full {
//         vtable: FFIObjectPresentation::StaticVTable {
//             name: vtable_name.clone(),
//             fq_trait_vtable: fq_trait_vtable.to_token_stream(),
//             methods_compositions,
//         },
//         export: BindingPresentation::ObjAsTrait {
//             name: Name::TraitFn(item_full_ty.clone(), trait_full_ty.clone()),
//             item_type: item_full_ty.to_token_stream(),
//             trait_type: ffi_full_trait_ty.to_token_stream(),
//             vtable_name,
//         },
//         destructor: BindingPresentation::ObjAsTraitDestructor {
//             name: Name::TraitDestructor(item_full_ty.clone(), trait_full_ty.clone()),
//             item_type: item_full_ty.to_token_stream(),
//             trait_type: ffi_full_trait_ty.to_token_stream(),
//         }
//     }
// }
// pub fn implement_trait_for_item(item_trait: (&ItemTrait, &ScopeChain), attrs_composition: &AttrsComposition, context: &ParentComposer<ScopeContext>) -> Expansion {
//     let (item_trait, trait_scope) = item_trait;
//     let source = context.borrow();
//     let AttrsComposition { ident: item_name, scope: item_scope, .. } = attrs_composition;
//     let self_ty = item_name.to_type();
//     let trait_ident = &item_trait.ident;
//     let item_full_ty = self_ty.resolve(&source);
//     let trait_full_ty = trait_ident.to_type().resolve(&source);
//     let composer = TraitComposer::from_item_trait(&item_trait, self_ty, context.sc, context);
//     let trait_decomposition = TraitDecompositionPart2::from_item_trait(&item_trait, self_ty, &EMPTY, context);
//     let (methods_declarations, methods_implementations) = trait_decomposition.method_composers.into_iter()
//         .map(|signature_decomposition| {
//             composer.compose_static()
//             let composer = signature_decomposition.borrow();
//             let dec: FnReturnTypeComposer = signature_decomposition.return_type;
//             // dec.compose()
//             // TraitComposer::from_item_trait()
//             // composer.expand()
//             let FnReturnTypeComposer { presentation: output, conversion: output_conversions } = dec;
//             let fn_name = signature_decomposition.ident.unwrap();
//             let args = Punctuated::<OwnedItemPresentableContext, Comma>::from_iter(signature_decomposition
//                 .arguments
//                 .iter()
//                 .map(|arg| arg.name_type_original.clone()))
//                 .present(&source);
//             let argument_conversions =
//                 Punctuated::<OwnedItemPresentableContext, Comma>::from_iter(signature_decomposition
//                     .arguments
//                     .iter()
//                     .map(|arg| OwnedItemPresentableContext::FieldType(arg.name_type_conversion.clone())))
//                     .present(&source);
//             let output_conversions = output_conversions.present(&source);
//             let name = Name::TraitImplVtable(item_name.clone(), fn_name);
//             // let name = format_ident!("{}_{}", item_name, fn_name);
//             let body = quote! {
//                 let obj = <#item_full_ty as #trait_full_ty>::#fn_name(#argument_conversions)
//                 #output_conversions
//             };
//             (
//                 BindingPresentation::StaticVTableInnerFnDeclaration { name: name.clone(), fn_name },
//                 BindingPresentation::StaticVTableInnerFn { name, args, output, body }
//             )
//         }).unzip();
//     let trait_vtable_ident = Name::Vtable(trait_ident.clone());
//     let trait_object_ident = Name::TraitObj(trait_ident.clone());
//     let trait_self_scope = trait_scope.parent_path_holder();
//     let is_defined_in_same_scope = item_scope.has_same_parent(&trait_scope);
//
//     let full_trait_type: Path = if is_defined_in_same_scope { parse_quote!(#trait_object_ident) } else { parse_quote!(#trait_self_scope::#trait_object_ident) };
//     let fq_trait_ty: Path = if is_defined_in_same_scope { trait_vtable_ident.to_path() } else { parse_quote!(#trait_self_scope::#trait_ident) };
//     let mut fq_trait_vtable = fq_trait_ty.ffi_external_path_converted(&source).unwrap();
//     fq_trait_vtable.segments.last_mut().unwrap().ident = format_ident!("{}_VTable", fq_trait_ty.mangle_ident_default());
//     let vtable_name = Name::TraitImplVtable(item_name.clone(), trait_ident.clone());
//     let ffi_full_trait_ty = full_trait_type.ffi_external_path_converted(&source).unwrap();
//
//     Expansion::TraitVTable {
//         vtable: BindingPresentation::StaticVTable {
//             name: vtable_name.clone(),
//             fq_trait_vtable: fq_trait_vtable.to_token_stream(),
//             methods_declarations,
//             methods_implementations,
//         },
//         export: BindingPresentation::ObjAsTrait {
//             name: Name::TraitFn(item_full_ty.clone(), trait_full_ty.clone()),
//             item_type: item_full_ty.to_token_stream(),
//             trait_type: ffi_full_trait_ty.to_token_stream(),
//             vtable_name,
//         },
//         destructor: BindingPresentation::ObjAsTraitDestructor {
//             name: Name::TraitDestructor(item_full_ty.clone(), trait_full_ty.clone()),
//             item_type: item_full_ty.to_token_stream(),
//             trait_type: ffi_full_trait_ty.to_token_stream(),
//         }
//     }
// }
