use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use crate::interface::package_boxed_expression;

pub enum ToConversionPresentation {
    Enum(Vec<TokenStream2>),
    Struct(TokenStream2),
    Vec,
    Map(TokenStream2, TokenStream2),
    Result(TokenStream2, TokenStream2)
}

impl ToTokens for ToConversionPresentation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ToConversionPresentation::Enum(conversions) => {
                package_boxed_expression(quote!(match obj { #(#conversions,)* }))
            },
            ToConversionPresentation::Struct(conversion) => {
                quote! {
                    #conversion
                }
            },
            ToConversionPresentation::Vec =>
                quote!(ferment_interfaces::FFIVecConversion::encode(obj)),
            ToConversionPresentation::Map(to_key_conversion, to_value_conversion) =>
                quote!(ferment_interfaces::boxed(Self { count: obj.len(), keys: #to_key_conversion, values: #to_value_conversion  })),
            ToConversionPresentation::Result(to_ok_conversion, to_error_conversion) => quote! {
                let (ok, error) = match obj {
                    Ok(o) => (#to_ok_conversion, std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), #to_error_conversion)
                };
                ferment_interfaces::boxed(Self { ok, error })
            }
        }.to_tokens(tokens)
    }
}