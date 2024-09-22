use quote::{quote, TokenStreamExt, ToTokens};
use syn::__private::TokenStream2;
use ferment_macro::MethodCall;
use crate::presentation::{FFIConversionDestroyMethod, FFIConversionFromMethod, FFIConversionToMethod};

#[derive(Clone, Debug, MethodCall)]
#[namespace = "ferment"]
pub enum InterfacesMethod {
    FFIConversionFrom(FFIConversionFromMethod),
    FFIConversionTo(FFIConversionToMethod),
    FFIConversionDestroy(FFIConversionDestroyMethod),
    FFIVecConversion(TokenStream2),
    FFIMapConversion(TokenStream2),
    Boxed,
    BoxedVec,
    BoxedArr,
    BoxedSlice,
    UnboxAny,
    UnboxAnyOpt,
    UnboxString,
    UnboxAnyVec,
    UnboxAnyVecPtr,
    UnboxVecPtr,

    FromOptPrimitive,
    ToOptPrimitive,
    DestroyOptPrimitive,

    FromPrimitiveGroup,
    FromOptPrimitiveGroup,
    FromComplexGroup,
    FromOptComplexGroup,

    ToComplexGroup,
    ToOptComplexGroup,
    ToPrimitiveGroup,
    ToOptPrimitiveGroup,

    FoldToMap,
    FoldToVec,
    FoldToResult,
    ToResult,
}
impl ToTokens for InterfacesMethod {
    fn to_tokens(&self, dst: &mut TokenStream2) {
        match self {
            InterfacesMethod::FFIConversionFrom(method) => quote!(FFIConversionFrom::#method),
            InterfacesMethod::FFIConversionTo(method) => quote!(FFIConversionTo::#method),
            InterfacesMethod::FFIConversionDestroy(method) => quote!(FFIConversionDestroy::#method),
            InterfacesMethod::FFIVecConversion(method) => quote!(FFIVecConversion::#method),
            InterfacesMethod::FFIMapConversion(method) => quote!(FFIMapConversion::#method),
            InterfacesMethod::Boxed => quote!(boxed),
            InterfacesMethod::BoxedVec => quote!(boxed_vec),
            InterfacesMethod::BoxedArr => quote!(boxed_arr),
            InterfacesMethod::BoxedSlice => quote!(boxed_slice),
            InterfacesMethod::UnboxAny => quote!(unbox_any),
            InterfacesMethod::UnboxAnyOpt => quote!(unbox_any_opt),
            InterfacesMethod::UnboxString => quote!(unbox_string),
            InterfacesMethod::UnboxAnyVec => quote!(unbox_any_vec_ptr),
            InterfacesMethod::UnboxAnyVecPtr => quote!(unbox_any_vec_ptr),
            InterfacesMethod::UnboxVecPtr => quote!(unbox_vec_ptr),
            InterfacesMethod::FromPrimitiveGroup => quote!(from_primitive_group),
            InterfacesMethod::FromOptPrimitiveGroup => quote!(from_opt_primitive_group),
            InterfacesMethod::FromComplexGroup => quote!(from_complex_group),
            InterfacesMethod::FromOptComplexGroup => quote!(from_opt_complex_group),
            InterfacesMethod::ToComplexGroup => quote!(to_complex_group),
            InterfacesMethod::ToOptComplexGroup => quote!(to_opt_complex_group),
            InterfacesMethod::ToPrimitiveGroup => quote!(to_primitive_group),
            InterfacesMethod::ToOptPrimitiveGroup => quote!(to_opt_primitive_group),
            InterfacesMethod::FoldToMap => quote!(fold_to_map),
            InterfacesMethod::FoldToVec => quote!(fold_to_vec),
            InterfacesMethod::FoldToResult => quote!(fold_to_result),
            InterfacesMethod::ToResult => quote!(to_result),
            InterfacesMethod::FromOptPrimitive => quote!(from_opt_primitive),
            InterfacesMethod::ToOptPrimitive => quote!(to_opt_primitive),
            InterfacesMethod::DestroyOptPrimitive => quote!(destroy_opt_primitive),
        }.to_tokens(dst)
    }
}