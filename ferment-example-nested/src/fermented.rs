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
    pub mod ferment_example {
        use crate as ferment_example_nested;
        pub mod data_contract {
            use crate as ferment_example_nested;
            pub mod v1 {
                use crate as ferment_example_nested;
                pub mod data_contract {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example::data_contract::v1::data_contract::DataContractV1`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_data_contract_v1_data_contract_DataContractV1 {}
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example::data_contract::v1::data_contract::DataContractV1,
                        > for ferment_example_data_contract_v1_data_contract_DataContractV1
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const ferment_example_data_contract_v1_data_contract_DataContractV1,
                        ) -> ferment_example::data_contract::v1::data_contract::DataContractV1
                        {
                            let ffi_ref = &*ffi;
                            ferment_example::data_contract::v1::data_contract::DataContractV1 {}
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example::data_contract::v1::data_contract::DataContractV1,
                        ) -> *const ferment_example_data_contract_v1_data_contract_DataContractV1
                        {
                            ferment_interfaces::boxed(
                                ferment_example_data_contract_v1_data_contract_DataContractV1 {},
                            )
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_data_contract_v1_data_contract_DataContractV1,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_data_contract_v1_data_contract_DataContractV1 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v1_data_contract_DataContractV1_ctor(
                    ) -> *mut ferment_example_data_contract_v1_data_contract_DataContractV1
                    {
                        ferment_interfaces::boxed(
                            ferment_example_data_contract_v1_data_contract_DataContractV1 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v1_data_contract_DataContractV1_destroy(
                        ffi: *mut ferment_example_data_contract_v1_data_contract_DataContractV1,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
            #[doc = "FFI-representation of the [`ferment_example::data_contract::DataContract`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_data_contract_DataContract {
                V0 (* mut crate :: fermented :: types :: ferment_example :: data_contract :: v0 :: data_contract :: ferment_example_data_contract_v0_data_contract_DataContractV0) , V1 (* mut crate :: fermented :: types :: ferment_example :: data_contract :: v1 :: data_contract :: ferment_example_data_contract_v1_data_contract_DataContractV1) }
            impl ferment_interfaces::FFIConversion<ferment_example::data_contract::DataContract>
                for ferment_example_data_contract_DataContract
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_data_contract_DataContract,
                ) -> ferment_example::data_contract::DataContract {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_data_contract_DataContract::V0(o_0) => {
                            ferment_example::data_contract::DataContract::V0(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_data_contract_DataContract::V1(o_0) => {
                            ferment_example::data_contract::DataContract::V1(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::data_contract::DataContract,
                ) -> *const ferment_example_data_contract_DataContract {
                    ferment_interfaces::boxed(match obj {
                        ferment_example::data_contract::DataContract::V0(o_0) => {
                            ferment_example_data_contract_DataContract::V0(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example::data_contract::DataContract::V1(o_0) => {
                            ferment_example_data_contract_DataContract::V1(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_data_contract_DataContract) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_data_contract_DataContract {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_data_contract_DataContract::V0(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                            ferment_example_data_contract_DataContract::V1(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_V0_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example :: data_contract :: v0 :: data_contract :: ferment_example_data_contract_v0_data_contract_DataContractV0,
            ) -> *mut ferment_example_data_contract_DataContract {
                ferment_interfaces::boxed(ferment_example_data_contract_DataContract::V0(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_V1_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example :: data_contract :: v1 :: data_contract :: ferment_example_data_contract_v1_data_contract_DataContractV1,
            ) -> *mut ferment_example_data_contract_DataContract {
                ferment_interfaces::boxed(ferment_example_data_contract_DataContract::V1(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_data_contract_DataContract_destroy(
                ffi: *mut ferment_example_data_contract_DataContract,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            pub mod v0 {
                use crate as ferment_example_nested;
                pub mod data_contract {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example::data_contract::v0::data_contract::DataContractV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_data_contract_v0_data_contract_DataContractV0 {}
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example::data_contract::v0::data_contract::DataContractV0,
                        > for ferment_example_data_contract_v0_data_contract_DataContractV0
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const ferment_example_data_contract_v0_data_contract_DataContractV0,
                        ) -> ferment_example::data_contract::v0::data_contract::DataContractV0
                        {
                            let ffi_ref = &*ffi;
                            ferment_example::data_contract::v0::data_contract::DataContractV0 {}
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example::data_contract::v0::data_contract::DataContractV0,
                        ) -> *const ferment_example_data_contract_v0_data_contract_DataContractV0
                        {
                            ferment_interfaces::boxed(
                                ferment_example_data_contract_v0_data_contract_DataContractV0 {},
                            )
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_data_contract_v0_data_contract_DataContractV0,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_data_contract_v0_data_contract_DataContractV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v0_data_contract_DataContractV0_ctor(
                    ) -> *mut ferment_example_data_contract_v0_data_contract_DataContractV0
                    {
                        ferment_interfaces::boxed(
                            ferment_example_data_contract_v0_data_contract_DataContractV0 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_data_contract_v0_data_contract_DataContractV0_destroy(
                        ffi: *mut ferment_example_data_contract_v0_data_contract_DataContractV0,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
        }
        #[doc = "FFI-representation of the [`ferment_example::get_root_struct`]"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_get_root_struct(
        ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct {
            let obj = ferment_example::get_root_struct();
            ferment_interfaces::FFIConversion::ffi_to(obj)
        }
        pub mod errors {
            use crate as ferment_example_nested;
            pub mod protocol_error {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example::errors::protocol_error::ProtocolError`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_errors_protocol_error_ProtocolError {
                    InvalidPKT (* mut crate :: fermented :: types :: ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::errors::protocol_error::ProtocolError,
                    > for ferment_example_errors_protocol_error_ProtocolError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_errors_protocol_error_ProtocolError,
                    ) -> ferment_example::errors::protocol_error::ProtocolError
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                o_0,
                            ) => {
                                ferment_example::errors::protocol_error::ProtocolError::InvalidPKT(
                                    ferment_interfaces::FFIConversion::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::errors::protocol_error::ProtocolError,
                    ) -> *const ferment_example_errors_protocol_error_ProtocolError
                    {
                        ferment_interfaces::boxed(match obj {
                            ferment_example::errors::protocol_error::ProtocolError::InvalidPKT(
                                o_0,
                            ) => ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            ),
                        })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_errors_protocol_error_ProtocolError,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_errors_protocol_error_ProtocolError {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(
                                    o_0,
                                ) => {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_protocol_error_ProtocolError_InvalidPKT_ctor(
                    o_0 : * mut crate :: fermented :: types :: ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                ) -> *mut ferment_example_errors_protocol_error_ProtocolError {
                    ferment_interfaces::boxed(
                        ferment_example_errors_protocol_error_ProtocolError::InvalidPKT(o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_errors_protocol_error_ProtocolError_destroy(
                    ffi: *mut ferment_example_errors_protocol_error_ProtocolError,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
        }
        pub mod nested {
            use crate as ferment_example_nested;
            #[doc = "FFI-representation of the [`ferment_example::nested::RootUser`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_RootUser {
                pub root: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            }
            impl ferment_interfaces::FFIConversion<ferment_example::nested::RootUser>
                for ferment_example_nested_RootUser
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_RootUser,
                ) -> ferment_example::nested::RootUser {
                    let ffi_ref = &*ffi;
                    ferment_example::nested::RootUser {
                        root: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.root),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::RootUser,
                ) -> *const ferment_example_nested_RootUser {
                    ferment_interfaces::boxed(ferment_example_nested_RootUser {
                        root: ferment_interfaces::FFIConversion::ffi_to(obj.root),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_RootUser) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_RootUser {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.root);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_ctor(
                root: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            ) -> *mut ferment_example_nested_RootUser {
                ferment_interfaces::boxed(ferment_example_nested_RootUser { root })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_destroy(
                ffi: *mut ferment_example_nested_RootUser,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_get_root(
                obj: *const ferment_example_nested_RootUser,
            ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
            {
                (*obj).root
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_RootUser_set_root(
                obj: *mut ferment_example_nested_RootUser,
                value: *mut crate::fermented::types::ferment_example::ferment_example_RootStruct,
            ) {
                (*obj).root = value;
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::HashID`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_HashID(*mut crate::fermented::generics::Arr_u8_32);
            impl ferment_interfaces::FFIConversion<ferment_example::nested::HashID>
                for ferment_example_nested_HashID
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_HashID,
                ) -> ferment_example::nested::HashID {
                    let ffi_ref = &*ffi;
                    ferment_interfaces::FFIConversion::ffi_from(ffi_ref.0)
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example::nested::HashID,
                ) -> *const ferment_example_nested_HashID {
                    ferment_interfaces::boxed(ferment_example_nested_HashID(
                        ferment_interfaces::FFIConversion::ffi_to(obj),
                    ))
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_HashID) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_HashID {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_ctor(
                o_0: *mut crate::fermented::generics::Arr_u8_32,
            ) -> *mut ferment_example_nested_HashID {
                ferment_interfaces::boxed(ferment_example_nested_HashID(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_destroy(
                ffi: *mut ferment_example_nested_HashID,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_get_0(
                obj: *const ferment_example_nested_HashID,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_HashID_set_0(
                obj: *mut ferment_example_nested_HashID,
                value: *mut crate::fermented::generics::Arr_u8_32,
            ) {
                (*obj).0 = value;
            }
            pub mod double_nested {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example::nested::double_nested::get_root_struct_3`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_double_nested_get_root_struct_3(
                ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
                {
                    let obj = ferment_example::nested::double_nested::get_root_struct_3();
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
            #[doc = "FFI-representation of the [`ferment_example::nested::get_root_struct_2`]"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_get_root_struct_2(
            ) -> *mut crate::fermented::types::ferment_example::ferment_example_RootStruct
            {
                let obj = ferment_example::nested::get_root_struct_2();
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
        #[doc = "FFI-representation of the [`ferment_example::RootStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_RootStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment_interfaces::FFIConversion<ferment_example::RootStruct> for ferment_example_RootStruct {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_RootStruct,
            ) -> ferment_example::RootStruct {
                let ffi_ref = &*ffi;
                ferment_example::RootStruct {
                    name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example::RootStruct,
            ) -> *const ferment_example_RootStruct {
                ferment_interfaces::boxed(ferment_example_RootStruct {
                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_RootStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_RootStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                        ffi_ref.name,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_RootStruct {
            ferment_interfaces::boxed(ferment_example_RootStruct { name })
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_destroy(
            ffi: *mut ferment_example_RootStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_get_name(
            obj: *const ferment_example_RootStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_RootStruct_set_name(
            obj: *mut ferment_example_RootStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
        }
        pub mod state_transition {
            use crate as ferment_example_nested;
            pub mod errors {
                use crate as ferment_example_nested;
                pub mod invalid_identity_public_key_type_error {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError
                    {
                        pub public_key_type: *mut std::os::raw::c_char,
                    }
                    impl ferment_interfaces :: FFIConversion < ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError > for ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { unsafe fn ffi_from_const (ffi : * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) -> ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError { let ffi_ref = & * ffi ; ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError { public_key_type : ferment_interfaces :: FFIConversion :: ffi_from (ffi_ref . public_key_type) } } unsafe fn ffi_to_const (obj : ferment_example :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError) -> * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { ferment_interfaces :: boxed (ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type : ferment_interfaces :: FFIConversion :: ffi_to (obj . public_key_type) }) } unsafe fn destroy (ffi : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) { ferment_interfaces :: unbox_any (ffi) ; } }
                    impl Drop for ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { fn drop (& mut self) { unsafe { let ffi_ref = self ; < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . public_key_type) ; } } }
                    #[no_mangle]                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_ctor (public_key_type : * mut std :: os :: raw :: c_char) -> * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError{
                        ferment_interfaces :: boxed (ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_destroy(
                        ffi : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_get_public_key_type(
                        obj : * const ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).public_key_type
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_set_public_key_type(
                        obj : * mut ferment_example_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                        value: *mut std::os::raw::c_char,
                    ) {
                        (*obj).public_key_type = value;
                    }
                }
            }
        }
        pub mod example {
            use crate as ferment_example_nested;
            pub mod custom_conversion {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example::example::custom_conversion::StructUsesDuration`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_example_custom_conversion_StructUsesDuration {
                    pub time: *mut ferment_example::std_time_Duration,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example::example::custom_conversion::StructUsesDuration,
                    > for ferment_example_example_custom_conversion_StructUsesDuration
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_example_custom_conversion_StructUsesDuration,
                    ) -> ferment_example::example::custom_conversion::StructUsesDuration
                    {
                        let ffi_ref = &*ffi;
                        ferment_example::example::custom_conversion::StructUsesDuration {
                            time: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.time),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example::example::custom_conversion::StructUsesDuration,
                    ) -> *const ferment_example_example_custom_conversion_StructUsesDuration
                    {
                        ferment_interfaces::boxed(
                            ferment_example_example_custom_conversion_StructUsesDuration {
                                time: ferment_interfaces::FFIConversion::ffi_to(obj.time),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_example_custom_conversion_StructUsesDuration,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_example_custom_conversion_StructUsesDuration {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.time);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDuration_ctor(
                    time: *mut ferment_example::std_time_Duration,
                ) -> *mut ferment_example_example_custom_conversion_StructUsesDuration
                {
                    ferment_interfaces::boxed(
                        ferment_example_example_custom_conversion_StructUsesDuration { time },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDuration_destroy(
                    ffi: *mut ferment_example_example_custom_conversion_StructUsesDuration,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDuration_get_time(
                    obj: *const ferment_example_example_custom_conversion_StructUsesDuration,
                ) -> *mut ferment_example::std_time_Duration {
                    (*obj).time
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_custom_conversion_StructUsesDuration_set_time(
                    obj: *mut ferment_example_example_custom_conversion_StructUsesDuration,
                    value: *mut ferment_example::std_time_Duration,
                ) {
                    (*obj).time = value;
                }
            }
            pub mod address {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example::example::address::address_with_script_pubkey`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_address_address_with_script_pubkey(
                    script: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut std::os::raw::c_char {
                    let obj = ferment_example::example::address::address_with_script_pubkey(
                        ferment_interfaces::FFIConversion::ffi_from(script),
                    );
                    ferment_interfaces::FFIConversion::ffi_to_opt(obj)
                }
                #[doc = "FFI-representation of the [`ferment_example::example::address::address_simple_result`]"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_example_address_address_simple_result(
                    script: *mut crate::fermented::generics::Vec_u32,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    let obj = ferment_example::example::address::address_simple_result(
                        ferment_interfaces::FFIConversion::ffi_from(script),
                    );
                    ferment_interfaces::FFIConversion::ffi_to(obj)
                }
            }
        }
    }
    pub mod ferment_example_nested {
        use crate as ferment_example_nested;
        pub mod model {
            use crate as ferment_example_nested;
            pub mod snapshot {
                use crate as ferment_example_nested;
                #[doc = "FFI-representation of the [`ferment_example_nested::model::snapshot::LLMQSnapshot`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_nested_model_snapshot_LLMQSnapshot { pub member_list : * mut crate :: fermented :: generics :: Vec_u8 , pub skip_list : * mut crate :: fermented :: generics :: Vec_i32 , pub skip_list_mode : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode , pub option_vec : * mut crate :: fermented :: generics :: Vec_u8 }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::model::snapshot::LLMQSnapshot,
                    > for ferment_example_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                    ) -> ferment_example_nested::model::snapshot::LLMQSnapshot {
                        let ffi_ref = &*ffi;
                        ferment_example_nested::model::snapshot::LLMQSnapshot {
                            member_list: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.member_list,
                            ),
                            skip_list: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.skip_list,
                            ),
                            skip_list_mode: ferment_interfaces::FFIConversion::ffi_from(
                                ffi_ref.skip_list_mode,
                            ),
                            option_vec: ferment_interfaces::FFIConversion::ffi_from_opt(
                                ffi_ref.option_vec,
                            ),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::model::snapshot::LLMQSnapshot,
                    ) -> *const ferment_example_nested_model_snapshot_LLMQSnapshot
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_snapshot_LLMQSnapshot {
                                member_list: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.member_list,
                                ),
                                skip_list: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list),
                                skip_list_mode: ferment_interfaces::FFIConversion::ffi_to(
                                    obj.skip_list_mode,
                                ),
                                option_vec: match obj.option_vec {
                                    Some(vec) => ferment_interfaces::FFIConversion::ffi_to(vec),
                                    None => std::ptr::null_mut(),
                                },
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_model_snapshot_LLMQSnapshot {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.member_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list_mode);
                            if !(ffi_ref.option_vec).is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.option_vec);
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_ctor(
                    member_list: *mut crate::fermented::generics::Vec_u8,
                    skip_list: *mut crate::fermented::generics::Vec_i32,
                    skip_list_mode : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    option_vec: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshot {
                    ferment_interfaces::boxed(ferment_example_nested_model_snapshot_LLMQSnapshot {
                        member_list,
                        skip_list,
                        skip_list_mode,
                        option_vec,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_member_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_skip_list_mode (obj : * const ferment_example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_get_option_vec(
                    obj: *const ferment_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_member_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).member_list = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_i32,
                ) {
                    (*obj).skip_list = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_skip_list_mode(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    (*obj).skip_list_mode = value;
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshot_set_option_vec(
                    obj: *mut ferment_example_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).option_vec = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    NoSkipping = 0,
                    SkipFirst = 1,
                    SkipExcept = 2,
                    SkipAll = 3,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    > for ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) -> ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    ) -> *const ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept , ferment_example_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll => ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => { } , ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => { } } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_NoSkipping_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping,
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipFirst_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst,
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipExcept_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept,
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipAll_ctor(
                ) -> *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll,
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
                    ffi: *mut ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            pub mod quorum {
                use crate as ferment_example_nested;
                pub mod quorum_type {
                    use crate as ferment_example_nested;
                    #[doc = "FFI-representation of the [`ferment_example_nested::model::quorum::quorum_type::QuorumType`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub enum ferment_example_nested_model_quorum_quorum_type_QuorumType {
                        Normal,
                        Rotated,
                    }
                    impl
                        ferment_interfaces::FFIConversion<
                            ferment_example_nested::model::quorum::quorum_type::QuorumType,
                        > for ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const ferment_example_nested_model_quorum_quorum_type_QuorumType,
                        ) -> ferment_example_nested::model::quorum::quorum_type::QuorumType
                        {
                            let ffi_ref = &*ffi;
                            match ffi_ref { ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal => ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Normal , ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated => ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Rotated }
                        }
                        unsafe fn ffi_to_const(
                            obj: ferment_example_nested::model::quorum::quorum_type::QuorumType,
                        ) -> *const ferment_example_nested_model_quorum_quorum_type_QuorumType
                        {
                            ferment_interfaces :: boxed (match obj { ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Normal => ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal , ferment_example_nested :: model :: quorum :: quorum_type :: QuorumType :: Rotated => ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated })
                        }
                        unsafe fn destroy(
                            ffi: *mut ferment_example_nested_model_quorum_quorum_type_QuorumType,
                        ) {
                            ferment_interfaces::unbox_any(ffi);
                        }
                    }
                    impl Drop for ferment_example_nested_model_quorum_quorum_type_QuorumType {
                        fn drop(&mut self) {
                            unsafe {
                                match self { ferment_example_nested_model_quorum_quorum_type_QuorumType :: Normal => { } , ferment_example_nested_model_quorum_quorum_type_QuorumType :: Rotated => { } } ;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_Normal_ctor(
                    ) -> *mut ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_quorum_quorum_type_QuorumType::Normal,
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_Rotated_ctor(
                    ) -> *mut ferment_example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment_interfaces::boxed(
                            ferment_example_nested_model_quorum_quorum_type_QuorumType::Rotated,
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn ferment_example_nested_model_quorum_quorum_type_QuorumType_destroy(
                        ffi: *mut ferment_example_nested_model_quorum_quorum_type_QuorumType,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::model::TestModLevelOptSnapshot`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_nested_model_TestModLevelOptSnapshot {
                VO (* mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelOptSnapshot,
                > for ferment_example_nested_model_TestModLevelOptSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelOptSnapshot,
                ) -> ferment_example_nested::model::TestModLevelOptSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelOptSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from_opt(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelOptSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelOptSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelOptSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelOptSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to_opt(o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelOptSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelOptSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                                if !(*o_0).is_null() {
                                    ferment_interfaces::unbox_any(*o_0);
                                }
                            }
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelOptSnapshot_VO_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut ferment_example_nested_model_TestModLevelOptSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelOptSnapshot::VO(
                    o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelOptSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelOptSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::model::Quorum`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_model_Quorum { pub llmq_type : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType }
            impl ferment_interfaces::FFIConversion<ferment_example_nested::model::Quorum>
                for ferment_example_nested_model_Quorum
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_Quorum,
                ) -> ferment_example_nested::model::Quorum {
                    let ffi_ref = &*ffi;
                    ferment_example_nested::model::Quorum {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.llmq_type),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::Quorum,
                ) -> *const ferment_example_nested_model_Quorum {
                    ferment_interfaces::boxed(ferment_example_nested_model_Quorum {
                        llmq_type: ferment_interfaces::FFIConversion::ffi_to(obj.llmq_type),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_Quorum) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_Quorum {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment_interfaces::unbox_any(ffi_ref.llmq_type);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_ctor(
                llmq_type : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType,
            ) -> *mut ferment_example_nested_model_Quorum {
                ferment_interfaces::boxed(ferment_example_nested_model_Quorum { llmq_type })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_destroy(
                ffi: *mut ferment_example_nested_model_Quorum,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_get_llmq_type (obj : * const ferment_example_nested_model_Quorum) -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType{
                (*obj).llmq_type
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_Quorum_set_llmq_type(
                obj: *mut ferment_example_nested_model_Quorum,
                value : * mut crate :: fermented :: types :: ferment_example_nested :: model :: quorum :: quorum_type :: ferment_example_nested_model_quorum_quorum_type_QuorumType,
            ) {
                (*obj).llmq_type = value;
            }
            pub mod ferment_example {
                use crate as ferment_example_nested;
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::model::TestModLevelVecSnapshot`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_nested_model_TestModLevelVecSnapshot {
                VO (* mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelVecSnapshot,
                > for ferment_example_nested_model_TestModLevelVecSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelVecSnapshot,
                ) -> ferment_example_nested::model::TestModLevelVecSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelVecSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelVecSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelVecSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelVecSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelVecSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelVecSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelVecSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelVecSnapshot_VO_ctor(
                o_0 : * mut crate :: fermented :: generics :: Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut ferment_example_nested_model_TestModLevelVecSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelVecSnapshot::VO(
                    o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelVecSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelVecSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::model::TestModLevelSnapshot`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_nested_model_TestModLevelSnapshot {
                VO (* mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot) }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::model::TestModLevelSnapshot,
                > for ferment_example_nested_model_TestModLevelSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_model_TestModLevelSnapshot,
                ) -> ferment_example_nested::model::TestModLevelSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                            ferment_example_nested::model::TestModLevelSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::model::TestModLevelSnapshot,
                ) -> *const ferment_example_nested_model_TestModLevelSnapshot {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_nested::model::TestModLevelSnapshot::VO(o_0) => {
                            ferment_example_nested_model_TestModLevelSnapshot::VO(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_model_TestModLevelSnapshot) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_model_TestModLevelSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelSnapshot_VO_ctor(
                o_0 : * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshot,
            ) -> *mut ferment_example_nested_model_TestModLevelSnapshot {
                ferment_interfaces::boxed(ferment_example_nested_model_TestModLevelSnapshot::VO(
                    o_0,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_model_TestModLevelSnapshot_destroy(
                ffi: *mut ferment_example_nested_model_TestModLevelSnapshot,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod some_inner_2 {
            use crate as ferment_example_nested;
            #[doc = "FFI-representation of the [`ferment_example_nested::some_inner_2::set_btree_set`]"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_set_btree_set(
                set: *mut crate::fermented::generics::std_collections_BTreeSet_String,
            ) {
                let obj = ferment_example_nested::some_inner_2::set_btree_set(
                    ferment_interfaces::FFIConversion::ffi_from(set),
                );
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::some_inner_2::get_btree_set`]"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_get_btree_set(
            ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String {
                let obj = ferment_example_nested::some_inner_2::get_btree_set();
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::some_inner_2::get_normal_quorum`]"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_get_normal_quorum () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                let obj = ferment_example_nested::some_inner_2::get_normal_quorum();
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
            #[doc = "FFI-representation of the [`ferment_example_nested::some_inner_2::DocumentTypeV0`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_nested_some_inner_2_DocumentTypeV0 {
                pub name: *mut std::os::raw::c_char,
                pub identifier_paths:
                    *mut crate::fermented::generics::std_collections_BTreeSet_String,
                pub binary_paths: *mut crate::fermented::generics::std_collections_BTreeSet_String,
            }
            impl
                ferment_interfaces::FFIConversion<
                    ferment_example_nested::some_inner_2::DocumentTypeV0,
                > for ferment_example_nested_some_inner_2_DocumentTypeV0
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_nested_some_inner_2_DocumentTypeV0,
                ) -> ferment_example_nested::some_inner_2::DocumentTypeV0 {
                    let ffi_ref = &*ffi;
                    ferment_example_nested::some_inner_2::DocumentTypeV0 {
                        name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                        identifier_paths: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.identifier_paths,
                        ),
                        binary_paths: ferment_interfaces::FFIConversion::ffi_from(
                            ffi_ref.binary_paths,
                        ),
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_nested::some_inner_2::DocumentTypeV0,
                ) -> *const ferment_example_nested_some_inner_2_DocumentTypeV0 {
                    ferment_interfaces::boxed(ferment_example_nested_some_inner_2_DocumentTypeV0 {
                        name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                        identifier_paths: ferment_interfaces::FFIConversion::ffi_to(
                            obj.identifier_paths,
                        ),
                        binary_paths: ferment_interfaces::FFIConversion::ffi_to(obj.binary_paths),
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_nested_some_inner_2_DocumentTypeV0) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_nested_some_inner_2_DocumentTypeV0 {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . name) ;
                        ferment_interfaces::unbox_any(ffi_ref.identifier_paths);
                        ferment_interfaces::unbox_any(ffi_ref.binary_paths);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_ctor(
                name: *mut std::os::raw::c_char,
                identifier_paths: *mut crate::fermented::generics::std_collections_BTreeSet_String,
                binary_paths: *mut crate::fermented::generics::std_collections_BTreeSet_String,
            ) -> *mut ferment_example_nested_some_inner_2_DocumentTypeV0 {
                ferment_interfaces::boxed(ferment_example_nested_some_inner_2_DocumentTypeV0 {
                    name,
                    identifier_paths,
                    binary_paths,
                })
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_destroy(
                ffi: *mut ferment_example_nested_some_inner_2_DocumentTypeV0,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_get_name(
                obj: *const ferment_example_nested_some_inner_2_DocumentTypeV0,
            ) -> *mut std::os::raw::c_char {
                (*obj).name
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_get_identifier_paths(
                obj: *const ferment_example_nested_some_inner_2_DocumentTypeV0,
            ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String {
                (*obj).identifier_paths
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_get_binary_paths(
                obj: *const ferment_example_nested_some_inner_2_DocumentTypeV0,
            ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String {
                (*obj).binary_paths
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_set_name(
                obj: *mut ferment_example_nested_some_inner_2_DocumentTypeV0,
                value: *mut std::os::raw::c_char,
            ) {
                (*obj).name = value;
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_set_identifier_paths(
                obj: *mut ferment_example_nested_some_inner_2_DocumentTypeV0,
                value: *mut crate::fermented::generics::std_collections_BTreeSet_String,
            ) {
                (*obj).identifier_paths = value;
            }
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_nested_some_inner_2_DocumentTypeV0_set_binary_paths(
                obj: *mut ferment_example_nested_some_inner_2_DocumentTypeV0,
                value: *mut crate::fermented::generics::std_collections_BTreeSet_String,
            ) {
                (*obj).binary_paths = value;
            }
        }
        pub mod some_inner {
            use crate as ferment_example_nested;
            #[doc = "FFI-representation of the [`ferment_example_nested::some_inner::get_normal_quorum`]"]
            #[no_mangle]            pub unsafe extern "C" fn ferment_example_nested_some_inner_get_normal_quorum () -> * mut crate :: fermented :: types :: ferment_example_nested :: model :: ferment_example_nested_model_Quorum{
                let obj = ferment_example_nested::some_inner::get_normal_quorum();
                ferment_interfaces::FFIConversion::ffi_to(obj)
            }
        }
        #[doc = "FFI-representation of the [`ferment_example_nested::SomeStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_nested_SomeStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment_interfaces::FFIConversion<ferment_example_nested::SomeStruct>
            for ferment_example_nested_SomeStruct
        {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_nested_SomeStruct,
            ) -> ferment_example_nested::SomeStruct {
                let ffi_ref = &*ffi;
                ferment_example_nested::SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example_nested::SomeStruct,
            ) -> *const ferment_example_nested_SomeStruct {
                ferment_interfaces::boxed(ferment_example_nested_SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_nested_SomeStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_nested_SomeStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment_interfaces::FFIConversion<String>>::destroy(
                        ffi_ref.name,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_nested_SomeStruct {
            ferment_interfaces::boxed(ferment_example_nested_SomeStruct { name })
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_destroy(
            ffi: *mut ferment_example_nested_SomeStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_get_name(
            obj: *const ferment_example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_nested_SomeStruct_set_name(
            obj: *mut ferment_example_nested_SomeStruct,
            value: *mut std::os::raw::c_char,
        ) {
            (*obj).name = value;
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
    use crate as ferment_example_nested;
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Vec<u32>> for Vec_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_u32) -> Vec<u32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<u32>) -> *const Vec_u32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_u32 {
        type Value = Vec<u32>;
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
    impl Drop for Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_ctor(count: usize, values: *mut u32) -> *mut Vec_u32 {
        ferment_interfaces::boxed(Vec_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_destroy(ffi: *mut Vec_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment_interfaces::FFIConversion<std::collections::BTreeSet<String>>
        for std_collections_BTreeSet_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_String,
        ) -> std::collections::BTreeSet<String> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<String>,
        ) -> *const std_collections_BTreeSet_String {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_String) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for std_collections_BTreeSet_String {
        type Value = std::collections::BTreeSet<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment_interfaces::from_complex_btree_set(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::to_complex_vec(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_String {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_BTreeSet_String {
        ferment_interfaces::boxed(std_collections_BTreeSet_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_destroy(
        ffi: *mut std_collections_BTreeSet_String,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_i32 {
        pub count: usize,
        pub values: *mut i32,
    }
    impl ferment_interfaces::FFIConversion<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(ffi: *mut Vec_i32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion for Vec_i32 {
        type Value = Vec<i32>;
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
    impl Drop for Vec_i32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(count: usize, values: *mut i32) -> *mut Vec_i32 {
        ferment_interfaces::boxed(Vec_i32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_32 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment_interfaces::FFIConversion<[u8; 32]> for Arr_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_32) -> [u8; 32] {
            let ffi_ref = &*ffi;
            std::slice::from_raw_parts(ffi_ref.values, ffi_ref.count)
                .try_into()
                .expect("Array Length mismatch")
        }
        unsafe fn ffi_to_const(obj: [u8; 32]) -> *const Arr_u8_32 {
            ferment_interfaces::boxed(Self {
                count: obj.len(),
                values: ferment_interfaces::boxed_vec(obj.to_vec()),
            })
        }
        unsafe fn destroy(ffi: *mut Arr_u8_32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Arr_u8_32 {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_32 {
        ferment_interfaces::boxed(Arr_u8_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_destroy(ffi: *mut Arr_u8_32) {
        ferment_interfaces::unbox_any(ffi);
    }
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
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_u32_err_u32 {
        pub ok: *mut u32,
        pub error: *mut u32,
    }
    impl ferment_interfaces::FFIConversion<Result<u32, u32>> for Result_ok_u32_err_u32 {
        unsafe fn ffi_from_const(ffi: *const Result_ok_u32_err_u32) -> Result<u32, u32> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(ffi_ref.ok, ffi_ref.error, |o| *o, |o| *o)
        }
        unsafe fn ffi_to_const(obj: Result<u32, u32>) -> *const Result_ok_u32_err_u32 {
            ferment_interfaces::boxed({
                let (ok, error) = match obj {
                    Ok(o) => (o as *mut _, std::ptr::null_mut()),
                    Err(o) => (std::ptr::null_mut(), o as *mut _),
                };
                Self { ok, error }
            })
        }
        unsafe fn destroy(ffi: *mut Result_ok_u32_err_u32) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_u32 {
        fn drop(&mut self) {
            unsafe {
                if !(self.ok).is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !(self.error).is_null() {
                    ferment_interfaces::unbox_any(self.error);
                };
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_ctor(
        ok: *mut u32,
        error: *mut u32,
    ) -> *mut Result_ok_u32_err_u32 {
        ferment_interfaces::boxed(Result_ok_u32_err_u32 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_destroy(ffi: *mut Result_ok_u32_err_u32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode }
    impl
        ferment_interfaces::FFIConversion<
            Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>,
        > for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
        ) -> Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode> {
            ferment_interfaces::FFIVecConversion::decode(&*ffi)
        }
        unsafe fn ffi_to_const(
            obj: Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>,
        ) -> *const Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
            ferment_interfaces::FFIVecConversion::encode(obj)
        }
        unsafe fn destroy(
            ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl ferment_interfaces::FFIVecConversion
        for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        type Value = Vec<ferment_example_nested::model::snapshot::LLMQSnapshotSkipMode>;
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
    impl Drop for Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        fn drop(&mut self) {
            unsafe {
                ferment_interfaces::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: ferment_example_nested :: model :: snapshot :: ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) -> *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        ferment_interfaces::boxed(
            Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode { count, values },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
        ffi: *mut Vec_ferment_example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
