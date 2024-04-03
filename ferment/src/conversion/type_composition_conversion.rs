use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use syn::{parse_quote, Path, Type};
use quote::ToTokens;
use proc_macro2::TokenStream as TokenStream2;
use syn::punctuated::Punctuated;
use syn::token::Comma;
pub use crate::composition::{GenericBoundComposition, TypeComposition, TraitDecompositionPart1};
use crate::composition::NestedArgument;
use crate::conversion::ObjectConversion;
use crate::ext::Pop;

#[derive(Clone)]
pub enum TypeCompositionConversion {
    Trait(TypeComposition, TraitDecompositionPart1, Vec<Path>),
    TraitType(TypeComposition),
    // TraitAssociatedType(TypeComposition),
    Object(TypeComposition),
    Primitive(TypeComposition),
    Bounds(GenericBoundComposition),
    SmartPointer(TypeComposition),
    Fn(TypeComposition),
    Tuple(TypeComposition),

    Unknown(TypeComposition),
    Imported(TypeComposition, Path),
}

impl ToTokens for TypeCompositionConversion {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.to_ty().to_tokens(tokens)
        // match self {
        //     TypeCompositionConversion::Imported(ty, path) => {
        //         let mut path = path.clone();
        //         path.segments.pop();
        //         path.to_tokens(tokens);
        //         ty.ty.to_tokens(tokens);
        //         println!("TypeCompositionConversion::Imported::ToTokens: {}", tokens)
        //     },
        //     _ => self.to_ty().to_tokens(tokens)
        // }
    }
}

impl TypeCompositionConversion {

    pub fn is_unknown(&self) -> bool {
        match self {
            TypeCompositionConversion::Unknown(..) => true,
            _ => false
        }
    }
    pub fn is_imported(&self) -> bool {
        match self {
            TypeCompositionConversion::Imported(..) => true,
            _ => false
        }
    }
    pub fn is_refined(&self) -> bool {
        match self {
            TypeCompositionConversion::Imported(..) |
            TypeCompositionConversion::Unknown(..) => false,
            other => {
                !other.nested_arguments()
                    .iter()
                    .find(|arg|
                        match arg {
                            NestedArgument::Object(obj) => {
                                match obj {
                                    ObjectConversion::Type(ty) => !ty.is_refined(),
                                    ObjectConversion::Item(_, _) => false,
                                    ObjectConversion::Empty => false
                                }
                            }
                        }).is_some()
            },
        }
    }
    pub fn nested_arguments(&self) -> &Punctuated<NestedArgument, Comma> {
        &self.ty_composition().nested_arguments
    }
    pub fn replace_composition_type(&mut self, with_ty: Type) {
        match self {
            TypeCompositionConversion::Trait(ty, ..) |
            TypeCompositionConversion::TraitType(ty) |
            // TypeCompositionConversion::TraitAssociatedType(ty) |
            TypeCompositionConversion::Object(ty, ..) |
            TypeCompositionConversion::Primitive(ty) |
            TypeCompositionConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeCompositionConversion::SmartPointer(ty, ..) |
            TypeCompositionConversion::Unknown(ty, ..) |
            TypeCompositionConversion::Tuple(ty) |
            TypeCompositionConversion::Imported(ty, ..) |
            TypeCompositionConversion::Fn(ty, ..) => ty.ty = with_ty,
        }

    }
    pub fn ty_composition(&self) -> &TypeComposition {
        match self {
            TypeCompositionConversion::Trait(ty, ..) |
            TypeCompositionConversion::TraitType(ty) |
            // TypeCompositionConversion::TraitAssociatedType(ty) |
            TypeCompositionConversion::Object(ty, ..) |
            TypeCompositionConversion::Primitive(ty) |
            TypeCompositionConversion::Bounds(GenericBoundComposition { type_composition: ty, .. }) |
            TypeCompositionConversion::SmartPointer(ty, ..) |
            TypeCompositionConversion::Unknown(ty, ..) |
            TypeCompositionConversion::Tuple(ty) |
            TypeCompositionConversion::Imported(ty, ..) |
            TypeCompositionConversion::Fn(ty, ..) => ty,
        }
    }
    pub fn ty(&self) -> &Type {
        &self.ty_composition().ty
    }
    pub fn to_ty(&self) -> Type {
        match self {
            TypeCompositionConversion::Imported(ty, import_path) => {
                // println!("to_ty: {} import_path: {}", ty, import_path.to_token_stream());
                // let ty = match &ty.ty {
                //     Type::Path(type_path) => {
                //         let path = import_path.popped();
                //         parse_quote!(#path::#type_path)
                //     },
                //     Type::Tuple(type_tuple) => {
                //         type_tuple.elems
                //         type_tuple.elems.iter().map(|ty| )
                //         // ty.nested_arguments.iter().map(|arg| match arg {
                //         //     NestedArgument::Object(obj) => obj.to_ty().unwrap()
                //         // })
                //     },
                //     ty => ty.clone(),
                // };

                // let nested = &ty.nested_arguments;
                let ty = &ty.ty;
                let path = import_path.popped();
                parse_quote!(#path::#ty)
            },
            // TypeCompositionConversion::Tuple(ty) => {
            //     println!("to_ty tuple: {}", ty);
            //     let nested = ty.nested_arguments.iter().filter_map(|d| match d { NestedArgument::Object(obj) => obj.to_ty() }).collect::<Punctuated<Type, Comma>>();
            //     parse_quote!((#nested))
            // }
            _ => self.ty_composition().ty.clone()
        }
    }
}

impl Debug for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeCompositionConversion::Trait(ty, _decomposition, _super_bounds) =>
                f.write_str(format!("Trait({})", ty).as_str()),
            TypeCompositionConversion::Object(ty) =>
                f.write_str(format!("Object({})", ty).as_str()),
            TypeCompositionConversion::Unknown(ty) =>
                f.write_str(format!("Unknown({})", ty).as_str()),
            TypeCompositionConversion::Primitive(ty) =>
                f.write_str(format!("Primitive({})", ty).as_str()),
            TypeCompositionConversion::TraitType(ty) =>
                f.write_str(format!("TraitType({})", ty).as_str()),
            TypeCompositionConversion::Bounds(gbc) =>
                f.write_str(format!("Bounds({})", gbc).as_str()),
            TypeCompositionConversion::SmartPointer(ty) =>
                f.write_str(format!("SmartPointer({})", ty).as_str()),
            // TypeCompositionConversion::TraitAssociatedType(ty) =>
            //     f.write_str(format!("TraitAssociatedType({})", ty).as_str()),
            TypeCompositionConversion::Fn(ty) =>
                f.write_str(format!("Fn({})", ty).as_str()),
            TypeCompositionConversion::Tuple(ty) =>
                f.write_str(format!("Tuple({})", ty).as_str()),
            TypeCompositionConversion::Imported(ty, import_path) =>
                f.write_str(format!("Imported({}, {})", ty, import_path.to_token_stream()).as_str()),

        }
    }
}

impl Display for TypeCompositionConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq for TypeCompositionConversion {
    fn eq(&self, other: &Self) -> bool {
        let self_tokens = [self.to_ty().to_token_stream()];
        let other_tokens = [other.to_ty().to_token_stream()];
        self_tokens.iter()
            .map(|t| t.to_string())
            .zip(other_tokens.iter().map(|t| t.to_string()))
            .all(|(a, b)| a == b)
    }
}

impl Eq for TypeCompositionConversion {}

impl Hash for TypeCompositionConversion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_ty().to_token_stream().to_string().hash(state);
    }
}

