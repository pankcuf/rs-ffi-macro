#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod types {
    pub mod nested {
        #[doc = "FFI-representation of the [`crate::nested::FeatureVersion`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct FeatureVersion(u16);
        impl ferment_interfaces::FFIConversion<crate::nested::FeatureVersion> for FeatureVersion {
            unsafe fn ffi_from_const(ffi: *const FeatureVersion) -> crate::nested::FeatureVersion {
                let ffi_ref = &*ffi;
                ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: crate::nested::FeatureVersion) -> *const FeatureVersion {
                ferment_interfaces::boxed(FeatureVersion(obj))
            }
            unsafe fn destroy(ffi: *mut FeatureVersion) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for FeatureVersion {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn FeatureVersion_ctor(o_0: u16) -> *mut FeatureVersion {
            ferment_interfaces::boxed(FeatureVersion(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersion_destroy(ffi: *mut FeatureVersion) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersion_get_0(obj: *const FeatureVersion) -> u16 {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersion_set_0(obj: *mut FeatureVersion, value: u16) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::Identifier`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct Identifier(*mut crate::fermented::types::nested::IdentifierBytes32);
        impl ferment_interfaces::FFIConversion<crate::nested::Identifier> for Identifier {
            unsafe fn ffi_from_const(ffi: *const Identifier) -> crate::nested::Identifier {
                let ffi_ref = &*ffi;
                crate::nested::Identifier(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::Identifier) -> *const Identifier {
                ferment_interfaces::boxed(Identifier(ferment_interfaces::FFIConversion::ffi_to(
                    obj.0,
                )))
            }
            unsafe fn destroy(ffi: *mut Identifier) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for Identifier {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn Identifier_ctor(
            o_0: *mut crate::fermented::types::nested::IdentifierBytes32,
        ) -> *mut Identifier {
            ferment_interfaces::boxed(Identifier(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn Identifier_destroy(ffi: *mut Identifier) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn Identifier_get_0(
            obj: *const Identifier,
        ) -> *mut crate::fermented::types::nested::IdentifierBytes32 {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn Identifier_set_0(
            obj: *mut Identifier,
            value: *mut crate::fermented::types::nested::IdentifierBytes32,
        ) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::PlatformVersion`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct PlatformVersion {
            pub protocol_version: u32,
            pub identity: *mut crate::fermented::types::nested::FeatureVersionBounds,
            pub proofs: *mut crate::fermented::types::nested::FeatureVersionBounds,
        }
        impl ferment_interfaces::FFIConversion<crate::nested::PlatformVersion> for PlatformVersion {
            unsafe fn ffi_from_const(
                ffi: *const PlatformVersion,
            ) -> crate::nested::PlatformVersion {
                let ffi_ref = &*ffi;
                crate::nested::PlatformVersion {
                    protocol_version: ffi_ref.protocol_version,
                    identity: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.identity),
                    proofs: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.proofs),
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::PlatformVersion) -> *const PlatformVersion {
                ferment_interfaces::boxed(PlatformVersion {
                    protocol_version: obj.protocol_version,
                    identity: ferment_interfaces::FFIConversion::ffi_to(obj.identity),
                    proofs: ferment_interfaces::FFIConversion::ffi_to(obj.proofs),
                })
            }
            unsafe fn destroy(ffi: *mut PlatformVersion) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for PlatformVersion {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.identity);
                    ferment_interfaces::unbox_any(ffi_ref.proofs);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn PlatformVersion_ctor(
            protocol_version: u32,
            identity: *mut crate::fermented::types::nested::FeatureVersionBounds,
            proofs: *mut crate::fermented::types::nested::FeatureVersionBounds,
        ) -> *mut PlatformVersion {
            ferment_interfaces::boxed(PlatformVersion {
                protocol_version,
                identity,
                proofs,
            })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_destroy(ffi: *mut PlatformVersion) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_get_protocol_version(
            obj: *const PlatformVersion,
        ) -> u32 {
            (*obj).protocol_version
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_get_identity(
            obj: *const PlatformVersion,
        ) -> *mut crate::fermented::types::nested::FeatureVersionBounds {
            (*obj).identity
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_get_proofs(
            obj: *const PlatformVersion,
        ) -> *mut crate::fermented::types::nested::FeatureVersionBounds {
            (*obj).proofs
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_set_protocol_version(
            obj: *mut PlatformVersion,
            value: u32,
        ) {
            (*obj).protocol_version = value;
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_set_identity(
            obj: *mut PlatformVersion,
            value: *mut crate::fermented::types::nested::FeatureVersionBounds,
        ) {
            (*obj).identity = value;
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn PlatformVersion_set_proofs(
            obj: *mut PlatformVersion,
            value: *mut crate::fermented::types::nested::FeatureVersionBounds,
        ) {
            (*obj).proofs = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::DataContractNotPresentError`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct DataContractNotPresentError {
            pub data_contract_id: *mut crate::fermented::types::nested::Identifier,
        }
        impl ferment_interfaces::FFIConversion<crate::nested::DataContractNotPresentError>
            for DataContractNotPresentError
        {
            unsafe fn ffi_from_const(
                ffi: *const DataContractNotPresentError,
            ) -> crate::nested::DataContractNotPresentError {
                let ffi_ref = &*ffi;
                crate::nested::DataContractNotPresentError {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_from(
                        ffi_ref.data_contract_id,
                    ),
                }
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::DataContractNotPresentError,
            ) -> *const DataContractNotPresentError {
                ferment_interfaces::boxed(DataContractNotPresentError {
                    data_contract_id: ferment_interfaces::FFIConversion::ffi_to(
                        obj.data_contract_id,
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut DataContractNotPresentError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for DataContractNotPresentError {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.data_contract_id);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn DataContractNotPresentError_ctor(
            data_contract_id: *mut crate::fermented::types::nested::Identifier,
        ) -> *mut DataContractNotPresentError {
            ferment_interfaces::boxed(DataContractNotPresentError { data_contract_id })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn DataContractNotPresentError_destroy(
            ffi: *mut DataContractNotPresentError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn DataContractNotPresentError_get_data_contract_id(
            obj: *const DataContractNotPresentError,
        ) -> *mut crate::fermented::types::nested::Identifier {
            (*obj).data_contract_id
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn DataContractNotPresentError_set_data_contract_id(
            obj: *mut DataContractNotPresentError,
            value: *mut crate::fermented::types::nested::Identifier,
        ) {
            (*obj).data_contract_id = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::BinaryData`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct BinaryData(*mut crate::fermented::generics::Vec_u8);
        impl ferment_interfaces::FFIConversion<crate::nested::BinaryData> for BinaryData {
            unsafe fn ffi_from_const(ffi: *const BinaryData) -> crate::nested::BinaryData {
                let ffi_ref = &*ffi;
                crate::nested::BinaryData(ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0))
            }
            unsafe fn ffi_to_const(obj: crate::nested::BinaryData) -> *const BinaryData {
                ferment_interfaces::boxed(BinaryData(ferment_interfaces::FFIConversion::ffi_to(
                    obj.0,
                )))
            }
            unsafe fn destroy(ffi: *mut BinaryData) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for BinaryData {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn BinaryData_ctor(
            o_0: *mut crate::fermented::generics::Vec_u8,
        ) -> *mut BinaryData {
            ferment_interfaces::boxed(BinaryData(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn BinaryData_destroy(ffi: *mut BinaryData) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn BinaryData_get_0(
            obj: *const BinaryData,
        ) -> *mut crate::fermented::generics::Vec_u8 {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn BinaryData_set_0(
            obj: *mut BinaryData,
            value: *mut crate::fermented::generics::Vec_u8,
        ) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::FeatureVersionBounds`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct FeatureVersionBounds {
            pub min_version: *mut crate::fermented::types::nested::FeatureVersion,
            pub max_version: *mut crate::fermented::types::nested::FeatureVersion,
            pub default_current_version: *mut crate::fermented::types::nested::FeatureVersion,
        }
        impl ferment_interfaces::FFIConversion<crate::nested::FeatureVersionBounds>
            for FeatureVersionBounds
        {
            unsafe fn ffi_from_const(
                ffi: *const FeatureVersionBounds,
            ) -> crate::nested::FeatureVersionBounds {
                let ffi_ref = &*ffi;
                crate::nested::FeatureVersionBounds {
                    min_version: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.min_version),
                    max_version: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.max_version),
                    default_current_version: ferment_interfaces::FFIConversion::ffi_from(
                        ffi_ref.default_current_version,
                    ),
                }
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::FeatureVersionBounds,
            ) -> *const FeatureVersionBounds {
                ferment_interfaces::boxed(FeatureVersionBounds {
                    min_version: ferment_interfaces::FFIConversion::ffi_to(obj.min_version),
                    max_version: ferment_interfaces::FFIConversion::ffi_to(obj.max_version),
                    default_current_version: ferment_interfaces::FFIConversion::ffi_to(
                        obj.default_current_version,
                    ),
                })
            }
            unsafe fn destroy(ffi: *mut FeatureVersionBounds) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for FeatureVersionBounds {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.min_version);
                    ferment_interfaces::unbox_any(ffi_ref.max_version);
                    ferment_interfaces::unbox_any(ffi_ref.default_current_version);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn FeatureVersionBounds_ctor(
            min_version: *mut crate::fermented::types::nested::FeatureVersion,
            max_version: *mut crate::fermented::types::nested::FeatureVersion,
            default_current_version: *mut crate::fermented::types::nested::FeatureVersion,
        ) -> *mut FeatureVersionBounds {
            ferment_interfaces::boxed(FeatureVersionBounds {
                min_version,
                max_version,
                default_current_version,
            })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_destroy(ffi: *mut FeatureVersionBounds) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_get_min_version(
            obj: *const FeatureVersionBounds,
        ) -> *mut crate::fermented::types::nested::FeatureVersion {
            (*obj).min_version
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_get_max_version(
            obj: *const FeatureVersionBounds,
        ) -> *mut crate::fermented::types::nested::FeatureVersion {
            (*obj).max_version
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_get_default_current_version(
            obj: *const FeatureVersionBounds,
        ) -> *mut crate::fermented::types::nested::FeatureVersion {
            (*obj).default_current_version
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_set_min_version(
            obj: *mut FeatureVersionBounds,
            value: *mut crate::fermented::types::nested::FeatureVersion,
        ) {
            (*obj).min_version = value;
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_set_max_version(
            obj: *mut FeatureVersionBounds,
            value: *mut crate::fermented::types::nested::FeatureVersion,
        ) {
            (*obj).max_version = value;
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn FeatureVersionBounds_set_default_current_version(
            obj: *mut FeatureVersionBounds,
            value: *mut crate::fermented::types::nested::FeatureVersion,
        ) {
            (*obj).default_current_version = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::OptionalFeatureVersion`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct OptionalFeatureVersion(u16);
        impl ferment_interfaces::FFIConversion<crate::nested::OptionalFeatureVersion>
            for OptionalFeatureVersion
        {
            unsafe fn ffi_from_const(
                ffi: *const OptionalFeatureVersion,
            ) -> crate::nested::OptionalFeatureVersion {
                let ffi_ref = &*ffi;
                (ffi_ref.0 > 0).then_some(ffi_ref.0)
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::OptionalFeatureVersion,
            ) -> *const OptionalFeatureVersion {
                ferment_interfaces::boxed(OptionalFeatureVersion(obj.unwrap_or(0)))
            }
            unsafe fn destroy(ffi: *mut OptionalFeatureVersion) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for OptionalFeatureVersion {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn OptionalFeatureVersion_ctor(
            o_0: u16,
        ) -> *mut OptionalFeatureVersion {
            ferment_interfaces::boxed(OptionalFeatureVersion(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn OptionalFeatureVersion_destroy(ffi: *mut OptionalFeatureVersion) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn OptionalFeatureVersion_get_0(
            obj: *const OptionalFeatureVersion,
        ) -> u16 {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn OptionalFeatureVersion_set_0(
            obj: *mut OptionalFeatureVersion,
            value: u16,
        ) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::HashID`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct HashID(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<crate::nested::HashID> for HashID {
            unsafe fn ffi_from_const(ffi: *const HashID) -> crate::nested::HashID {
                let ffi_ref = &*ffi;
                *ffi_ref.0
            }
            unsafe fn ffi_to_const(obj: crate::nested::HashID) -> *const HashID {
                ferment_interfaces::boxed(HashID(ferment_interfaces::boxed(obj)))
            }
            unsafe fn destroy(ffi: *mut HashID) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for HashID {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn HashID_ctor(o_0: *mut [u8; 32]) -> *mut HashID {
            ferment_interfaces::boxed(HashID(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn HashID_destroy(ffi: *mut HashID) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn HashID_get_0(obj: *const HashID) -> *mut [u8; 32] {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn HashID_set_0(obj: *mut HashID, value: *mut [u8; 32]) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`crate::nested::IdentifierBytes32`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct IdentifierBytes32(*mut [u8; 32]);
        impl ferment_interfaces::FFIConversion<crate::nested::IdentifierBytes32> for IdentifierBytes32 {
            unsafe fn ffi_from_const(
                ffi: *const IdentifierBytes32,
            ) -> crate::nested::IdentifierBytes32 {
                let ffi_ref = &*ffi;
                crate::nested::IdentifierBytes32(*ffi_ref.0)
            }
            unsafe fn ffi_to_const(
                obj: crate::nested::IdentifierBytes32,
            ) -> *const IdentifierBytes32 {
                ferment_interfaces::boxed(IdentifierBytes32(ferment_interfaces::boxed(obj.0)))
            }
            unsafe fn destroy(ffi: *mut IdentifierBytes32) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for IdentifierBytes32 {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment_interfaces::unbox_any(ffi_ref.0);
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        #[inline(never)]
        pub unsafe extern "C" fn IdentifierBytes32_ctor(
            o_0: *mut [u8; 32],
        ) -> *mut IdentifierBytes32 {
            ferment_interfaces::boxed(IdentifierBytes32(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn IdentifierBytes32_destroy(ffi: *mut IdentifierBytes32) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn IdentifierBytes32_get_0(
            obj: *const IdentifierBytes32,
        ) -> *mut [u8; 32] {
            (*obj).0
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn IdentifierBytes32_set_0(
            obj: *mut IdentifierBytes32,
            value: *mut [u8; 32],
        ) {
            (*obj).0 = value;
        }
        #[doc = "FFI-representation of the [`ProtocolError`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub enum ProtocolError {
            IdentifierError(*mut std::os::raw::c_char),
            StringDecodeError(*mut std::os::raw::c_char),
            StringDecodeError2(*mut std::os::raw::c_char, u32),
            EmptyPublicKeyDataError,
            MaxEncodedBytesReachedError {
                max_size_kbytes: usize,
                size_hit: usize,
            },
            EncodingError(*mut std::os::raw::c_char),
            EncodingError2(*mut std::os::raw::c_char),
            DataContractNotPresentError(
                *mut crate::fermented::types::nested::DataContractNotPresentError,
            ),
            UnknownVersionMismatch {
                method: *mut std::os::raw::c_char,
                known_versions: *mut crate::fermented::generics::Vec_crate_nested_FeatureVersion,
                received: *mut crate::fermented::types::nested::FeatureVersion,
            },
        }
        impl ferment_interfaces::FFIConversion<crate::nested::ProtocolError> for ProtocolError {
            unsafe fn ffi_from_const(ffi: *const ProtocolError) -> crate::nested::ProtocolError {
                let ffi_ref = &*ffi;
                match ffi_ref {
                    ProtocolError::IdentifierError(o_0) => {
                        crate::nested::ProtocolError::IdentifierError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::StringDecodeError(o_0) => {
                        crate::nested::ProtocolError::StringDecodeError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::StringDecodeError2(o_0, o_1) => {
                        crate::nested::ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            *o_1,
                        )
                    }
                    ProtocolError::EmptyPublicKeyDataError => {
                        crate::nested::ProtocolError::EmptyPublicKeyDataError
                    }
                    ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => crate::nested::ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes: *max_size_kbytes,
                        size_hit: *size_hit,
                    },
                    ProtocolError::EncodingError(o_0) => {
                        crate::nested::ProtocolError::EncodingError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::EncodingError2(o_0) => {
                        crate::nested::ProtocolError::EncodingError2(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::DataContractNotPresentError(o_0) => {
                        crate::nested::ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        )
                    }
                    ProtocolError::UnknownVersionMismatch {
                        method,
                        known_versions,
                        received,
                    } => crate::nested::ProtocolError::UnknownVersionMismatch {
                        method: ferment_interfaces::FFIConversion::ffi_from(*method),
                        known_versions: ferment_interfaces::FFIConversion::ffi_from(
                            *known_versions,
                        ),
                        received: ferment_interfaces::FFIConversion::ffi_from(*received),
                    },
                }
            }
            unsafe fn ffi_to_const(obj: crate::nested::ProtocolError) -> *const ProtocolError {
                ferment_interfaces::boxed(match obj {
                    crate::nested::ProtocolError::IdentifierError(o_0) => {
                        ProtocolError::IdentifierError(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError(o_0) => {
                        ProtocolError::StringDecodeError(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::StringDecodeError2(o_0, o_1) => {
                        ProtocolError::StringDecodeError2(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                            o_1,
                        )
                    }
                    crate::nested::ProtocolError::EmptyPublicKeyDataError => {
                        ProtocolError::EmptyPublicKeyDataError
                    }
                    crate::nested::ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes,
                        size_hit,
                    } => ProtocolError::MaxEncodedBytesReachedError {
                        max_size_kbytes: max_size_kbytes,
                        size_hit: size_hit,
                    },
                    crate::nested::ProtocolError::EncodingError(o_0) => {
                        ProtocolError::EncodingError(ferment_interfaces::FFIConversion::ffi_to(o_0))
                    }
                    crate::nested::ProtocolError::EncodingError2(o_0) => {
                        ProtocolError::EncodingError2(ferment_interfaces::FFIConversion::ffi_to(
                            o_0,
                        ))
                    }
                    crate::nested::ProtocolError::DataContractNotPresentError(o_0) => {
                        ProtocolError::DataContractNotPresentError(
                            ferment_interfaces::FFIConversion::ffi_to(o_0),
                        )
                    }
                    crate::nested::ProtocolError::UnknownVersionMismatch {
                        method,
                        known_versions,
                        received,
                    } => ProtocolError::UnknownVersionMismatch {
                        method: ferment_interfaces::FFIConversion::ffi_to(method),
                        known_versions: ferment_interfaces::FFIConversion::ffi_to(known_versions),
                        received: ferment_interfaces::FFIConversion::ffi_to(received),
                    },
                })
            }
            unsafe fn destroy(ffi: *mut ProtocolError) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ProtocolError {
            fn drop(&mut self) {
                unsafe {
                    match self {
                        ProtocolError::IdentifierError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ;
                        }
                        ProtocolError::StringDecodeError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ;
                        }
                        ProtocolError::StringDecodeError2(o_0, o_1) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ;
                        }
                        ProtocolError::EmptyPublicKeyDataError => {}
                        ProtocolError::MaxEncodedBytesReachedError {
                            max_size_kbytes,
                            size_hit,
                        } => {}
                        ProtocolError::EncodingError(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* o_0) ;
                        }
                        ProtocolError::EncodingError2(o_0) => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < & str >> :: destroy (* o_0) ;
                        }
                        ProtocolError::DataContractNotPresentError(o_0) => {
                            ferment_interfaces::unbox_any(*o_0);
                        }
                        ProtocolError::UnknownVersionMismatch {
                            method,
                            known_versions,
                            received,
                        } => {
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (* method) ;
                            ferment_interfaces::unbox_any(*known_versions);
                            ferment_interfaces::unbox_any(*received);
                        }
                    }
                }
            }
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_IdentifierError_ctor(
            o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::IdentifierError(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_StringDecodeError_ctor(
            o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::StringDecodeError(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_StringDecodeError2_ctor(
            o_0: *mut std::os::raw::c_char,
            o_1: u32,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::StringDecodeError2(o_0, o_1))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EmptyPublicKeyDataError_ctor() -> *mut ProtocolError
        {
            ferment_interfaces::boxed(ProtocolError::EmptyPublicKeyDataError)
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_MaxEncodedBytesReachedError_ctor(
            max_size_kbytes: usize,
            size_hit: usize,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::MaxEncodedBytesReachedError {
                max_size_kbytes,
                size_hit,
            })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EncodingError_ctor(
            o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::EncodingError(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_EncodingError2_ctor(
            o_0: *mut std::os::raw::c_char,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::EncodingError2(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_DataContractNotPresentError_ctor(
            o_0: *mut crate::fermented::types::nested::DataContractNotPresentError,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::DataContractNotPresentError(o_0))
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_UnknownVersionMismatch_ctor(
            method: *mut std::os::raw::c_char,
            known_versions: *mut crate::fermented::generics::Vec_crate_nested_FeatureVersion,
            received: *mut crate::fermented::types::nested::FeatureVersion,
        ) -> *mut ProtocolError {
            ferment_interfaces::boxed(ProtocolError::UnknownVersionMismatch {
                method,
                known_versions,
                received,
            })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ProtocolError_destroy(ffi: *mut ProtocolError) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    #[doc = "FFI-representation of the [`crate::RootStruct`]"]
    #[repr(C)]
    #[derive(Clone)]
    pub struct RootStruct {
        pub name: *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<crate::RootStruct> for RootStruct {
        unsafe fn ffi_from_const(ffi: *const RootStruct) -> crate::RootStruct {
            let ffi_ref = &*ffi;
            crate::RootStruct {
                name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
            }
        }
        unsafe fn ffi_to_const(obj: crate::RootStruct) -> *const RootStruct {
            ferment_interfaces::boxed(RootStruct {
                name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
            })
        }
        unsafe fn destroy(ffi: *mut RootStruct) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for RootStruct {
        fn drop(&mut self) {
            unsafe {
                let ffi_ref = self;
                <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                    ffi_ref.name,
                );
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    #[inline(never)]
    pub unsafe extern "C" fn RootStruct_ctor(name: *mut std::os::raw::c_char) -> *mut RootStruct {
        ferment_interfaces::boxed(RootStruct { name })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn RootStruct_destroy(ffi: *mut RootStruct) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn RootStruct_get_name(
        obj: *const RootStruct,
    ) -> *mut std::os::raw::c_char {
        (*obj).name
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn RootStruct_set_name(
        obj: *mut RootStruct,
        value: *mut std::os::raw::c_char,
    ) {
        (*obj).name = value;
    }
    pub mod identity {
        pub mod identity {
            #[doc = "FFI-representation of the [`ContractBounds`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ContractBounds {
                SingleContract {
                    id: *mut crate::fermented::types::nested::Identifier,
                },
                SingleContractDocumentType {
                    id: *mut crate::fermented::types::nested::Identifier,
                    document_type_name: *mut std::os::raw::c_char,
                },
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::ContractBounds>
                for ContractBounds
            {
                unsafe fn ffi_from_const(
                    ffi: *const ContractBounds,
                ) -> crate::identity::identity::ContractBounds {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ContractBounds::SingleContract { id } => {
                            crate::identity::identity::ContractBounds::SingleContract {
                                id: ferment_interfaces::FFIConversion::ffi_from(*id),
                            }
                        }
                        ContractBounds::SingleContractDocumentType {
                            id,
                            document_type_name,
                        } => {
                            crate::identity::identity::ContractBounds::SingleContractDocumentType {
                                id: ferment_interfaces::FFIConversion::ffi_from(*id),
                                document_type_name: ferment_interfaces::FFIConversion::ffi_from(
                                    *document_type_name,
                                ),
                            }
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::ContractBounds,
                ) -> *const ContractBounds {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::ContractBounds::SingleContract { id } => {
                            ContractBounds::SingleContract {
                                id: ferment_interfaces::FFIConversion::ffi_to(id),
                            }
                        }
                        crate::identity::identity::ContractBounds::SingleContractDocumentType {
                            id,
                            document_type_name,
                        } => ContractBounds::SingleContractDocumentType {
                            id: ferment_interfaces::FFIConversion::ffi_to(id),
                            document_type_name: ferment_interfaces::FFIConversion::ffi_to(
                                document_type_name,
                            ),
                        },
                    })
                }
                unsafe fn destroy(ffi: *mut ContractBounds) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ContractBounds {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ContractBounds::SingleContract { id } => {
                                ferment_interfaces::unbox_any(*id);
                            }
                            ContractBounds::SingleContractDocumentType {
                                id,
                                document_type_name,
                            } => {
                                ferment_interfaces::unbox_any(*id);
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*document_type_name);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ContractBounds_SingleContract_ctor(
                id: *mut crate::fermented::types::nested::Identifier,
            ) -> *mut ContractBounds {
                ferment_interfaces::boxed(ContractBounds::SingleContract { id })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ContractBounds_SingleContractDocumentType_ctor(
                id: *mut crate::fermented::types::nested::Identifier,
                document_type_name: *mut std::os::raw::c_char,
            ) -> *mut ContractBounds {
                ferment_interfaces::boxed(ContractBounds::SingleContractDocumentType {
                    id,
                    document_type_name,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ContractBounds_destroy(ffi: *mut ContractBounds) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]            pub unsafe extern "C" fn create_basic_identity (id : * mut [u8 ; 32] , _platform_version : * mut crate :: fermented :: types :: nested :: PlatformVersion ,) -> * mut crate :: fermented :: generics :: Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError{
                let obj = crate::identity::identity::Identity::create_basic_identity(
                    *id,
                    &ferment_interfaces::FFIConversion::ffi_from(_platform_version),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[no_mangle]
            pub unsafe extern "C" fn create_basic_identity_v0(
                id: *mut [u8; 32],
            ) -> *mut crate::fermented::types::identity::identity::Identity {
                let obj = crate::identity::identity::Identity::create_basic_identity_v0(*id);
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[no_mangle]
            pub unsafe extern "C" fn get_balance(obj: *mut Identity) -> u64 {
                let obj = crate::identity::identity::Identity::get_balance(
                    &ferment_interfaces::FFIConversion::ffi_from(obj),
                );
                obj
            }
            #[doc = "FFI-representation of the [`crate::identity::identity::TimestampMillis`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct TimestampMillis(u64);
            impl ferment_interfaces::FFIConversion<crate::identity::identity::TimestampMillis>
                for TimestampMillis
            {
                unsafe fn ffi_from_const(
                    ffi: *const TimestampMillis,
                ) -> crate::identity::identity::TimestampMillis {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::TimestampMillis,
                ) -> *const TimestampMillis {
                    ferment_interfaces::boxed(TimestampMillis(obj))
                }
                unsafe fn destroy(ffi: *mut TimestampMillis) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for TimestampMillis {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn TimestampMillis_ctor(o_0: u64) -> *mut TimestampMillis {
                ferment_interfaces::boxed(TimestampMillis(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn TimestampMillis_destroy(ffi: *mut TimestampMillis) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn TimestampMillis_get_0(obj: *const TimestampMillis) -> u64 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn TimestampMillis_set_0(obj: *mut TimestampMillis, value: u64) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`crate::identity::identity::IdentityV0`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct IdentityV0 { pub id : * mut crate :: fermented :: types :: nested :: Identifier , pub public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey , pub balance : u64 , pub revision : * mut crate :: fermented :: types :: identity :: identity :: Revision , }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::IdentityV0> for IdentityV0 {
                unsafe fn ffi_from_const(
                    ffi: *const IdentityV0,
                ) -> crate::identity::identity::IdentityV0 {
                    let ffi_ref = &*ffi;
                    crate::identity::identity::IdentityV0 {
                        id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.id),
                        public_keys: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.public_keys,
                        ),
                        balance: ffi_ref.balance,
                        revision: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.revision),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::IdentityV0,
                ) -> *const IdentityV0 {
                    ferment_interfaces::boxed(IdentityV0 {
                        id: ferment_interfaces::FFIConversion::ffi_to(obj.id),
                        public_keys: ferment_interfaces::FFIConversion::ffi_to(obj.public_keys),
                        balance: obj.balance,
                        revision: ferment_interfaces::FFIConversion::ffi_to(obj.revision),
                    })
                }
                unsafe fn destroy(ffi: *mut IdentityV0) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for IdentityV0 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.id);
                        ferment_interfaces::unbox_any(ffi_ref.public_keys);
                        ferment_interfaces::unbox_any(ffi_ref.revision);
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn IdentityV0_ctor(
                id: *mut crate::fermented::types::nested::Identifier,
                public_keys : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey,
                balance: u64,
                revision: *mut crate::fermented::types::identity::identity::Revision,
            ) -> *mut IdentityV0 {
                ferment_interfaces::boxed(IdentityV0 {
                    id,
                    public_keys,
                    balance,
                    revision,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_destroy(ffi: *mut IdentityV0) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_get_id(
                obj: *const IdentityV0,
            ) -> *mut crate::fermented::types::nested::Identifier {
                (*obj).id
            }
            #[doc = r" # Safety"]
            #[no_mangle]            pub unsafe extern "C" fn IdentityV0_get_public_keys (obj : * const IdentityV0) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey{
                (*obj).public_keys
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_get_balance(obj: *const IdentityV0) -> u64 {
                (*obj).balance
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_get_revision(
                obj: *const IdentityV0,
            ) -> *mut crate::fermented::types::identity::identity::Revision {
                (*obj).revision
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_set_id(
                obj: *mut IdentityV0,
                value: *mut crate::fermented::types::nested::Identifier,
            ) {
                (*obj).id = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_set_public_keys(
                obj: *mut IdentityV0,
                value : * mut crate :: fermented :: generics :: std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey,
            ) {
                (*obj).public_keys = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_set_balance(obj: *mut IdentityV0, value: u64) {
                (*obj).balance = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityV0_set_revision(
                obj: *mut IdentityV0,
                value: *mut crate::fermented::types::identity::identity::Revision,
            ) {
                (*obj).revision = value;
            }
            #[doc = "FFI-representation of the [`Purpose`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum Purpose {
                AUTHENTICATION = 0,
                ENCRYPTION = 1,
                DECRYPTION = 2,
                WITHDRAW = 3,
                SYSTEM = 4,
                VOTING = 5,
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::Purpose> for Purpose {
                unsafe fn ffi_from_const(
                    ffi: *const Purpose,
                ) -> crate::identity::identity::Purpose {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        Purpose::AUTHENTICATION => {
                            crate::identity::identity::Purpose::AUTHENTICATION
                        }
                        Purpose::ENCRYPTION => crate::identity::identity::Purpose::ENCRYPTION,
                        Purpose::DECRYPTION => crate::identity::identity::Purpose::DECRYPTION,
                        Purpose::WITHDRAW => crate::identity::identity::Purpose::WITHDRAW,
                        Purpose::SYSTEM => crate::identity::identity::Purpose::SYSTEM,
                        Purpose::VOTING => crate::identity::identity::Purpose::VOTING,
                    }
                }
                unsafe fn ffi_to_const(obj: crate::identity::identity::Purpose) -> *const Purpose {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::Purpose::AUTHENTICATION => {
                            Purpose::AUTHENTICATION
                        }
                        crate::identity::identity::Purpose::ENCRYPTION => Purpose::ENCRYPTION,
                        crate::identity::identity::Purpose::DECRYPTION => Purpose::DECRYPTION,
                        crate::identity::identity::Purpose::WITHDRAW => Purpose::WITHDRAW,
                        crate::identity::identity::Purpose::SYSTEM => Purpose::SYSTEM,
                        crate::identity::identity::Purpose::VOTING => Purpose::VOTING,
                    })
                }
                unsafe fn destroy(ffi: *mut Purpose) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Purpose {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            Purpose::AUTHENTICATION => {}
                            Purpose::ENCRYPTION => {}
                            Purpose::DECRYPTION => {}
                            Purpose::WITHDRAW => {}
                            Purpose::SYSTEM => {}
                            Purpose::VOTING => {}
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_AUTHENTICATION_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::AUTHENTICATION)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_ENCRYPTION_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::ENCRYPTION)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_DECRYPTION_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::DECRYPTION)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_WITHDRAW_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::WITHDRAW)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_SYSTEM_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::SYSTEM)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_VOTING_ctor() -> *mut Purpose {
                ferment_interfaces::boxed(Purpose::VOTING)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Purpose_destroy(ffi: *mut Purpose) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`Identity`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum Identity {
                V0(*mut crate::fermented::types::identity::identity::IdentityV0),
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::Identity> for Identity {
                unsafe fn ffi_from_const(
                    ffi: *const Identity,
                ) -> crate::identity::identity::Identity {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        Identity::V0(o_0) => crate::identity::identity::Identity::V0(
                            ferment_interfaces::FFIConversion::ffi_from(*o_0),
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::Identity,
                ) -> *const Identity {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::Identity::V0(o_0) => {
                            Identity::V0(ferment_interfaces::FFIConversion::ffi_to(o_0))
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut Identity) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Identity {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            Identity::V0(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Identity_V0_ctor(
                o_0: *mut crate::fermented::types::identity::identity::IdentityV0,
            ) -> *mut Identity {
                ferment_interfaces::boxed(Identity::V0(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Identity_destroy(ffi: *mut Identity) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`crate::identity::identity::Revision`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct Revision(u64);
            impl ferment_interfaces::FFIConversion<crate::identity::identity::Revision> for Revision {
                unsafe fn ffi_from_const(
                    ffi: *const Revision,
                ) -> crate::identity::identity::Revision {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::Revision,
                ) -> *const Revision {
                    ferment_interfaces::boxed(Revision(obj))
                }
                unsafe fn destroy(ffi: *mut Revision) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for Revision {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn Revision_ctor(o_0: u64) -> *mut Revision {
                ferment_interfaces::boxed(Revision(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Revision_destroy(ffi: *mut Revision) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Revision_get_0(obj: *const Revision) -> u64 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn Revision_set_0(obj: *mut Revision, value: u64) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`SecurityLevel`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum SecurityLevel {
                MASTER = 0,
                CRITICAL = 1,
                HIGH = 2,
                MEDIUM = 3,
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::SecurityLevel> for SecurityLevel {
                unsafe fn ffi_from_const(
                    ffi: *const SecurityLevel,
                ) -> crate::identity::identity::SecurityLevel {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        SecurityLevel::MASTER => crate::identity::identity::SecurityLevel::MASTER,
                        SecurityLevel::CRITICAL => {
                            crate::identity::identity::SecurityLevel::CRITICAL
                        }
                        SecurityLevel::HIGH => crate::identity::identity::SecurityLevel::HIGH,
                        SecurityLevel::MEDIUM => crate::identity::identity::SecurityLevel::MEDIUM,
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::SecurityLevel,
                ) -> *const SecurityLevel {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::SecurityLevel::MASTER => SecurityLevel::MASTER,
                        crate::identity::identity::SecurityLevel::CRITICAL => {
                            SecurityLevel::CRITICAL
                        }
                        crate::identity::identity::SecurityLevel::HIGH => SecurityLevel::HIGH,
                        crate::identity::identity::SecurityLevel::MEDIUM => SecurityLevel::MEDIUM,
                    })
                }
                unsafe fn destroy(ffi: *mut SecurityLevel) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for SecurityLevel {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            SecurityLevel::MASTER => {}
                            SecurityLevel::CRITICAL => {}
                            SecurityLevel::HIGH => {}
                            SecurityLevel::MEDIUM => {}
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn SecurityLevel_MASTER_ctor() -> *mut SecurityLevel {
                ferment_interfaces::boxed(SecurityLevel::MASTER)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn SecurityLevel_CRITICAL_ctor() -> *mut SecurityLevel {
                ferment_interfaces::boxed(SecurityLevel::CRITICAL)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn SecurityLevel_HIGH_ctor() -> *mut SecurityLevel {
                ferment_interfaces::boxed(SecurityLevel::HIGH)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn SecurityLevel_MEDIUM_ctor() -> *mut SecurityLevel {
                ferment_interfaces::boxed(SecurityLevel::MEDIUM)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn SecurityLevel_destroy(ffi: *mut SecurityLevel) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`KeyType`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum KeyType {
                ECDSA_SECP256K1 = 0,
                BLS12_381 = 1,
                ECDSA_HASH160 = 2,
                BIP13_SCRIPT_HASH = 3,
                EDDSA_25519_HASH160 = 4,
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::KeyType> for KeyType {
                unsafe fn ffi_from_const(
                    ffi: *const KeyType,
                ) -> crate::identity::identity::KeyType {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        KeyType::ECDSA_SECP256K1 => {
                            crate::identity::identity::KeyType::ECDSA_SECP256K1
                        }
                        KeyType::BLS12_381 => crate::identity::identity::KeyType::BLS12_381,
                        KeyType::ECDSA_HASH160 => crate::identity::identity::KeyType::ECDSA_HASH160,
                        KeyType::BIP13_SCRIPT_HASH => {
                            crate::identity::identity::KeyType::BIP13_SCRIPT_HASH
                        }
                        KeyType::EDDSA_25519_HASH160 => {
                            crate::identity::identity::KeyType::EDDSA_25519_HASH160
                        }
                    }
                }
                unsafe fn ffi_to_const(obj: crate::identity::identity::KeyType) -> *const KeyType {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::KeyType::ECDSA_SECP256K1 => {
                            KeyType::ECDSA_SECP256K1
                        }
                        crate::identity::identity::KeyType::BLS12_381 => KeyType::BLS12_381,
                        crate::identity::identity::KeyType::ECDSA_HASH160 => KeyType::ECDSA_HASH160,
                        crate::identity::identity::KeyType::BIP13_SCRIPT_HASH => {
                            KeyType::BIP13_SCRIPT_HASH
                        }
                        crate::identity::identity::KeyType::EDDSA_25519_HASH160 => {
                            KeyType::EDDSA_25519_HASH160
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut KeyType) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for KeyType {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            KeyType::ECDSA_SECP256K1 => {}
                            KeyType::BLS12_381 => {}
                            KeyType::ECDSA_HASH160 => {}
                            KeyType::BIP13_SCRIPT_HASH => {}
                            KeyType::EDDSA_25519_HASH160 => {}
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_ECDSA_SECP256K1_ctor() -> *mut KeyType {
                ferment_interfaces::boxed(KeyType::ECDSA_SECP256K1)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_BLS12_381_ctor() -> *mut KeyType {
                ferment_interfaces::boxed(KeyType::BLS12_381)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_ECDSA_HASH160_ctor() -> *mut KeyType {
                ferment_interfaces::boxed(KeyType::ECDSA_HASH160)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_BIP13_SCRIPT_HASH_ctor() -> *mut KeyType {
                ferment_interfaces::boxed(KeyType::BIP13_SCRIPT_HASH)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_EDDSA_25519_HASH160_ctor() -> *mut KeyType {
                ferment_interfaces::boxed(KeyType::EDDSA_25519_HASH160)
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyType_destroy(ffi: *mut KeyType) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`crate::identity::identity::KeyID`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct KeyID(u32);
            impl ferment_interfaces::FFIConversion<crate::identity::identity::KeyID> for KeyID {
                unsafe fn ffi_from_const(ffi: *const KeyID) -> crate::identity::identity::KeyID {
                    let ffi_ref = &*ffi;
                    ffi_ref.0
                }
                unsafe fn ffi_to_const(obj: crate::identity::identity::KeyID) -> *const KeyID {
                    ferment_interfaces::boxed(KeyID(obj))
                }
                unsafe fn destroy(ffi: *mut KeyID) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for KeyID {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn KeyID_ctor(o_0: u32) -> *mut KeyID {
                ferment_interfaces::boxed(KeyID(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyID_destroy(ffi: *mut KeyID) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyID_get_0(obj: *const KeyID) -> u32 {
                (*obj).0
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn KeyID_set_0(obj: *mut KeyID, value: u32) {
                (*obj).0 = value;
            }
            #[doc = "FFI-representation of the [`crate::identity::identity::IdentityPublicKeyV0`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct IdentityPublicKeyV0 {
                pub id: *mut crate::fermented::types::identity::identity::KeyID,
                pub purpose: *mut crate::fermented::types::identity::identity::Purpose,
                pub security_level: *mut crate::fermented::types::identity::identity::SecurityLevel,
                pub contract_bounds:
                    *mut crate::fermented::types::identity::identity::ContractBounds,
                pub key_type: *mut crate::fermented::types::identity::identity::KeyType,
                pub read_only: bool,
                pub data: *mut crate::fermented::types::nested::BinaryData,
                pub disabled_at: *mut crate::fermented::types::identity::identity::TimestampMillis,
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::IdentityPublicKeyV0>
                for IdentityPublicKeyV0
            {
                unsafe fn ffi_from_const(
                    ffi: *const IdentityPublicKeyV0,
                ) -> crate::identity::identity::IdentityPublicKeyV0 {
                    let ffi_ref = &*ffi;
                    crate::identity::identity::IdentityPublicKeyV0 {
                        id: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.id),
                        purpose: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.purpose),
                        security_level: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.security_level,
                        ),
                        contract_bounds: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.contract_bounds,
                        ),
                        key_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.key_type),
                        read_only: ffi_ref.read_only,
                        data: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.data),
                        disabled_at: ferment_interfaces::FFIConversion::ffi_from_opt(
                            ffi_ref.disabled_at,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::IdentityPublicKeyV0,
                ) -> *const IdentityPublicKeyV0 {
                    ferment_interfaces::boxed(IdentityPublicKeyV0 {
                        id: ferment_interfaces::FFIConversion::ffi_to(obj.id),
                        purpose: ferment_interfaces::FFIConversion::ffi_to(obj.purpose),
                        security_level: ferment_interfaces::FFIConversion::ffi_to(
                            obj.security_level,
                        ),
                        contract_bounds: ferment_interfaces::FFIConversion::ffi_to_opt(
                            obj.contract_bounds,
                        ),
                        key_type: ferment_interfaces::FFIConversion::ffi_to(obj.key_type),
                        read_only: obj.read_only,
                        data: ferment_interfaces::FFIConversion::ffi_to(obj.data),
                        disabled_at: ferment_interfaces::FFIConversion::ffi_to_opt(obj.disabled_at),
                    })
                }
                unsafe fn destroy(ffi: *mut IdentityPublicKeyV0) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for IdentityPublicKeyV0 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.id);
                        ferment_interfaces::unbox_any(ffi_ref.purpose);
                        ferment_interfaces::unbox_any(ffi_ref.security_level);
                        if !ffi_ref.contract_bounds.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.contract_bounds);
                        };
                        ferment_interfaces::unbox_any(ffi_ref.key_type);
                        ferment_interfaces::unbox_any(ffi_ref.data);
                        if !ffi_ref.disabled_at.is_null() {
                            ferment_interfaces::unbox_any(ffi_ref.disabled_at);
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            #[inline(never)]
            pub unsafe extern "C" fn IdentityPublicKeyV0_ctor(
                id: *mut crate::fermented::types::identity::identity::KeyID,
                purpose: *mut crate::fermented::types::identity::identity::Purpose,
                security_level: *mut crate::fermented::types::identity::identity::SecurityLevel,
                contract_bounds: *mut crate::fermented::types::identity::identity::ContractBounds,
                key_type: *mut crate::fermented::types::identity::identity::KeyType,
                read_only: bool,
                data: *mut crate::fermented::types::nested::BinaryData,
                disabled_at: *mut crate::fermented::types::identity::identity::TimestampMillis,
            ) -> *mut IdentityPublicKeyV0 {
                ferment_interfaces::boxed(IdentityPublicKeyV0 {
                    id,
                    purpose,
                    security_level,
                    contract_bounds,
                    key_type,
                    read_only,
                    data,
                    disabled_at,
                })
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_destroy(ffi: *mut IdentityPublicKeyV0) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_id(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::KeyID {
                (*obj).id
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_purpose(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::Purpose {
                (*obj).purpose
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_security_level(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::SecurityLevel {
                (*obj).security_level
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_contract_bounds(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::ContractBounds {
                (*obj).contract_bounds
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_key_type(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::KeyType {
                (*obj).key_type
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_read_only(
                obj: *const IdentityPublicKeyV0,
            ) -> bool {
                (*obj).read_only
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_data(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::nested::BinaryData {
                (*obj).data
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_get_disabled_at(
                obj: *const IdentityPublicKeyV0,
            ) -> *mut crate::fermented::types::identity::identity::TimestampMillis {
                (*obj).disabled_at
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_id(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::KeyID,
            ) {
                (*obj).id = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_purpose(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::Purpose,
            ) {
                (*obj).purpose = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_security_level(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::SecurityLevel,
            ) {
                (*obj).security_level = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_contract_bounds(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::ContractBounds,
            ) {
                (*obj).contract_bounds = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_key_type(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::KeyType,
            ) {
                (*obj).key_type = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_read_only(
                obj: *mut IdentityPublicKeyV0,
                value: bool,
            ) {
                (*obj).read_only = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_data(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::nested::BinaryData,
            ) {
                (*obj).data = value;
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKeyV0_set_disabled_at(
                obj: *mut IdentityPublicKeyV0,
                value: *mut crate::fermented::types::identity::identity::TimestampMillis,
            ) {
                (*obj).disabled_at = value;
            }
            #[doc = "FFI-representation of the [`create_platform_v0`]"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn create_platform_v0(
                identity: *mut crate::fermented::types::nested::FeatureVersionBounds,
                proofs: *mut crate::fermented::types::nested::FeatureVersionBounds,
            ) -> *mut crate::fermented::types::nested::PlatformVersion {
                let obj = crate::identity::identity::create_platform_v0(
                    ferment_interfaces::FFIConversion::ffi_from(identity),
                    ferment_interfaces::FFIConversion::ffi_from(proofs),
                );
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the [`IdentityPublicKey`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum IdentityPublicKey {
                V0(*mut crate::fermented::types::identity::identity::IdentityPublicKeyV0),
            }
            impl ferment_interfaces::FFIConversion<crate::identity::identity::IdentityPublicKey>
                for IdentityPublicKey
            {
                unsafe fn ffi_from_const(
                    ffi: *const IdentityPublicKey,
                ) -> crate::identity::identity::IdentityPublicKey {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        IdentityPublicKey::V0(o_0) => {
                            crate::identity::identity::IdentityPublicKey::V0(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: crate::identity::identity::IdentityPublicKey,
                ) -> *const IdentityPublicKey {
                    ferment_interfaces::boxed(match obj {
                        crate::identity::identity::IdentityPublicKey::V0(o_0) => {
                            IdentityPublicKey::V0(ferment_interfaces::FFIConversion::ffi_to(o_0))
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut IdentityPublicKey) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for IdentityPublicKey {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            IdentityPublicKey::V0(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        }
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKey_V0_ctor(
                o_0: *mut crate::fermented::types::identity::identity::IdentityPublicKeyV0,
            ) -> *mut IdentityPublicKey {
                ferment_interfaces::boxed(IdentityPublicKey::V0(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn IdentityPublicKey_destroy(ffi: *mut IdentityPublicKey) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
    }
}
#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod generics {
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u8) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u8 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_primitive_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj),
            })
        }
    }
    impl Drop for Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    #[inline(never)]
    pub unsafe extern "C" fn Vec_u8_ctor(values: *mut u8, count: usize) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey
    {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::types::identity::identity::KeyID,
        pub values: *mut *mut crate::fermented::types::identity::identity::IdentityPublicKey,
    }
    impl ferment_interfaces :: FFIConversion < std :: collections :: BTreeMap < crate :: identity :: identity :: KeyID , crate :: identity :: identity :: IdentityPublicKey > > for std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey { unsafe fn ffi_from_const (ffi : * const std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey) -> std :: collections :: BTreeMap < crate :: identity :: identity :: KeyID , crate :: identity :: identity :: IdentityPublicKey > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_map (ffi_ref . count , ffi_ref . keys , ffi_ref . values , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : std :: collections :: BTreeMap < crate :: identity :: identity :: KeyID , crate :: identity :: identity :: IdentityPublicKey >) -> * const std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey { ferment_interfaces :: boxed (Self { count : obj . len () , keys : ferment_interfaces :: to_complex_vec (obj . keys () . cloned ()) , values : ferment_interfaces :: to_complex_vec (obj . values () . cloned ()) }) } unsafe fn destroy (ffi : * mut std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey { fn drop (& mut self) { unsafe { ferment_interfaces :: unbox_any_vec_ptr (self . keys , self . count) ; ferment_interfaces :: unbox_any_vec_ptr (self . values , self . count) ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]
    #[inline(never)]    pub unsafe extern "C" fn std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_ctor (keys : * mut * mut crate :: fermented :: types :: identity :: identity :: KeyID , values : * mut * mut crate :: fermented :: types :: identity :: identity :: IdentityPublicKey , count : usize) -> * mut std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey{
        ferment_interfaces :: boxed (std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey { count , keys , values , })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey_destroy(
        ffi : * mut std_collections_Map_keys_crate_identity_identity_KeyID_values_crate_identity_identity_IdentityPublicKey,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError {
        pub ok: *mut crate::fermented::types::identity::identity::Identity,
        pub error: *mut crate::fermented::types::nested::ProtocolError,
    }
    impl
        ferment_interfaces::FFIConversion<
            Result<crate::identity::identity::Identity, crate::nested::ProtocolError>,
        > for Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError,
        ) -> Result<crate::identity::identity::Identity, crate::nested::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<crate::identity::identity::Identity, crate::nested::ProtocolError>,
        ) -> *const Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError
        {
            let (ok, error) = match obj {
                Ok(o) => (
                    ferment_interfaces::FFIConversion::ffi_to(o),
                    std::ptr::null_mut(),
                ),
                Err(o) => (
                    std::ptr::null_mut(),
                    ferment_interfaces::FFIConversion::ffi_to(o),
                ),
            };
            ferment_interfaces::boxed(Self { ok, error })
        }
        unsafe fn destroy(
            ffi: *mut Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                }
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    #[inline(never)]
    pub unsafe extern "C" fn Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_ctor(
        ok: *mut crate::fermented::types::identity::identity::Identity,
        error: *mut crate::fermented::types::nested::ProtocolError,
    ) -> *mut Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError {
        ferment_interfaces::boxed(
            Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError_destroy(
        ffi: *mut Result_ok_crate_identity_identity_Identity_err_crate_nested_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_crate_nested_FeatureVersion {
        pub count: usize,
        pub values: *mut *mut crate::fermented::types::nested::FeatureVersion,
    }
    impl ferment_interfaces::FFIConversion<Vec<crate::nested::FeatureVersion>>
        for Vec_crate_nested_FeatureVersion
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_crate_nested_FeatureVersion,
        ) -> Vec<crate::nested::FeatureVersion> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<crate::nested::FeatureVersion>,
        ) -> *const Vec_crate_nested_FeatureVersion {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_crate_nested_FeatureVersion) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_crate_nested_FeatureVersion {
        type Value = Vec<crate::nested::FeatureVersion>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_vec(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_crate_nested_FeatureVersion {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    #[inline(never)]
    pub unsafe extern "C" fn Vec_crate_nested_FeatureVersion_ctor(
        values: *mut *mut crate::fermented::types::nested::FeatureVersion,
        count: usize,
    ) -> *mut Vec_crate_nested_FeatureVersion {
        ferment_interfaces::boxed(Vec_crate_nested_FeatureVersion { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_crate_nested_FeatureVersion_destroy(
        ffi: *mut Vec_crate_nested_FeatureVersion,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unused_braces,
    unused_imports,
    unused_unsafe,
    unused_variables,
    unused_qualifications
)]
pub mod vtables {}
