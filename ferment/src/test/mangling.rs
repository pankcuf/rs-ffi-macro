use quote::{format_ident, quote};
use syn::Ident;
use crate::path_conversion::PathConversion;

#[cfg(test)]
fn ident_from_str(s: &str) -> Ident {
    format_ident!("{}", s)
}

#[test]
fn mangle_generic_ident_test() {
    // Vec<Simple>
    assert_eq!(
        PathConversion::from("Vec<u8>").into_mangled_generic_ident(),
        ident_from_str("Vec_u8")
    );
    assert_eq!(
        PathConversion::from("Vec<u32>").into_mangled_generic_ident(),
        ident_from_str("Vec_u32")
    );
    assert_eq!(
        PathConversion::from("Vec<bool>").into_mangled_generic_ident(),
        ident_from_str("Vec_bool")
    );
    // Vec<Complex>
    assert_eq!(
        PathConversion::from("Vec<module::HashID>").into_mangled_generic_ident(),
        ident_from_str("Vec_module_HashID")
    );
    // Vec<Vec<Simple>
    assert_eq!(
        PathConversion::from("Vec<Vec<u8>>").into_mangled_generic_ident(),
        ident_from_str("Vec_Vec_u8")
    );
    // Vec<Vec<Complex>
    assert_eq!(
        PathConversion::from("Vec<Vec<module::HashID>>").into_mangled_generic_ident(),
        ident_from_str("Vec_Vec_module_HashID")
    );
    // Vec<Vec<Vec<Simple>>
    assert_eq!(
        PathConversion::from("Vec<Vec<Vec<u8>>>").into_mangled_generic_ident(),
        ident_from_str("Vec_Vec_Vec_u8")
    );
    // Vec<Vec<Vec<Complex>>
    assert_eq!(
        PathConversion::from("Vec<Vec<Vec<module::HashID>>>").into_mangled_generic_ident(),
        ident_from_str("Vec_Vec_Vec_module_HashID")
    );
    // Vec<Map<Simple, Simple>>
    assert_eq!(
        PathConversion::from("Vec<BTreeMap<u32, u32>>").into_mangled_generic_ident(),
        ident_from_str("Vec_Map_keys_u32_values_u32")
    );
    // Vec<Map<Complex, Complex>>
    assert_eq!(
        PathConversion::from("Vec<BTreeMap<module::HashID, module::KeyID>>")
            .into_mangled_generic_ident(),
        ident_from_str("Vec_Map_keys_module_HashID_values_module_KeyID")
    );

    // Map<Simple, Simple>
    assert_eq!(
        PathConversion::from("BTreeMap<u32, u32>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_u32_values_u32")
    );
    // Map<Simple, Complex>
    assert_eq!(
        PathConversion::from("BTreeMap<u32, module::HashID>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_u32_values_module_HashID")
    );
    // Map<Complex, Simple>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, u32>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_u32")
    );
    // Map<Complex, Complex>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, module::HashID>")
            .into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_module_HashID")
    );
    // Map<Complex, Vec<Simple>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, Vec<u32>>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Vec_u32")
    );
    // Map<Complex, Vec<Complex>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, Vec<module::KeyID>>")
            .into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Vec_module_KeyID")
    );
    // Map<Complex, Map<Complex, Complex>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>")
            .into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID")
    );
    // Map<Complex, Map<Complex, Vec<Simple>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<u32>>>")
            .into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_u32")
    );
    // Map<Complex, Map<Complex, Vec<Complex>>>
    assert_eq!(
        PathConversion::from(
            "BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>"
        )
            .into_mangled_generic_ident(),
        ident_from_str(
            "Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"
        )
    );
    // Map<Complex, Map<Complex, Map<Complex, Complex>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID"));
    // Map<Complex, Map<Complex, Map<Complex, Vec<Complex>>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>>").into_mangled_generic_ident(),
        ident_from_str("Map_keys_module_HashID_values_Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID"));
}

#[test]
fn mangle_generic_arguments_types_test() {
    // Vec<Simple>
    assert_eq!(
        PathConversion::from("Vec<u8>").mangled_generic_arguments_types_strings(),
        vec!["u8"]
    );

    assert_eq!(
        PathConversion::from("Vec<u32>").mangled_generic_arguments_types_strings(),
        vec!["u32"]
    );

    assert_eq!(
        PathConversion::from("Vec<bool>").mangled_generic_arguments_types_strings(),
        vec!["bool"]
    );
    // Vec<Complex>
    assert_eq!(
        PathConversion::from("Vec<module::HashID>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut module::HashID_FFI).to_string()]
    );
    // Vec<Vec<Simple>
    assert_eq!(
        PathConversion::from("Vec<Vec<u8>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Vec_u8_FFI).to_string()]
    );
    // Vec<Vec<Complex>
    assert_eq!(
        PathConversion::from("Vec<Vec<module::HashID>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Vec_module_HashID_FFI).to_string()]
    );
    // Vec<Vec<Vec<Simple>>
    assert_eq!(
        PathConversion::from("Vec<Vec<Vec<u8>>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Vec_Vec_u8_FFI).to_string()]
    );
    // Vec<Vec<Vec<Complex>>
    assert_eq!(
        PathConversion::from("Vec<Vec<Vec<module::HashID>>>")
            .mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Vec_Vec_module_HashID_FFI).to_string()]
    );
    // Vec<Map<Simple, Simple>>
    assert_eq!(
        PathConversion::from("Vec<BTreeMap<u32, u32>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Map_keys_u32_values_u32_FFI).to_string()]
    );
    // Vec<Map<Complex, Complex>>
    assert_eq!(
        PathConversion::from("Vec<BTreeMap<module::HashID, module::KeyID>>")
            .mangled_generic_arguments_types_strings(),
        vec![quote!(*mut Map_keys_module_HashID_values_module_KeyID_FFI).to_string()]
    );

    // Map<Simple, Simple>
    assert_eq!(
        PathConversion::from("BTreeMap<u32, u32>").mangled_generic_arguments_types_strings(),
        vec![quote!(u32).to_string(), quote!(u32).to_string()]
    );
    // Map<Simple, Complex>
    assert_eq!(
        PathConversion::from("BTreeMap<u32, module::HashID>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(u32).to_string(),
            quote!(*mut module::HashID_FFI).to_string()
        ]
    );
    // Map<Complex, Simple>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, u32>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(u32).to_string()
        ]
    );
    // Map<Complex, Complex>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, module::HashID>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut module::HashID_FFI).to_string()
        ]
    );
    // Map<Complex, Vec<Simple>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, Vec<u32>>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut Vec_u32_FFI).to_string()
        ]
    );
    // Map<Complex, Vec<Complex>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, Vec<module::KeyID>>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut Vec_module_KeyID_FFI).to_string()
        ]
    );
    // Map<Complex, Map<Complex, Complex>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut Map_keys_module_HashID_values_module_KeyID_FFI).to_string()
        ]
    );
    // Map<Complex, Map<Complex, Vec<Simple>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<u32>>>")
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut Map_keys_module_HashID_values_Vec_u32_FFI).to_string()
        ]
    );
    // Map<Complex, Map<Complex, Vec<Complex>>>
    assert_eq!(
        PathConversion::from(
            "BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>"
        )
            .mangled_generic_arguments_types_strings(),
        vec![
            quote!(*mut module::HashID_FFI).to_string(),
            quote!(*mut Map_keys_module_HashID_values_Vec_module_KeyID_FFI).to_string()
        ]
    );
    // Map<Complex, Map<Complex, Map<Complex, Complex>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, module::KeyID>>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut module::HashID_FFI).to_string(), quote!(*mut Map_keys_module_HashID_values_Map_keys_module_HashID_values_module_KeyID_FFI).to_string()]);
    // Map<Complex, Map<Complex, Map<Complex, Vec<Complex>>>>
    assert_eq!(
        PathConversion::from("BTreeMap<module::HashID, BTreeMap<module::HashID, BTreeMap<module::HashID, Vec<module::KeyID>>>>").mangled_generic_arguments_types_strings(),
        vec![quote!(*mut module::HashID_FFI).to_string(), quote!(*mut Map_keys_module_HashID_values_Map_keys_module_HashID_values_Vec_module_KeyID_FFI).to_string()]);
}
