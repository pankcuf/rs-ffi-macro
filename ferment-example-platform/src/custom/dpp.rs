#[allow(non_camel_case_types)]
#[ferment_macro::register(dpp::validation::JsonSchemaValidator)]
// #[derive(Clone)]
pub struct dpp_validation_JsonSchemaValidator {
    validator: Box<dpp::validation::JsonSchemaValidator>,
}
impl ferment_interfaces::FFIConversion<dpp::validation::JsonSchemaValidator> for dpp_validation_JsonSchemaValidator {
    unsafe fn ffi_from_const(ffi: *const Self) -> dpp::validation::JsonSchemaValidator {
        *(*ffi).validator
    }
    unsafe fn ffi_to_const(obj: dpp::validation::JsonSchemaValidator) -> *const Self {
        ferment_interfaces::boxed(dpp_validation_JsonSchemaValidator { validator: Box::new(obj) })
    }
}

#[allow(non_camel_case_types)]
#[ferment_macro::register(dpp::identity::core_script::CoreScript)]
// #[derive(Clone)]
pub struct CoreScriptFFI {
    pub raw: Box<dpp::identity::core_script::CoreScript>,
}
impl ferment_interfaces::FFIConversion<dpp::identity::core_script::CoreScript> for CoreScriptFFI {
    unsafe fn ffi_from_const(ffi: *const Self) -> dpp::identity::core_script::CoreScript {
        let ffi = &*ffi;
        let raw = &*ffi.raw;
        raw.clone()
    }
    unsafe fn ffi_to_const(obj: dpp::identity::core_script::CoreScript) -> *const Self {
        ferment_interfaces::boxed(Self { raw: Box::new(obj) })
    }
}

// impl Drop for CoreScriptFFI {
//     fn drop(&mut self) {
//         unsafe {
//             ferment_interfaces::unbox_any(self.raw);
//         }
//     }
// }

#[allow(non_camel_case_types)]
#[ferment_macro::register(dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator)]
// #[derive(Clone)]
pub struct StatelessJsonSchemaLazyValidatorFFI {
    pub raw: Box<dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator>,
}
impl ferment_interfaces::FFIConversion<dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator> for StatelessJsonSchemaLazyValidatorFFI {
    unsafe fn ffi_from_const(ffi: *const Self) -> dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator {
        *(*ffi).raw
    }
    unsafe fn ffi_to_const(obj: dpp::data_contract::document_type::v0::validator::StatelessJsonSchemaLazyValidator) -> *const Self {
        ferment_interfaces::boxed(Self { raw: Box::new(obj) })
    }
}

// impl Drop for StatelessJsonSchemaLazyValidatorFFI {
//     fn drop(&mut self) {
//         unsafe {
//             ferment_interfaces::unbox_any(self.raw);
//         }
//     }
// }
//
//
