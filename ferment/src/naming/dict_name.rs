use std::fmt::Formatter;
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

#[derive(Clone, Debug)]
pub enum DictionaryName {
    Ok,
    Error,
    Keys,
    Values,
    Count,
    Obj,
    Object,
    Value,
    Vtable,
    Self_,
    I,
    O,
    Package,
    Interface,
    Ffi,
    FfiRef,
}

impl std::fmt::Display for DictionaryName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
impl ToTokens for DictionaryName {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            DictionaryName::Ok => quote!(ok),
            DictionaryName::Error => quote!(error),
            DictionaryName::Keys => quote!(keys),
            DictionaryName::Values => quote!(values),
            DictionaryName::Count => quote!(count),
            DictionaryName::Obj => quote!(obj),
            DictionaryName::Object => quote!(object),
            DictionaryName::Value => quote!(value),
            DictionaryName::Package => quote!(ferment_interfaces),
            DictionaryName::Interface => quote!(FFIConversion),
            DictionaryName::Self_ => quote!(self_),
            DictionaryName::I => quote!(i),
            DictionaryName::O => quote!(o),
            DictionaryName::Ffi => quote!(ffi),
            DictionaryName::FfiRef => quote!(ffi_ref),
            DictionaryName::Vtable => quote!(vtable),
        }
            .to_tokens(tokens)
    }
}