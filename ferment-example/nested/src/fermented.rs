#[allow(
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::redundant_field_names,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    redundant_semicolons,
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod types {
    pub mod example_simple {
        use crate as example_nested;
        pub mod state_transition {
            use crate as example_nested;
            pub mod errors {
                use crate as example_nested;
                pub mod invalid_identity_public_key_type_error {
                    use crate as example_nested;
                    #[doc = "FFI-representation of the [`example_simple::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError
                    {
                        pub public_key_type: *mut std::os::raw::c_char,
                    }
                    impl ferment :: FFIConversionFrom < example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError > for example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError {
                        unsafe fn ffi_from_const (
                            ffi : * const example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError
                        ) -> example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError { let ffi_ref = & * ffi ;
                            example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError {
                                public_key_type : ferment :: FFIConversionFrom :: ffi_from (ffi_ref . public_key_type)
                            }
                        }
                    }
                    impl ferment :: FFIConversionTo < example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError > for example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError {
                        unsafe fn ffi_to_const (
                            obj : example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError
                        ) -> * const example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { ferment :: boxed (example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type : ferment :: FFIConversionTo :: ffi_to (obj . public_key_type) }) } }
                    impl ferment :: FFIConversionDestroy < example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: InvalidIdentityPublicKeyTypeError > for example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { unsafe fn destroy (ffi : * mut example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) { ferment :: unbox_any (ffi) ; } }
                    impl Drop for example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { fn drop (& mut self) { unsafe { let ffi_ref = self ; < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (ffi_ref . public_key_type) ; } } }
                    #[no_mangle]                    pub unsafe extern "C" fn example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_ctor < > (public_key_type : * mut std :: os :: raw :: c_char) -> * mut example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError{
                        ferment :: boxed (example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError { public_key_type })
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_destroy(
                        ffi : * mut example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_get_public_key_type(
                        obj : * const example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).public_key_type
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError_set_public_key_type(
                        obj : * const example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).public_key_type
                    }
                }
            }
        }
        #[doc = "FFI-representation of the [`example_simple::get_root_struct`]"]
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_get_root_struct(
        ) -> *mut crate::fermented::types::example_simple::example_simple_RootStruct {
            let obj = example_simple::get_root_struct();
            ferment::FFIConversionTo::ffi_to(obj)
        }
        pub mod nested {
            use crate as example_nested;
            #[doc = "FFI-representation of the [`example_simple::nested::get_root_struct_2`]"]
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_get_root_struct_2(
            ) -> *mut crate::fermented::types::example_simple::example_simple_RootStruct
            {
                let obj = example_simple::nested::get_root_struct_2();
                ferment::FFIConversionTo::ffi_to(obj)
            }
            #[doc = "FFI-representation of the [`example_simple::nested::RootUser`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct example_simple_nested_RootUser {
                pub root: *mut crate::fermented::types::example_simple::example_simple_RootStruct,
            }
            impl ferment::FFIConversionFrom<example_simple::nested::RootUser>
                for example_simple_nested_RootUser
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_simple_nested_RootUser,
                ) -> example_simple::nested::RootUser {
                    let ffi_ref = &*ffi;
                    example_simple::nested::RootUser {
                        root: ferment::FFIConversionFrom::ffi_from(ffi_ref.root),
                    }
                }
            }
            impl ferment::FFIConversionTo<example_simple::nested::RootUser> for example_simple_nested_RootUser {
                unsafe fn ffi_to_const(
                    obj: example_simple::nested::RootUser,
                ) -> *const example_simple_nested_RootUser {
                    ferment::boxed(example_simple_nested_RootUser {
                        root: ferment::FFIConversionTo::ffi_to(obj.root),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_simple::nested::RootUser>
                for example_simple_nested_RootUser
            {
                unsafe fn destroy(ffi: *mut example_simple_nested_RootUser) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_simple_nested_RootUser {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.root);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_RootUser_ctor(
                root: *mut crate::fermented::types::example_simple::example_simple_RootStruct,
            ) -> *mut example_simple_nested_RootUser {
                ferment::boxed(example_simple_nested_RootUser { root })
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_RootUser_destroy(
                ffi: *mut example_simple_nested_RootUser,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_RootUser_get_root(
                obj: *const example_simple_nested_RootUser,
            ) -> *mut crate::fermented::types::example_simple::example_simple_RootStruct
            {
                (*obj).root
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_RootUser_set_root(
                obj: *const example_simple_nested_RootUser,
            ) -> *mut crate::fermented::types::example_simple::example_simple_RootStruct
            {
                (*obj).root
            }
            #[doc = "FFI-representation of the [`example_simple::nested::HashID`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct example_simple_nested_HashID(*mut crate::fermented::generics::Arr_u8_32);
            impl ferment::FFIConversionFrom<example_simple::nested::HashID> for example_simple_nested_HashID {
                unsafe fn ffi_from_const(
                    ffi: *const example_simple_nested_HashID,
                ) -> example_simple::nested::HashID {
                    let ffi_ref = &*ffi;
                    ferment::FFIConversionFrom::ffi_from(ffi_ref.0)
                }
            }
            impl ferment::FFIConversionTo<example_simple::nested::HashID> for example_simple_nested_HashID {
                unsafe fn ffi_to_const(
                    obj: example_simple::nested::HashID,
                ) -> *const example_simple_nested_HashID {
                    ferment::boxed(example_simple_nested_HashID(
                        ferment::FFIConversionTo::ffi_to(obj),
                    ))
                }
            }
            impl ferment::FFIConversionDestroy<example_simple::nested::HashID>
                for example_simple_nested_HashID
            {
                unsafe fn destroy(ffi: *mut example_simple_nested_HashID) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_simple_nested_HashID {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.0);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_HashID_ctor(
                o_0: *mut crate::fermented::generics::Arr_u8_32,
            ) -> *mut example_simple_nested_HashID {
                ferment::boxed(example_simple_nested_HashID(o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_HashID_destroy(
                ffi: *mut example_simple_nested_HashID,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_HashID_get_0(
                obj: *const example_simple_nested_HashID,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).0
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_HashID_set_0(
                obj: *const example_simple_nested_HashID,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).0
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::nested::TestEnum`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_simple_nested_TestEnum {
                Variant1 (* mut std :: os :: raw :: c_char) , Variant2 , Variant3 (* mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID , u32) , Variant4 (* mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID , u32 , * mut std :: os :: raw :: c_char) , Variant5 (* mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_example_simple_nested_HashID , u32 , * mut std :: os :: raw :: c_char) , Variant6 (* mut crate :: fermented :: generics :: Arr_u8_32) }
            impl ferment::FFIConversionFrom<example_simple::nested::TestEnum>
                for example_simple_nested_TestEnum
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_simple_nested_TestEnum,
                ) -> example_simple::nested::TestEnum {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_simple_nested_TestEnum::Variant1(o_0) => {
                            example_simple::nested::TestEnum::Variant1(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                        example_simple_nested_TestEnum::Variant2 => {
                            example_simple::nested::TestEnum::Variant2
                        }
                        example_simple_nested_TestEnum::Variant3(o_0, o_1) => {
                            example_simple::nested::TestEnum::Variant3(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                                *o_1,
                            )
                        }
                        example_simple_nested_TestEnum::Variant4(o_0, o_1, o_2) => {
                            example_simple::nested::TestEnum::Variant4(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                                *o_1,
                                ferment::FFIConversionFrom::ffi_from(*o_2),
                            )
                        }
                        example_simple_nested_TestEnum::Variant5(o_0, o_1, o_2) => {
                            example_simple::nested::TestEnum::Variant5(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                                *o_1,
                                ferment::FFIConversionFrom::ffi_from(*o_2),
                            )
                        }
                        example_simple_nested_TestEnum::Variant6(o_0) => {
                            example_simple::nested::TestEnum::Variant6(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_simple::nested::TestEnum> for example_simple_nested_TestEnum {
                unsafe fn ffi_to_const(
                    obj: example_simple::nested::TestEnum,
                ) -> *const example_simple_nested_TestEnum {
                    ferment::boxed(match obj {
                        example_simple::nested::TestEnum::Variant1(o_0) => {
                            example_simple_nested_TestEnum::Variant1(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        example_simple::nested::TestEnum::Variant2 => {
                            example_simple_nested_TestEnum::Variant2
                        }
                        example_simple::nested::TestEnum::Variant3(o_0, o_1) => {
                            example_simple_nested_TestEnum::Variant3(
                                ferment::FFIConversionTo::ffi_to(o_0),
                                o_1,
                            )
                        }
                        example_simple::nested::TestEnum::Variant4(o_0, o_1, o_2) => {
                            example_simple_nested_TestEnum::Variant4(
                                ferment::FFIConversionTo::ffi_to(o_0),
                                o_1,
                                ferment::FFIConversionTo::ffi_to(o_2),
                            )
                        }
                        example_simple::nested::TestEnum::Variant5(o_0, o_1, o_2) => {
                            example_simple_nested_TestEnum::Variant5(
                                ferment::FFIConversionTo::ffi_to(o_0),
                                o_1,
                                ferment::FFIConversionTo::ffi_to(o_2),
                            )
                        }
                        example_simple::nested::TestEnum::Variant6(o_0) => {
                            example_simple_nested_TestEnum::Variant6(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_simple::nested::TestEnum>
                for example_simple_nested_TestEnum
            {
                unsafe fn destroy(ffi: *mut example_simple_nested_TestEnum) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_simple_nested_TestEnum {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_simple_nested_TestEnum::Variant1(o_0) => {
                                < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_0) ;
                            }
                            example_simple_nested_TestEnum::Variant2 => {}
                            example_simple_nested_TestEnum::Variant3(o_0, o_1) => {
                                ferment::unbox_any(*o_0);
                            }
                            example_simple_nested_TestEnum::Variant4(o_0, o_1, o_2) => {
                                ferment::unbox_any(*o_0);
                                < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_2) ;
                            }
                            example_simple_nested_TestEnum::Variant5(o_0, o_1, o_2) => {
                                ferment::unbox_any(*o_0);
                                < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_2) ;
                            }
                            example_simple_nested_TestEnum::Variant6(o_0) => {
                                ferment::unbox_any(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant1_ctor(
                o_o_0: *mut std::os::raw::c_char,
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant1(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant2_ctor(
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant2 {})
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant3_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID,
                o_o_1: u32,
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant3(o_o_0, o_o_1))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant4_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID,
                o_o_1: u32,
                o_o_2: *mut std::os::raw::c_char,
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant4(
                    o_o_0, o_o_1, o_o_2,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant5_ctor(
                o_o_0 : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_example_simple_nested_HashID,
                o_o_1: u32,
                o_o_2: *mut std::os::raw::c_char,
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant5(
                    o_o_0, o_o_1, o_o_2,
                ))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_Variant6_ctor(
                o_o_0: *mut crate::fermented::generics::Arr_u8_32,
            ) -> *mut example_simple_nested_TestEnum {
                ferment::boxed(example_simple_nested_TestEnum::Variant6(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_nested_TestEnum_destroy(
                ffi: *mut example_simple_nested_TestEnum,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod double_nested {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_simple::nested::double_nested::get_root_struct_3`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_nested_double_nested_get_root_struct_3(
                ) -> *mut crate::fermented::types::example_simple::example_simple_RootStruct
                {
                    let obj = example_simple::nested::double_nested::get_root_struct_3();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
            }
        }
        #[doc = "FFI-representation of the [`example_simple::Unstable`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct example_simple_Unstable {
            pub secs: *mut u128,
            pub nanos: *mut i128,
        }
        impl ferment::FFIConversionFrom<example_simple::Unstable> for example_simple_Unstable {
            unsafe fn ffi_from_const(
                ffi: *const example_simple_Unstable,
            ) -> example_simple::Unstable {
                let ffi_ref = &*ffi;
                example_simple::Unstable {
                    secs: (&*ffi_ref.secs).clone(),
                    nanos: (&*ffi_ref.nanos).clone(),
                }
            }
        }
        impl ferment::FFIConversionTo<example_simple::Unstable> for example_simple_Unstable {
            unsafe fn ffi_to_const(
                obj: example_simple::Unstable,
            ) -> *const example_simple_Unstable {
                ferment::boxed(example_simple_Unstable {
                    secs: ferment::boxed(obj.secs),
                    nanos: ferment::boxed(obj.nanos),
                })
            }
        }
        impl ferment::FFIConversionDestroy<example_simple::Unstable> for example_simple_Unstable {
            unsafe fn destroy(ffi: *mut example_simple_Unstable) {
                ferment::unbox_any(ffi);
            }
        }
        impl Drop for example_simple_Unstable {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    ferment::unbox_any(ffi_ref.secs);
                    ferment::unbox_any(ffi_ref.nanos);
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_ctor(
            secs: *mut u128,
            nanos: *mut i128,
        ) -> *mut example_simple_Unstable {
            ferment::boxed(example_simple_Unstable { secs, nanos })
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_destroy(
            ffi: *mut example_simple_Unstable,
        ) {
            ferment::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_get_secs(
            obj: *const example_simple_Unstable,
        ) -> *mut u128 {
            (*obj).secs
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_get_nanos(
            obj: *const example_simple_Unstable,
        ) -> *mut i128 {
            (*obj).nanos
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_set_secs(
            obj: *const example_simple_Unstable,
        ) -> *mut u128 {
            (*obj).secs
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_Unstable_set_nanos(
            obj: *const example_simple_Unstable,
        ) -> *mut i128 {
            (*obj).nanos
        }
        #[doc = "FFI-representation of the [`example_simple::RootStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct example_simple_RootStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment::FFIConversionFrom<example_simple::RootStruct> for example_simple_RootStruct {
            unsafe fn ffi_from_const(
                ffi: *const example_simple_RootStruct,
            ) -> example_simple::RootStruct {
                let ffi_ref = &*ffi;
                example_simple::RootStruct {
                    name: ferment::FFIConversionFrom::ffi_from(ffi_ref.name),
                }
            }
        }
        impl ferment::FFIConversionTo<example_simple::RootStruct> for example_simple_RootStruct {
            unsafe fn ffi_to_const(
                obj: example_simple::RootStruct,
            ) -> *const example_simple_RootStruct {
                ferment::boxed(example_simple_RootStruct {
                    name: ferment::FFIConversionTo::ffi_to(obj.name),
                })
            }
        }
        impl ferment::FFIConversionDestroy<example_simple::RootStruct> for example_simple_RootStruct {
            unsafe fn destroy(ffi: *mut example_simple_RootStruct) {
                ferment::unbox_any(ffi);
            }
        }
        impl Drop for example_simple_RootStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment::FFIConversionDestroy<String>>::destroy(
                        ffi_ref.name,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_RootStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut example_simple_RootStruct {
            ferment::boxed(example_simple_RootStruct { name })
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_RootStruct_destroy(
            ffi: *mut example_simple_RootStruct,
        ) {
            ferment::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_RootStruct_get_name(
            obj: *const example_simple_RootStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_simple_RootStruct_set_name(
            obj: *const example_simple_RootStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        pub mod example {
            use crate as example_nested;
            pub mod address {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_simple::example::address::address_simple_result`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_address_address_simple_result(
                    script: *mut crate::fermented::generics::Vec_u32,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    let obj = example_simple::example::address::address_simple_result(
                        ferment::FFIConversionFrom::ffi_from(script),
                    );
                    ferment::FFIConversionTo::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`example_simple::example::address::address_with_script_pubkey`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_address_address_with_script_pubkey(
                    script: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut std::os::raw::c_char {
                    let obj = example_simple::example::address::address_with_script_pubkey(
                        ferment::FFIConversionFrom::ffi_from(script),
                    );
                    ferment::FFIConversionTo::ffi_to_opt(obj)
                }
            }
            pub mod custom_conversion {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_simple::example::custom_conversion::StructUsesDurationTuple`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_simple_example_custom_conversion_StructUsesDurationTuple {
                    pub time:
                        *mut crate::fermented::generics::Tuple_std_time_Duration_std_time_Duration,
                }
                impl
                    ferment::FFIConversionFrom<
                        example_simple::example::custom_conversion::StructUsesDurationTuple,
                    > for example_simple_example_custom_conversion_StructUsesDurationTuple
                {
                    unsafe fn ffi_from_const(
                        ffi : * const example_simple_example_custom_conversion_StructUsesDurationTuple,
                    ) -> example_simple::example::custom_conversion::StructUsesDurationTuple
                    {
                        let ffi_ref = &*ffi;
                        example_simple::example::custom_conversion::StructUsesDurationTuple {
                            time: ferment::FFIConversionFrom::ffi_from(ffi_ref.time),
                        }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        example_simple::example::custom_conversion::StructUsesDurationTuple,
                    > for example_simple_example_custom_conversion_StructUsesDurationTuple
                {
                    unsafe fn ffi_to_const(
                        obj: example_simple::example::custom_conversion::StructUsesDurationTuple,
                    ) -> *const example_simple_example_custom_conversion_StructUsesDurationTuple
                    {
                        ferment::boxed(
                            example_simple_example_custom_conversion_StructUsesDurationTuple {
                                time: ferment::FFIConversionTo::ffi_to(obj.time),
                            },
                        )
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_simple::example::custom_conversion::StructUsesDurationTuple,
                    > for example_simple_example_custom_conversion_StructUsesDurationTuple
                {
                    unsafe fn destroy(
                        ffi: *mut example_simple_example_custom_conversion_StructUsesDurationTuple,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_example_custom_conversion_StructUsesDurationTuple {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.time);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesDurationTuple_ctor(
                    time : * mut crate :: fermented :: generics :: Tuple_std_time_Duration_std_time_Duration,
                ) -> *mut example_simple_example_custom_conversion_StructUsesDurationTuple
                {
                    ferment::boxed(
                        example_simple_example_custom_conversion_StructUsesDurationTuple { time },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesDurationTuple_destroy(
                    ffi: *mut example_simple_example_custom_conversion_StructUsesDurationTuple,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesDurationTuple_get_time(
                    obj: *const example_simple_example_custom_conversion_StructUsesDurationTuple,
                ) -> *mut crate::fermented::generics::Tuple_std_time_Duration_std_time_Duration
                {
                    (*obj).time
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesDurationTuple_set_time(
                    obj: *const example_simple_example_custom_conversion_StructUsesDurationTuple,
                ) -> *mut crate::fermented::generics::Tuple_std_time_Duration_std_time_Duration
                {
                    (*obj).time
                }
                #[doc = "FFI-representation of the [`example_simple::example::custom_conversion::StructUsesGenericWithCustom`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_simple_example_custom_conversion_StructUsesGenericWithCustom { pub time : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration }
                impl
                    ferment::FFIConversionFrom<
                        example_simple::example::custom_conversion::StructUsesGenericWithCustom,
                    > for example_simple_example_custom_conversion_StructUsesGenericWithCustom
                {
                    unsafe fn ffi_from_const(
                        ffi : * const example_simple_example_custom_conversion_StructUsesGenericWithCustom,
                    ) -> example_simple::example::custom_conversion::StructUsesGenericWithCustom
                    {
                        let ffi_ref = &*ffi;
                        example_simple::example::custom_conversion::StructUsesGenericWithCustom {
                            time: ferment::FFIConversionFrom::ffi_from(ffi_ref.time),
                        }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        example_simple::example::custom_conversion::StructUsesGenericWithCustom,
                    > for example_simple_example_custom_conversion_StructUsesGenericWithCustom
                {
                    unsafe fn ffi_to_const(
                        obj : example_simple :: example :: custom_conversion :: StructUsesGenericWithCustom,
                    ) -> *const example_simple_example_custom_conversion_StructUsesGenericWithCustom
                    {
                        ferment::boxed(
                            example_simple_example_custom_conversion_StructUsesGenericWithCustom {
                                time: ferment::FFIConversionTo::ffi_to(obj.time),
                            },
                        )
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_simple::example::custom_conversion::StructUsesGenericWithCustom,
                    > for example_simple_example_custom_conversion_StructUsesGenericWithCustom
                {
                    unsafe fn destroy(
                        ffi : * mut example_simple_example_custom_conversion_StructUsesGenericWithCustom,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_example_custom_conversion_StructUsesGenericWithCustom {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.time);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesGenericWithCustom_ctor(
                    time : * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration,
                ) -> *mut example_simple_example_custom_conversion_StructUsesGenericWithCustom
                {
                    ferment::boxed(
                        example_simple_example_custom_conversion_StructUsesGenericWithCustom {
                            time,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesGenericWithCustom_destroy(
                    ffi: *mut example_simple_example_custom_conversion_StructUsesGenericWithCustom,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesGenericWithCustom_get_time < > (obj : * const example_simple_example_custom_conversion_StructUsesGenericWithCustom) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration{
                    (*obj).time
                }
                #[no_mangle]                pub unsafe extern "C" fn example_simple_example_custom_conversion_StructUsesGenericWithCustom_set_time < > (obj : * const example_simple_example_custom_conversion_StructUsesGenericWithCustom) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_String_values_std_time_Duration{
                    (*obj).time
                }
            }
        }
        pub mod document {
            use crate as example_nested;
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::document::Document`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_simple_document_Document {
                V0,
            }
            impl ferment::FFIConversionFrom<example_simple::document::Document>
                for example_simple_document_Document
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_simple_document_Document,
                ) -> example_simple::document::Document {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_simple_document_Document::V0 => {
                            example_simple::document::Document::V0
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_simple::document::Document>
                for example_simple_document_Document
            {
                unsafe fn ffi_to_const(
                    obj: example_simple::document::Document,
                ) -> *const example_simple_document_Document {
                    ferment::boxed(match obj {
                        example_simple::document::Document::V0 => {
                            example_simple_document_Document::V0
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_simple::document::Document>
                for example_simple_document_Document
            {
                unsafe fn destroy(ffi: *mut example_simple_document_Document) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_simple_document_Document {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_simple_document_Document::V0 => {}
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_document_Document_V0_ctor(
            ) -> *mut example_simple_document_Document {
                ferment::boxed(example_simple_document_Document::V0 {})
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_document_Document_destroy(
                ffi: *mut example_simple_document_Document,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod errors {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::document::errors::DocumentError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum example_simple_document_errors_DocumentError {
                    InvalidActionError (u8) , InvalidInitialRevisionError { document : * mut crate :: fermented :: types :: example_simple :: document :: example_simple_document_Document } }
                impl ferment::FFIConversionFrom<example_simple::document::errors::DocumentError>
                    for example_simple_document_errors_DocumentError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_simple_document_errors_DocumentError,
                    ) -> example_simple::document::errors::DocumentError {
                        let ffi_ref = &*ffi;
                        match ffi_ref { example_simple_document_errors_DocumentError :: InvalidActionError (o_0) => example_simple :: document :: errors :: DocumentError :: InvalidActionError (* o_0) , example_simple_document_errors_DocumentError :: InvalidInitialRevisionError { document } => example_simple :: document :: errors :: DocumentError :: InvalidInitialRevisionError { document : Box :: new (ferment :: FFIConversionFrom :: ffi_from (* document)) } }
                    }
                }
                impl ferment::FFIConversionTo<example_simple::document::errors::DocumentError>
                    for example_simple_document_errors_DocumentError
                {
                    unsafe fn ffi_to_const(
                        obj: example_simple::document::errors::DocumentError,
                    ) -> *const example_simple_document_errors_DocumentError {
                        ferment :: boxed (match obj { example_simple :: document :: errors :: DocumentError :: InvalidActionError (o_0) => example_simple_document_errors_DocumentError :: InvalidActionError (o_0) , example_simple :: document :: errors :: DocumentError :: InvalidInitialRevisionError { document } => example_simple_document_errors_DocumentError :: InvalidInitialRevisionError { document : ferment :: FFIConversionTo :: ffi_to (* document) } , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl ferment::FFIConversionDestroy<example_simple::document::errors::DocumentError>
                    for example_simple_document_errors_DocumentError
                {
                    unsafe fn destroy(ffi: *mut example_simple_document_errors_DocumentError) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_document_errors_DocumentError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { example_simple_document_errors_DocumentError :: InvalidActionError (o_0) => { ; } , example_simple_document_errors_DocumentError :: InvalidInitialRevisionError { document } => { ferment :: unbox_any (* document) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_document_errors_DocumentError_InvalidActionError_ctor(
                    o_o_0: u8,
                ) -> *mut example_simple_document_errors_DocumentError {
                    ferment::boxed(
                        example_simple_document_errors_DocumentError::InvalidActionError(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_document_errors_DocumentError_InvalidInitialRevisionError_ctor(
                    document : * mut crate :: fermented :: types :: example_simple :: document :: example_simple_document_Document,
                ) -> *mut example_simple_document_errors_DocumentError {
                    ferment::boxed(
                        example_simple_document_errors_DocumentError::InvalidInitialRevisionError {
                            document,
                        },
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_document_errors_DocumentError_destroy(
                    ffi: *mut example_simple_document_errors_DocumentError,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
        }
        pub mod errors {
            use crate as example_nested;
            pub mod protocol_error {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::errors::protocol_error::ProtocolError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum example_simple_errors_protocol_error_ProtocolError {
                    InvalidPKT (* mut crate :: fermented :: types :: example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError) }
                impl
                    ferment::FFIConversionFrom<
                        example_simple::errors::protocol_error::ProtocolError,
                    > for example_simple_errors_protocol_error_ProtocolError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_simple_errors_protocol_error_ProtocolError,
                    ) -> example_simple::errors::protocol_error::ProtocolError {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            example_simple_errors_protocol_error_ProtocolError::InvalidPKT(o_0) => {
                                example_simple::errors::protocol_error::ProtocolError::InvalidPKT(
                                    ferment::FFIConversionFrom::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_simple::errors::protocol_error::ProtocolError>
                    for example_simple_errors_protocol_error_ProtocolError
                {
                    unsafe fn ffi_to_const(
                        obj: example_simple::errors::protocol_error::ProtocolError,
                    ) -> *const example_simple_errors_protocol_error_ProtocolError
                    {
                        ferment::boxed(match obj {
                            example_simple::errors::protocol_error::ProtocolError::InvalidPKT(
                                o_0,
                            ) => example_simple_errors_protocol_error_ProtocolError::InvalidPKT(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            ),
                            _ => unreachable!("This is unreachable"),
                        })
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_simple::errors::protocol_error::ProtocolError,
                    > for example_simple_errors_protocol_error_ProtocolError
                {
                    unsafe fn destroy(
                        ffi: *mut example_simple_errors_protocol_error_ProtocolError,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_errors_protocol_error_ProtocolError {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                example_simple_errors_protocol_error_ProtocolError::InvalidPKT(
                                    o_0,
                                ) => {
                                    ferment::unbox_any(*o_0);
                                }
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_protocol_error_ProtocolError_InvalidPKT_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: example_simple :: state_transition :: errors :: invalid_identity_public_key_type_error :: example_simple_state_transition_errors_invalid_identity_public_key_type_error_InvalidIdentityPublicKeyTypeError,
                ) -> *mut example_simple_errors_protocol_error_ProtocolError {
                    ferment::boxed(
                        example_simple_errors_protocol_error_ProtocolError::InvalidPKT(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_protocol_error_ProtocolError_destroy(
                    ffi: *mut example_simple_errors_protocol_error_ProtocolError,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
            pub mod context {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::errors::context::ContextProviderError`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum example_simple_errors_context_ContextProviderError {
                    Generic(*mut std::os::raw::c_char),
                    Config(*mut std::os::raw::c_char),
                    InvalidDataContract(*mut std::os::raw::c_char),
                    InvalidQuorum(*mut std::os::raw::c_char),
                }
                impl
                    ferment::FFIConversionFrom<
                        example_simple::errors::context::ContextProviderError,
                    > for example_simple_errors_context_ContextProviderError
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_simple_errors_context_ContextProviderError,
                    ) -> example_simple::errors::context::ContextProviderError {
                        let ffi_ref = &*ffi;
                        match ffi_ref { example_simple_errors_context_ContextProviderError :: Generic (o_0) => example_simple :: errors :: context :: ContextProviderError :: Generic (ferment :: FFIConversionFrom :: ffi_from (* o_0)) , example_simple_errors_context_ContextProviderError :: Config (o_0) => example_simple :: errors :: context :: ContextProviderError :: Config (ferment :: FFIConversionFrom :: ffi_from (* o_0)) , example_simple_errors_context_ContextProviderError :: InvalidDataContract (o_0) => example_simple :: errors :: context :: ContextProviderError :: InvalidDataContract (ferment :: FFIConversionFrom :: ffi_from (* o_0)) , example_simple_errors_context_ContextProviderError :: InvalidQuorum (o_0) => example_simple :: errors :: context :: ContextProviderError :: InvalidQuorum (ferment :: FFIConversionFrom :: ffi_from (* o_0)) }
                    }
                }
                impl ferment::FFIConversionTo<example_simple::errors::context::ContextProviderError>
                    for example_simple_errors_context_ContextProviderError
                {
                    unsafe fn ffi_to_const(
                        obj: example_simple::errors::context::ContextProviderError,
                    ) -> *const example_simple_errors_context_ContextProviderError
                    {
                        ferment :: boxed (match obj { example_simple :: errors :: context :: ContextProviderError :: Generic (o_0) => example_simple_errors_context_ContextProviderError :: Generic (ferment :: FFIConversionTo :: ffi_to (o_0)) , example_simple :: errors :: context :: ContextProviderError :: Config (o_0) => example_simple_errors_context_ContextProviderError :: Config (ferment :: FFIConversionTo :: ffi_to (o_0)) , example_simple :: errors :: context :: ContextProviderError :: InvalidDataContract (o_0) => example_simple_errors_context_ContextProviderError :: InvalidDataContract (ferment :: FFIConversionTo :: ffi_to (o_0)) , example_simple :: errors :: context :: ContextProviderError :: InvalidQuorum (o_0) => example_simple_errors_context_ContextProviderError :: InvalidQuorum (ferment :: FFIConversionTo :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") })
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_simple::errors::context::ContextProviderError,
                    > for example_simple_errors_context_ContextProviderError
                {
                    unsafe fn destroy(
                        ffi: *mut example_simple_errors_context_ContextProviderError,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_errors_context_ContextProviderError {
                    fn drop(&mut self) {
                        unsafe {
                            match self { example_simple_errors_context_ContextProviderError :: Generic (o_0) => { < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_0) ; } , example_simple_errors_context_ContextProviderError :: Config (o_0) => { < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_0) ; } , example_simple_errors_context_ContextProviderError :: InvalidDataContract (o_0) => { < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_0) ; } , example_simple_errors_context_ContextProviderError :: InvalidQuorum (o_0) => { < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ;
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_context_ContextProviderError_Generic_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut example_simple_errors_context_ContextProviderError {
                    ferment::boxed(example_simple_errors_context_ContextProviderError::Generic(
                        o_o_0,
                    ))
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_context_ContextProviderError_Config_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut example_simple_errors_context_ContextProviderError {
                    ferment::boxed(example_simple_errors_context_ContextProviderError::Config(
                        o_o_0,
                    ))
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_context_ContextProviderError_InvalidDataContract_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut example_simple_errors_context_ContextProviderError {
                    ferment::boxed(
                        example_simple_errors_context_ContextProviderError::InvalidDataContract(
                            o_o_0,
                        ),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_context_ContextProviderError_InvalidQuorum_ctor(
                    o_o_0: *mut std::os::raw::c_char,
                ) -> *mut example_simple_errors_context_ContextProviderError {
                    ferment::boxed(
                        example_simple_errors_context_ContextProviderError::InvalidQuorum(o_o_0),
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_errors_context_ContextProviderError_destroy(
                    ffi: *mut example_simple_errors_context_ContextProviderError,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
        }
        pub mod data_contract {
            use crate as example_nested;
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::data_contract::DataContract`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_simple_data_contract_DataContract {
                V0 (* mut crate :: fermented :: types :: example_simple :: data_contract :: v0 :: data_contract :: example_simple_data_contract_v0_data_contract_DataContractV0) , V1 (* mut crate :: fermented :: types :: example_simple :: data_contract :: v1 :: data_contract :: example_simple_data_contract_v1_data_contract_DataContractV1) , # [cfg (test)] Test }
            impl ferment::FFIConversionFrom<example_simple::data_contract::DataContract>
                for example_simple_data_contract_DataContract
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_simple_data_contract_DataContract,
                ) -> example_simple::data_contract::DataContract {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_simple_data_contract_DataContract::V0(o_0) => {
                            example_simple::data_contract::DataContract::V0(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                        example_simple_data_contract_DataContract::V1(o_0) => {
                            example_simple::data_contract::DataContract::V1(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                        #[cfg(test)]
                        example_simple_data_contract_DataContract::Test => {
                            example_simple::data_contract::DataContract::Test
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_simple::data_contract::DataContract>
                for example_simple_data_contract_DataContract
            {
                unsafe fn ffi_to_const(
                    obj: example_simple::data_contract::DataContract,
                ) -> *const example_simple_data_contract_DataContract {
                    ferment::boxed(match obj {
                        example_simple::data_contract::DataContract::V0(o_0) => {
                            example_simple_data_contract_DataContract::V0(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        example_simple::data_contract::DataContract::V1(o_0) => {
                            example_simple_data_contract_DataContract::V1(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        #[cfg(test)]
                        example_simple::data_contract::DataContract::Test => {
                            example_simple_data_contract_DataContract::Test
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_simple::data_contract::DataContract>
                for example_simple_data_contract_DataContract
            {
                unsafe fn destroy(ffi: *mut example_simple_data_contract_DataContract) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_simple_data_contract_DataContract {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_simple_data_contract_DataContract::V0(o_0) => {
                                ferment::unbox_any(*o_0);
                            }
                            example_simple_data_contract_DataContract::V1(o_0) => {
                                ferment::unbox_any(*o_0);
                            }
                            #[cfg(test)]
                            example_simple_data_contract_DataContract::Test => {}
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_data_contract_DataContract_V0_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_simple :: data_contract :: v0 :: data_contract :: example_simple_data_contract_v0_data_contract_DataContractV0,
            ) -> *mut example_simple_data_contract_DataContract {
                ferment::boxed(example_simple_data_contract_DataContract::V0(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_data_contract_DataContract_V1_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_simple :: data_contract :: v1 :: data_contract :: example_simple_data_contract_v1_data_contract_DataContractV1,
            ) -> *mut example_simple_data_contract_DataContract {
                ferment::boxed(example_simple_data_contract_DataContract::V1(o_o_0))
            }
            #[cfg(test)]
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_data_contract_DataContract_Test_ctor(
            ) -> *mut example_simple_data_contract_DataContract {
                ferment::boxed(example_simple_data_contract_DataContract::Test {})
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_simple_data_contract_DataContract_destroy(
                ffi: *mut example_simple_data_contract_DataContract,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod v0 {
                use crate as example_nested;
                pub mod data_contract {
                    use crate as example_nested;
                    #[doc = "FFI-representation of the [`example_simple::data_contract::v0::data_contract::DataContractV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_simple_data_contract_v0_data_contract_DataContractV0 {}
                    impl
                        ferment::FFIConversionFrom<
                            example_simple::data_contract::v0::data_contract::DataContractV0,
                        > for example_simple_data_contract_v0_data_contract_DataContractV0
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const example_simple_data_contract_v0_data_contract_DataContractV0,
                        ) -> example_simple::data_contract::v0::data_contract::DataContractV0
                        {
                            let ffi_ref = &*ffi;
                            example_simple::data_contract::v0::data_contract::DataContractV0 {}
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            example_simple::data_contract::v0::data_contract::DataContractV0,
                        > for example_simple_data_contract_v0_data_contract_DataContractV0
                    {
                        unsafe fn ffi_to_const(
                            obj: example_simple::data_contract::v0::data_contract::DataContractV0,
                        ) -> *const example_simple_data_contract_v0_data_contract_DataContractV0
                        {
                            ferment::boxed(
                                example_simple_data_contract_v0_data_contract_DataContractV0 {},
                            )
                        }
                    }
                    impl
                        ferment::FFIConversionDestroy<
                            example_simple::data_contract::v0::data_contract::DataContractV0,
                        > for example_simple_data_contract_v0_data_contract_DataContractV0
                    {
                        unsafe fn destroy(
                            ffi: *mut example_simple_data_contract_v0_data_contract_DataContractV0,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                    }
                    impl Drop for example_simple_data_contract_v0_data_contract_DataContractV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_v0_data_contract_DataContractV0_ctor(
                    ) -> *mut example_simple_data_contract_v0_data_contract_DataContractV0
                    {
                        ferment::boxed(
                            example_simple_data_contract_v0_data_contract_DataContractV0 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_v0_data_contract_DataContractV0_destroy(
                        ffi: *mut example_simple_data_contract_v0_data_contract_DataContractV0,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
            }
            pub mod v1 {
                use crate as example_nested;
                pub mod data_contract {
                    use crate as example_nested;
                    #[doc = "FFI-representation of the [`example_simple::data_contract::v1::data_contract::DataContractV1`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_simple_data_contract_v1_data_contract_DataContractV1 {}
                    impl
                        ferment::FFIConversionFrom<
                            example_simple::data_contract::v1::data_contract::DataContractV1,
                        > for example_simple_data_contract_v1_data_contract_DataContractV1
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const example_simple_data_contract_v1_data_contract_DataContractV1,
                        ) -> example_simple::data_contract::v1::data_contract::DataContractV1
                        {
                            let ffi_ref = &*ffi;
                            example_simple::data_contract::v1::data_contract::DataContractV1 {}
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            example_simple::data_contract::v1::data_contract::DataContractV1,
                        > for example_simple_data_contract_v1_data_contract_DataContractV1
                    {
                        unsafe fn ffi_to_const(
                            obj: example_simple::data_contract::v1::data_contract::DataContractV1,
                        ) -> *const example_simple_data_contract_v1_data_contract_DataContractV1
                        {
                            ferment::boxed(
                                example_simple_data_contract_v1_data_contract_DataContractV1 {},
                            )
                        }
                    }
                    impl
                        ferment::FFIConversionDestroy<
                            example_simple::data_contract::v1::data_contract::DataContractV1,
                        > for example_simple_data_contract_v1_data_contract_DataContractV1
                    {
                        unsafe fn destroy(
                            ffi: *mut example_simple_data_contract_v1_data_contract_DataContractV1,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                    }
                    impl Drop for example_simple_data_contract_v1_data_contract_DataContractV1 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_v1_data_contract_DataContractV1_ctor(
                    ) -> *mut example_simple_data_contract_v1_data_contract_DataContractV1
                    {
                        ferment::boxed(
                            example_simple_data_contract_v1_data_contract_DataContractV1 {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_v1_data_contract_DataContractV1_destroy(
                        ffi: *mut example_simple_data_contract_v1_data_contract_DataContractV1,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
            }
            pub mod document_type {
                use crate as example_nested;
                pub mod v0 {
                    use crate as example_nested;
                    #[doc = "FFI-representation of the [`example_simple::data_contract::document_type::v0::DocumentTypeV0`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    pub struct example_simple_data_contract_document_type_v0_DocumentTypeV0 {
                        pub name: *mut std::os::raw::c_char,
                        pub binary_paths:
                            *mut crate::fermented::generics::std_collections_BTreeSet_String,
                    }
                    impl
                        ferment::FFIConversionFrom<
                            example_simple::data_contract::document_type::v0::DocumentTypeV0,
                        > for example_simple_data_contract_document_type_v0_DocumentTypeV0
                    {
                        unsafe fn ffi_from_const(
                            ffi : * const example_simple_data_contract_document_type_v0_DocumentTypeV0,
                        ) -> example_simple::data_contract::document_type::v0::DocumentTypeV0
                        {
                            let ffi_ref = &*ffi;
                            example_simple::data_contract::document_type::v0::DocumentTypeV0 {
                                name: ferment::FFIConversionFrom::ffi_from(ffi_ref.name),
                                binary_paths: ferment::FFIConversionFrom::ffi_from(
                                    ffi_ref.binary_paths,
                                ),
                            }
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            example_simple::data_contract::document_type::v0::DocumentTypeV0,
                        > for example_simple_data_contract_document_type_v0_DocumentTypeV0
                    {
                        unsafe fn ffi_to_const(
                            obj: example_simple::data_contract::document_type::v0::DocumentTypeV0,
                        ) -> *const example_simple_data_contract_document_type_v0_DocumentTypeV0
                        {
                            ferment::boxed(
                                example_simple_data_contract_document_type_v0_DocumentTypeV0 {
                                    name: ferment::FFIConversionTo::ffi_to(obj.name),
                                    binary_paths: ferment::FFIConversionTo::ffi_to(
                                        obj.binary_paths,
                                    ),
                                },
                            )
                        }
                    }
                    impl
                        ferment::FFIConversionDestroy<
                            example_simple::data_contract::document_type::v0::DocumentTypeV0,
                        > for example_simple_data_contract_document_type_v0_DocumentTypeV0
                    {
                        unsafe fn destroy(
                            ffi: *mut example_simple_data_contract_document_type_v0_DocumentTypeV0,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                    }
                    impl Drop for example_simple_data_contract_document_type_v0_DocumentTypeV0 {
                        fn drop(&mut self) {
                            unsafe {
                                let ffi_ref = self;
                                < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (ffi_ref . name) ;
                                ferment::unbox_any(ffi_ref.binary_paths);
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_ctor(
                        name: *mut std::os::raw::c_char,
                        binary_paths : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String,
                    ) -> *mut example_simple_data_contract_document_type_v0_DocumentTypeV0
                    {
                        ferment::boxed(
                            example_simple_data_contract_document_type_v0_DocumentTypeV0 {
                                name,
                                binary_paths,
                            },
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_destroy(
                        ffi: *mut example_simple_data_contract_document_type_v0_DocumentTypeV0,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_get_name(
                        obj: *const example_simple_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).name
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_get_binary_paths(
                        obj: *const example_simple_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                    {
                        (*obj).binary_paths
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_set_name(
                        obj: *const example_simple_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut std::os::raw::c_char {
                        (*obj).name
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_simple_data_contract_document_type_v0_DocumentTypeV0_set_binary_paths(
                        obj: *const example_simple_data_contract_document_type_v0_DocumentTypeV0,
                    ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                    {
                        (*obj).binary_paths
                    }
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_simple::data_contract::document_type::DocumentType`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum example_simple_data_contract_document_type_DocumentType {
                    V0 (* mut crate :: fermented :: types :: example_simple :: data_contract :: document_type :: v0 :: example_simple_data_contract_document_type_v0_DocumentTypeV0) }
                impl
                    ferment::FFIConversionFrom<
                        example_simple::data_contract::document_type::DocumentType,
                    > for example_simple_data_contract_document_type_DocumentType
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_simple_data_contract_document_type_DocumentType,
                    ) -> example_simple::data_contract::document_type::DocumentType
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            example_simple_data_contract_document_type_DocumentType::V0(o_0) => {
                                example_simple::data_contract::document_type::DocumentType::V0(
                                    ferment::FFIConversionFrom::ffi_from(*o_0),
                                )
                            }
                        }
                    }
                }
                impl
                    ferment::FFIConversionTo<
                        example_simple::data_contract::document_type::DocumentType,
                    > for example_simple_data_contract_document_type_DocumentType
                {
                    unsafe fn ffi_to_const(
                        obj: example_simple::data_contract::document_type::DocumentType,
                    ) -> *const example_simple_data_contract_document_type_DocumentType
                    {
                        ferment::boxed(match obj {
                            example_simple::data_contract::document_type::DocumentType::V0(o_0) => {
                                example_simple_data_contract_document_type_DocumentType::V0(
                                    ferment::FFIConversionTo::ffi_to(o_0),
                                )
                            }
                            _ => unreachable!("This is unreachable"),
                        })
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_simple::data_contract::document_type::DocumentType,
                    > for example_simple_data_contract_document_type_DocumentType
                {
                    unsafe fn destroy(
                        ffi: *mut example_simple_data_contract_document_type_DocumentType,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_simple_data_contract_document_type_DocumentType {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                example_simple_data_contract_document_type_DocumentType::V0(
                                    o_0,
                                ) => {
                                    ferment::unbox_any(*o_0);
                                }
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_data_contract_document_type_DocumentType_V0_ctor(
                    o_o_0 : * mut crate :: fermented :: types :: example_simple :: data_contract :: document_type :: v0 :: example_simple_data_contract_document_type_v0_DocumentTypeV0,
                ) -> *mut example_simple_data_contract_document_type_DocumentType {
                    ferment::boxed(example_simple_data_contract_document_type_DocumentType::V0(
                        o_o_0,
                    ))
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_simple_data_contract_document_type_DocumentType_destroy(
                    ffi: *mut example_simple_data_contract_document_type_DocumentType,
                ) {
                    ferment::unbox_any(ffi);
                }
            }
        }
    }
    pub mod example_nested {
        use crate as example_nested;
        pub mod state_transition {
            use crate as example_nested;
            pub mod state_transitions {
                use crate as example_nested;
                pub mod contract {
                    use crate as example_nested;
                    pub mod data_contract_create_transition {
                        use crate as example_nested;
                        pub mod v0 {
                            use crate as example_nested;
                            #[doc = "FFI-representation of the [`example_nested::state_transition::state_transitions::contract::data_contract_create_transition::v0::DataContractCreateTransitionV0`]"]
                            #[repr(C)]
                            #[derive(Clone)]
                            pub struct example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0
                            {}
                            impl ferment :: FFIConversionFrom < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0 > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { unsafe fn ffi_from_const (ffi : * const example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0) -> example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0 { let ffi_ref = & * ffi ; example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0 { } } }
                            impl ferment :: FFIConversionTo < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0 > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { unsafe fn ffi_to_const (obj : example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0) -> * const example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { ferment :: boxed (example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { }) } }
                            impl ferment :: FFIConversionDestroy < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: DataContractCreateTransitionV0 > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { unsafe fn destroy (ffi : * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0) { ferment :: unbox_any (ffi) ; } }
                            impl Drop for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { fn drop (& mut self) { unsafe { let ffi_ref = self ; } } }
                            #[no_mangle]                            pub unsafe extern "C" fn example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0_ctor < > () -> * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0{
                                ferment :: boxed (example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0 { })
                            }
                            #[no_mangle]
                            pub unsafe extern "C" fn example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0_destroy(
                                ffi : * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0,
                            ) {
                                ferment::unbox_any(ffi);
                            }
                        }
                        #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::state_transition::state_transitions::contract::data_contract_create_transition::DataContractCreateTransition`]\"`]"]
                        #[repr(C)]
                        #[derive(Clone)]
                        #[non_exhaustive]
                        pub enum example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition
                        {
                            V0 (* mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0) }
                        impl ferment :: FFIConversionFrom < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition { unsafe fn ffi_from_const (ffi : * const example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition) -> example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition { let ffi_ref = & * ffi ; match ffi_ref { example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition :: V0 (o_0) => example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition :: V0 (ferment :: FFIConversionFrom :: ffi_from (* o_0)) } } }
                        impl ferment :: FFIConversionTo < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition { unsafe fn ffi_to_const (obj : example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition) -> * const example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition { ferment :: boxed (match obj { example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition :: V0 (o_0) => example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition :: V0 (ferment :: FFIConversionTo :: ffi_to (o_0)) , _ => unreachable ! ("This is unreachable") }) } }
                        impl ferment :: FFIConversionDestroy < example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: DataContractCreateTransition > for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition { unsafe fn destroy (ffi : * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition) { ferment :: unbox_any (ffi) ; ; } }
                        impl Drop for example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition { fn drop (& mut self) { unsafe { match self { example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition :: V0 (o_0) => { ferment :: unbox_any (* o_0) ; } , _ => unreachable ! ("This is unreachable") } ; } } }
                        #[no_mangle]                        pub unsafe extern "C" fn example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition_V0_ctor (o_o_0 : * mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: v0 :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_v0_DataContractCreateTransitionV0) -> * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition{
                            ferment :: boxed (example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition :: V0 (o_o_0))
                        }
                        #[no_mangle]
                        pub unsafe extern "C" fn example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition_destroy(
                            ffi : * mut example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                    }
                }
            }
        }
        #[doc = "FFI-representation of the [`example_nested::SomeStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct example_nested_SomeStruct {
            pub name: *mut std::os::raw::c_char,
            pub names: *mut std::os::raw::c_char,
        }
        impl ferment::FFIConversionFrom<example_nested::SomeStruct> for example_nested_SomeStruct {
            unsafe fn ffi_from_const(
                ffi: *const example_nested_SomeStruct,
            ) -> example_nested::SomeStruct {
                let ffi_ref = &*ffi;
                example_nested::SomeStruct {
                    name: ferment::FFIConversionFrom::ffi_from(ffi_ref.name),
                    names: ferment::FFIConversionFrom::ffi_from(ffi_ref.names),
                }
            }
        }
        impl ferment::FFIConversionTo<example_nested::SomeStruct> for example_nested_SomeStruct {
            unsafe fn ffi_to_const(
                obj: example_nested::SomeStruct,
            ) -> *const example_nested_SomeStruct {
                ferment::boxed(example_nested_SomeStruct {
                    name: ferment::FFIConversionTo::ffi_to(obj.name),
                    names: ferment::FFIConversionTo::ffi_to(obj.names),
                })
            }
        }
        impl ferment::FFIConversionDestroy<example_nested::SomeStruct> for example_nested_SomeStruct {
            unsafe fn destroy(ffi: *mut example_nested_SomeStruct) {
                ferment::unbox_any(ffi);
            }
        }
        impl Drop for example_nested_SomeStruct {
            fn drop(&mut self) {
                unsafe {
                    let ffi_ref = self;
                    <std::os::raw::c_char as ferment::FFIConversionDestroy<String>>::destroy(
                        ffi_ref.name,
                    );
                    <std::os::raw::c_char as ferment::FFIConversionDestroy<&str>>::destroy(
                        ffi_ref.names,
                    );
                }
            }
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_ctor(
            name: *mut std::os::raw::c_char,
            names: *mut std::os::raw::c_char,
        ) -> *mut example_nested_SomeStruct {
            ferment::boxed(example_nested_SomeStruct { name, names })
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_destroy(
            ffi: *mut example_nested_SomeStruct,
        ) {
            ferment::unbox_any(ffi);
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_get_name(
            obj: *const example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_get_names(
            obj: *const example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).names
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_set_name(
            obj: *const example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[no_mangle]
        pub unsafe extern "C" fn example_nested_SomeStruct_set_names(
            obj: *const example_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).names
        }
        pub mod entry {
            use crate as example_nested;
            pub mod processor {
                use crate as example_nested;
            }
            pub mod provider {
                use crate as example_nested;
            }
            #[doc = "FFI-representation of the [`example_nested::entry::SomeModel`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct example_nested_entry_SomeModel {
                pub hash: *mut crate::fermented::generics::Arr_u8_32,
                pub desc: *mut std::os::raw::c_char,
            }
            impl ferment::FFIConversionFrom<example_nested::entry::SomeModel>
                for example_nested_entry_SomeModel
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_entry_SomeModel,
                ) -> example_nested::entry::SomeModel {
                    let ffi_ref = &*ffi;
                    example_nested::entry::SomeModel {
                        hash: ferment::FFIConversionFrom::ffi_from(ffi_ref.hash),
                        desc: ferment::FFIConversionFrom::ffi_from(ffi_ref.desc),
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::entry::SomeModel> for example_nested_entry_SomeModel {
                unsafe fn ffi_to_const(
                    obj: example_nested::entry::SomeModel,
                ) -> *const example_nested_entry_SomeModel {
                    ferment::boxed(example_nested_entry_SomeModel {
                        hash: ferment::FFIConversionTo::ffi_to(obj.hash),
                        desc: ferment::FFIConversionTo::ffi_to(obj.desc),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::entry::SomeModel>
                for example_nested_entry_SomeModel
            {
                unsafe fn destroy(ffi: *mut example_nested_entry_SomeModel) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_entry_SomeModel {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.hash);
                        <std::os::raw::c_char as ferment::FFIConversionDestroy<String>>::destroy(
                            ffi_ref.desc,
                        );
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_ctor(
                hash: *mut crate::fermented::generics::Arr_u8_32,
                desc: *mut std::os::raw::c_char,
            ) -> *mut example_nested_entry_SomeModel {
                ferment::boxed(example_nested_entry_SomeModel { hash, desc })
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_destroy(
                ffi: *mut example_nested_entry_SomeModel,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_get_hash(
                obj: *const example_nested_entry_SomeModel,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).hash
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_get_desc(
                obj: *const example_nested_entry_SomeModel,
            ) -> *mut std::os::raw::c_char {
                (*obj).desc
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_set_hash(
                obj: *const example_nested_entry_SomeModel,
            ) -> *mut crate::fermented::generics::Arr_u8_32 {
                (*obj).hash
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_entry_SomeModel_set_desc(
                obj: *const example_nested_entry_SomeModel,
            ) -> *mut std::os::raw::c_char {
                (*obj).desc
            }
            pub mod core {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::entry::core::DashSharedCore::with_pointers`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_entry_core_DashSharedCore_with_pointers(
                    block_hash_by_height: example_nested::entry::BlockHashByHeight,
                    model_by_height: example_nested::entry::ModelByHeight,
                    context: *const std::os::raw::c_void,
                ) -> *mut example_nested::entry::core::DashSharedCore {
                    let obj = example_nested::entry::core::DashSharedCore::with_pointers(
                        block_hash_by_height,
                        model_by_height,
                        context,
                    );
                    ferment::boxed(obj)
                }
                #[doc = "FFI-representation of the [`example_nested::entry::core::DashSharedCore::with_lambdas`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_entry_core_DashSharedCore_with_lambdas(
                    block_hash_by_height: crate::fermented::generics::Fn_ARGS_u32_RTRN_Arr_u8_32,
                    model_by_height : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel,
                    context: *const std::os::raw::c_void,
                ) -> *mut example_nested::entry::core::DashSharedCore {
                    let obj = example_nested::entry::core::DashSharedCore::with_lambdas(
                        move |o_0| unsafe { block_hash_by_height.call(o_0) },
                        move |o_0| unsafe { model_by_height.call(o_0) },
                        context,
                    );
                    ferment::boxed(obj)
                }
            }
        }
        pub mod gen {
            use crate as example_nested;
            pub mod dict {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllArcExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllArcExamples { pub arc_simple : * mut crate :: fermented :: generics :: std_sync_Arc_u32 , pub arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot , pub arc_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Vec_u8 , pub arc_opt_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot , pub opt_arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_Option_String , pub crazy_type1 : * mut crate :: fermented :: generics :: Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError , pub crazy_type2 : * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllArcExamples>
                    for example_nested_gen_dict_AllArcExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllArcExamples,
                    ) -> example_nested::gen::dict::AllArcExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllArcExamples {
                            arc_simple: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_simple),
                            arc_complex: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_complex),
                            arc_generic: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_generic),
                            arc_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.arc_opt_generic,
                            ),
                            opt_arc_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_arc_complex,
                            ),
                            crazy_type1: ferment::FFIConversionFrom::ffi_from(ffi_ref.crazy_type1),
                            crazy_type2: ferment::FFIConversionFrom::ffi_from(ffi_ref.crazy_type2),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllArcExamples>
                    for example_nested_gen_dict_AllArcExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllArcExamples,
                    ) -> *const example_nested_gen_dict_AllArcExamples {
                        ferment::boxed(example_nested_gen_dict_AllArcExamples {
                            arc_simple: ferment::FFIConversionTo::ffi_to(obj.arc_simple),
                            arc_complex: ferment::FFIConversionTo::ffi_to(obj.arc_complex),
                            arc_generic: ferment::FFIConversionTo::ffi_to(obj.arc_generic),
                            arc_opt_generic: ferment::FFIConversionTo::ffi_to(obj.arc_opt_generic),
                            opt_arc_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_arc_complex,
                            ),
                            crazy_type1: ferment::FFIConversionTo::ffi_to(obj.crazy_type1),
                            crazy_type2: ferment::FFIConversionTo::ffi_to(obj.crazy_type2),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllArcExamples>
                    for example_nested_gen_dict_AllArcExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllArcExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllArcExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.arc_simple);
                            ferment::unbox_any(ffi_ref.arc_complex);
                            ferment::unbox_any(ffi_ref.arc_generic);
                            ferment::unbox_any(ffi_ref.arc_opt_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_arc_complex);
                            ferment::unbox_any(ffi_ref.crazy_type1);
                            ferment::unbox_any(ffi_ref.crazy_type2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_ctor(
                    arc_simple: *mut crate::fermented::generics::std_sync_Arc_u32,
                    arc_complex : * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot,
                    arc_generic: *mut crate::fermented::generics::std_sync_Arc_Vec_u8,
                    arc_opt_generic : * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
                    opt_arc_complex: *mut crate::fermented::generics::std_sync_Arc_Option_String,
                    crazy_type1 : * mut crate :: fermented :: generics :: Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
                    crazy_type2 : * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
                ) -> *mut example_nested_gen_dict_AllArcExamples {
                    ferment::boxed(example_nested_gen_dict_AllArcExamples {
                        arc_simple,
                        arc_complex,
                        arc_generic,
                        arc_opt_generic,
                        opt_arc_complex,
                        crazy_type1,
                        crazy_type2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllArcExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_arc_simple(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_arc_complex < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_arc_generic(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_arc_opt_generic < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_opt_arc_complex(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_crazy_type1 < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type1
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_get_crazy_type2 < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type2
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_arc_simple(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_arc_complex < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_arc_generic(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_arc_opt_generic < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_opt_arc_complex(
                    obj: *const example_nested_gen_dict_AllArcExamples,
                ) -> *mut crate::fermented::generics::std_sync_Arc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_crazy_type1 < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type1
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllArcExamples_set_crazy_type2 < > (obj : * const example_nested_gen_dict_AllArcExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type2
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllResultExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllResultExamples { pub result_ok_simple_err_simple : * mut crate :: fermented :: generics :: Result_ok_u32_err_u32 , pub result_ok_complex_err_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_String , pub result_ok_complex_2_err_complex : * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot , pub result_ok_complex_err_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Vec_u8 , pub result_ok_complex_err_opt_simple : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_u32 , pub result_ok_complex_err_opt_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_String , pub result_ok_complex_err_opt_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_Vec_u8 , pub crazy_type : * mut crate :: fermented :: generics :: Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError , pub crazy_type_2 : * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllResultExamples>
                    for example_nested_gen_dict_AllResultExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllResultExamples,
                    ) -> example_nested::gen::dict::AllResultExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllResultExamples {
                            result_ok_simple_err_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_simple_err_simple,
                            ),
                            result_ok_complex_err_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_err_complex,
                            ),
                            result_ok_complex_2_err_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_2_err_complex,
                            ),
                            result_ok_complex_err_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_err_generic,
                            ),
                            result_ok_complex_err_opt_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_err_opt_simple,
                            ),
                            result_ok_complex_err_opt_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_err_opt_complex,
                            ),
                            result_ok_complex_err_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.result_ok_complex_err_opt_generic,
                            ),
                            crazy_type: ferment::FFIConversionFrom::ffi_from(ffi_ref.crazy_type),
                            crazy_type_2: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.crazy_type_2,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllResultExamples>
                    for example_nested_gen_dict_AllResultExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllResultExamples,
                    ) -> *const example_nested_gen_dict_AllResultExamples {
                        ferment::boxed(example_nested_gen_dict_AllResultExamples {
                            result_ok_simple_err_simple: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_simple_err_simple,
                            ),
                            result_ok_complex_err_complex: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_err_complex,
                            ),
                            result_ok_complex_2_err_complex: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_2_err_complex,
                            ),
                            result_ok_complex_err_generic: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_err_generic,
                            ),
                            result_ok_complex_err_opt_simple: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_err_opt_simple,
                            ),
                            result_ok_complex_err_opt_complex: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_err_opt_complex,
                            ),
                            result_ok_complex_err_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.result_ok_complex_err_opt_generic,
                            ),
                            crazy_type: ferment::FFIConversionTo::ffi_to(obj.crazy_type),
                            crazy_type_2: ferment::FFIConversionTo::ffi_to(obj.crazy_type_2),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllResultExamples>
                    for example_nested_gen_dict_AllResultExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllResultExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllResultExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.result_ok_simple_err_simple);
                            ferment::unbox_any(ffi_ref.result_ok_complex_err_complex);
                            ferment::unbox_any(ffi_ref.result_ok_complex_2_err_complex);
                            ferment::unbox_any(ffi_ref.result_ok_complex_err_generic);
                            ferment::unbox_any(ffi_ref.result_ok_complex_err_opt_simple);
                            ferment::unbox_any(ffi_ref.result_ok_complex_err_opt_complex);
                            ferment::unbox_any(ffi_ref.result_ok_complex_err_opt_generic);
                            ferment::unbox_any(ffi_ref.crazy_type);
                            ferment::unbox_any(ffi_ref.crazy_type_2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_ctor(
                    result_ok_simple_err_simple : * mut crate :: fermented :: generics :: Result_ok_u32_err_u32,
                    result_ok_complex_err_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_String,
                    result_ok_complex_2_err_complex : * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot,
                    result_ok_complex_err_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Vec_u8,
                    result_ok_complex_err_opt_simple : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_u32,
                    result_ok_complex_err_opt_complex : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_String,
                    result_ok_complex_err_opt_generic : * mut crate :: fermented :: generics :: Result_ok_String_err_Option_Vec_u8,
                    crazy_type : * mut crate :: fermented :: generics :: Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
                    crazy_type_2 : * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError,
                ) -> *mut example_nested_gen_dict_AllResultExamples {
                    ferment::boxed(example_nested_gen_dict_AllResultExamples {
                        result_ok_simple_err_simple,
                        result_ok_complex_err_complex,
                        result_ok_complex_2_err_complex,
                        result_ok_complex_err_generic,
                        result_ok_complex_err_opt_simple,
                        result_ok_complex_err_opt_complex,
                        result_ok_complex_err_opt_generic,
                        crazy_type,
                        crazy_type_2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllResultExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_simple_err_simple(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    (*obj).result_ok_simple_err_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_complex(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_String {
                    (*obj).result_ok_complex_err_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_2_err_complex < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).result_ok_complex_2_err_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_generic(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Vec_u8 {
                    (*obj).result_ok_complex_err_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_simple(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_u32
                {
                    (*obj).result_ok_complex_err_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_complex(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_String
                {
                    (*obj).result_ok_complex_err_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_result_ok_complex_err_opt_generic(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_Vec_u8
                {
                    (*obj).result_ok_complex_err_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_crazy_type < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_get_crazy_type_2 < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_simple_err_simple(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_u32_err_u32 {
                    (*obj).result_ok_simple_err_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_complex(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_String {
                    (*obj).result_ok_complex_err_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_2_err_complex < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).result_ok_complex_2_err_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_generic(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Vec_u8 {
                    (*obj).result_ok_complex_err_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_simple(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_u32
                {
                    (*obj).result_ok_complex_err_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_complex(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_String
                {
                    (*obj).result_ok_complex_err_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_result_ok_complex_err_opt_generic(
                    obj: *const example_nested_gen_dict_AllResultExamples,
                ) -> *mut crate::fermented::generics::Result_ok_String_err_Option_Vec_u8
                {
                    (*obj).result_ok_complex_err_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_crazy_type < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllResultExamples_set_crazy_type_2 < > (obj : * const example_nested_gen_dict_AllResultExamples) -> * mut crate :: fermented :: generics :: Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).crazy_type_2
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllExamples { pub name : * mut std :: os :: raw :: c_char , pub all_map_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllMapExamples , pub all_result_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllResultExamples , pub all_set_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllSetExamples , pub all_arr_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllArrExamples , pub all_tuple_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllTupleExamples , pub all_opt_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllOptExamples }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllExamples>
                    for example_nested_gen_dict_AllExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllExamples,
                    ) -> example_nested::gen::dict::AllExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllExamples {
                            name: ferment::FFIConversionFrom::ffi_from(ffi_ref.name),
                            all_map_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_map_examples,
                            ),
                            all_result_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_result_examples,
                            ),
                            all_set_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_set_examples,
                            ),
                            all_arr_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_arr_examples,
                            ),
                            all_tuple_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_tuple_examples,
                            ),
                            all_opt_examples: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.all_opt_examples,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllExamples>
                    for example_nested_gen_dict_AllExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllExamples,
                    ) -> *const example_nested_gen_dict_AllExamples {
                        ferment::boxed(example_nested_gen_dict_AllExamples {
                            name: ferment::FFIConversionTo::ffi_to(obj.name),
                            all_map_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_map_examples,
                            ),
                            all_result_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_result_examples,
                            ),
                            all_set_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_set_examples,
                            ),
                            all_arr_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_arr_examples,
                            ),
                            all_tuple_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_tuple_examples,
                            ),
                            all_opt_examples: ferment::FFIConversionTo::ffi_to(
                                obj.all_opt_examples,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllExamples>
                    for example_nested_gen_dict_AllExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            < std :: os :: raw :: c_char as ferment :: FFIConversionDestroy < String >> :: destroy (ffi_ref . name) ;
                            ferment::unbox_any(ffi_ref.all_map_examples);
                            ferment::unbox_any(ffi_ref.all_result_examples);
                            ferment::unbox_any(ffi_ref.all_set_examples);
                            ferment::unbox_any(ffi_ref.all_arr_examples);
                            ferment::unbox_any(ffi_ref.all_tuple_examples);
                            ferment::unbox_any(ffi_ref.all_opt_examples);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_ctor(
                    name: *mut std::os::raw::c_char,
                    all_map_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllMapExamples,
                    all_result_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllResultExamples,
                    all_set_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllSetExamples,
                    all_arr_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllArrExamples,
                    all_tuple_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllTupleExamples,
                    all_opt_examples : * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllOptExamples,
                ) -> *mut example_nested_gen_dict_AllExamples {
                    ferment::boxed(example_nested_gen_dict_AllExamples {
                        name,
                        all_map_examples,
                        all_result_examples,
                        all_set_examples,
                        all_arr_examples,
                        all_tuple_examples,
                        all_opt_examples,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_name(
                    obj: *const example_nested_gen_dict_AllExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).name
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_map_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllMapExamples{
                    (*obj).all_map_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_result_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllResultExamples{
                    (*obj).all_result_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_set_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllSetExamples{
                    (*obj).all_set_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_arr_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllArrExamples{
                    (*obj).all_arr_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_tuple_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllTupleExamples{
                    (*obj).all_tuple_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_get_all_opt_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllOptExamples{
                    (*obj).all_opt_examples
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_name(
                    obj: *const example_nested_gen_dict_AllExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).name
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_map_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllMapExamples{
                    (*obj).all_map_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_result_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllResultExamples{
                    (*obj).all_result_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_set_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllSetExamples{
                    (*obj).all_set_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_arr_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllArrExamples{
                    (*obj).all_arr_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_tuple_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllTupleExamples{
                    (*obj).all_tuple_examples
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllExamples_set_all_opt_examples < > (obj : * const example_nested_gen_dict_AllExamples) -> * mut crate :: fermented :: types :: example_nested :: gen :: dict :: example_nested_gen_dict_AllOptExamples{
                    (*obj).all_opt_examples
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllArrExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllArrExamples {
                    pub arr: *mut crate::fermented::generics::Arr_u8_32,
                    pub opt_arr: *mut crate::fermented::generics::Arr_u8_32,
                    pub complex_arr: *mut crate::fermented::generics::Arr_String_32,
                    pub complex_arr_2:
                        *mut crate::fermented::generics::Arr_example_nested_model_Quorum_32,
                    pub generic_arr_2: *mut crate::fermented::generics::Arr_Vec_u8_32,
                }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllArrExamples>
                    for example_nested_gen_dict_AllArrExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllArrExamples,
                    ) -> example_nested::gen::dict::AllArrExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllArrExamples {
                            arr: ferment::FFIConversionFrom::ffi_from(ffi_ref.arr),
                            opt_arr: ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.opt_arr),
                            complex_arr: ferment::FFIConversionFrom::ffi_from(ffi_ref.complex_arr),
                            complex_arr_2: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.complex_arr_2,
                            ),
                            generic_arr_2: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.generic_arr_2,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllArrExamples>
                    for example_nested_gen_dict_AllArrExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllArrExamples,
                    ) -> *const example_nested_gen_dict_AllArrExamples {
                        ferment::boxed(example_nested_gen_dict_AllArrExamples {
                            arr: ferment::FFIConversionTo::ffi_to(obj.arr),
                            opt_arr: ferment::FFIConversionTo::ffi_to_opt(obj.opt_arr),
                            complex_arr: ferment::FFIConversionTo::ffi_to(obj.complex_arr),
                            complex_arr_2: ferment::FFIConversionTo::ffi_to(obj.complex_arr_2),
                            generic_arr_2: ferment::FFIConversionTo::ffi_to(obj.generic_arr_2),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllArrExamples>
                    for example_nested_gen_dict_AllArrExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllArrExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllArrExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.arr);
                            ferment::unbox_any_opt(ffi_ref.opt_arr);
                            ferment::unbox_any(ffi_ref.complex_arr);
                            ferment::unbox_any(ffi_ref.complex_arr_2);
                            ferment::unbox_any(ffi_ref.generic_arr_2);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_ctor(
                    arr: *mut crate::fermented::generics::Arr_u8_32,
                    opt_arr: *mut crate::fermented::generics::Arr_u8_32,
                    complex_arr: *mut crate::fermented::generics::Arr_String_32,
                    complex_arr_2 : * mut crate :: fermented :: generics :: Arr_example_nested_model_Quorum_32,
                    generic_arr_2: *mut crate::fermented::generics::Arr_Vec_u8_32,
                ) -> *mut example_nested_gen_dict_AllArrExamples {
                    ferment::boxed(example_nested_gen_dict_AllArrExamples {
                        arr,
                        opt_arr,
                        complex_arr,
                        complex_arr_2,
                        generic_arr_2,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllArrExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_get_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_get_opt_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).opt_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_get_complex_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_String_32 {
                    (*obj).complex_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_get_complex_arr_2(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_example_nested_model_Quorum_32
                {
                    (*obj).complex_arr_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_get_generic_arr_2(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_Vec_u8_32 {
                    (*obj).generic_arr_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_set_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_set_opt_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_u8_32 {
                    (*obj).opt_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_set_complex_arr(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_String_32 {
                    (*obj).complex_arr
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_set_complex_arr_2(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_example_nested_model_Quorum_32
                {
                    (*obj).complex_arr_2
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllArrExamples_set_generic_arr_2(
                    obj: *const example_nested_gen_dict_AllArrExamples,
                ) -> *mut crate::fermented::generics::Arr_Vec_u8_32 {
                    (*obj).generic_arr_2
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllRcExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllRcExamples { pub arc_simple : * mut crate :: fermented :: generics :: std_rc_Rc_u32 , pub arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot , pub arc_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Vec_u8 , pub arc_opt_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot , pub opt_arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_Option_String }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllRcExamples>
                    for example_nested_gen_dict_AllRcExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllRcExamples,
                    ) -> example_nested::gen::dict::AllRcExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllRcExamples {
                            arc_simple: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_simple),
                            arc_complex: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_complex),
                            arc_generic: ferment::FFIConversionFrom::ffi_from(ffi_ref.arc_generic),
                            arc_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.arc_opt_generic,
                            ),
                            opt_arc_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_arc_complex,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllRcExamples>
                    for example_nested_gen_dict_AllRcExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllRcExamples,
                    ) -> *const example_nested_gen_dict_AllRcExamples {
                        ferment::boxed(example_nested_gen_dict_AllRcExamples {
                            arc_simple: ferment::FFIConversionTo::ffi_to(obj.arc_simple),
                            arc_complex: ferment::FFIConversionTo::ffi_to(obj.arc_complex),
                            arc_generic: ferment::FFIConversionTo::ffi_to(obj.arc_generic),
                            arc_opt_generic: ferment::FFIConversionTo::ffi_to(obj.arc_opt_generic),
                            opt_arc_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_arc_complex,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllRcExamples>
                    for example_nested_gen_dict_AllRcExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllRcExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllRcExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.arc_simple);
                            ferment::unbox_any(ffi_ref.arc_complex);
                            ferment::unbox_any(ffi_ref.arc_generic);
                            ferment::unbox_any(ffi_ref.arc_opt_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_arc_complex);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_ctor(
                    arc_simple: *mut crate::fermented::generics::std_rc_Rc_u32,
                    arc_complex : * mut crate :: fermented :: generics :: std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot,
                    arc_generic: *mut crate::fermented::generics::std_rc_Rc_Vec_u8,
                    arc_opt_generic : * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
                    opt_arc_complex: *mut crate::fermented::generics::std_rc_Rc_Option_String,
                ) -> *mut example_nested_gen_dict_AllRcExamples {
                    ferment::boxed(example_nested_gen_dict_AllRcExamples {
                        arc_simple,
                        arc_complex,
                        arc_generic,
                        arc_opt_generic,
                        opt_arc_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllRcExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_get_arc_simple(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_get_arc_complex < > (obj : * const example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_get_arc_generic(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_get_arc_opt_generic < > (obj : * const example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_get_opt_arc_complex(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_set_arc_simple(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_u32 {
                    (*obj).arc_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_set_arc_complex < > (obj : * const example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_set_arc_generic(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Vec_u8 {
                    (*obj).arc_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_set_arc_opt_generic < > (obj : * const example_nested_gen_dict_AllRcExamples) -> * mut crate :: fermented :: generics :: std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRcExamples_set_opt_arc_complex(
                    obj: *const example_nested_gen_dict_AllRcExamples,
                ) -> *mut crate::fermented::generics::std_rc_Rc_Option_String {
                    (*obj).opt_arc_complex
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllTupleExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllTupleExamples {
                    pub tuple_string: *mut crate::fermented::generics::Tuple_String_String,
                    pub tuple_with_generic:
                        *mut crate::fermented::generics::Tuple_String_Vec_String,
                }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllTupleExamples>
                    for example_nested_gen_dict_AllTupleExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllTupleExamples,
                    ) -> example_nested::gen::dict::AllTupleExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllTupleExamples {
                            tuple_string: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.tuple_string,
                            ),
                            tuple_with_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.tuple_with_generic,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllTupleExamples>
                    for example_nested_gen_dict_AllTupleExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllTupleExamples,
                    ) -> *const example_nested_gen_dict_AllTupleExamples {
                        ferment::boxed(example_nested_gen_dict_AllTupleExamples {
                            tuple_string: ferment::FFIConversionTo::ffi_to(obj.tuple_string),
                            tuple_with_generic: ferment::FFIConversionTo::ffi_to(
                                obj.tuple_with_generic,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllTupleExamples>
                    for example_nested_gen_dict_AllTupleExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllTupleExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllTupleExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.tuple_string);
                            ferment::unbox_any(ffi_ref.tuple_with_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_ctor(
                    tuple_string: *mut crate::fermented::generics::Tuple_String_String,
                    tuple_with_generic: *mut crate::fermented::generics::Tuple_String_Vec_String,
                ) -> *mut example_nested_gen_dict_AllTupleExamples {
                    ferment::boxed(example_nested_gen_dict_AllTupleExamples {
                        tuple_string,
                        tuple_with_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllTupleExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_get_tuple_string(
                    obj: *const example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_String {
                    (*obj).tuple_string
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_get_tuple_with_generic(
                    obj: *const example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_Vec_String {
                    (*obj).tuple_with_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_set_tuple_string(
                    obj: *const example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_String {
                    (*obj).tuple_string
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllTupleExamples_set_tuple_with_generic(
                    obj: *const example_nested_gen_dict_AllTupleExamples,
                ) -> *mut crate::fermented::generics::Tuple_String_Vec_String {
                    (*obj).tuple_with_generic
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllSetExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllSetExamples { pub btreeset_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_u32 , pub btreeset_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String , pub btreeset_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Vec_u8 , pub btreeset_opt_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_u32 , pub btreeset_opt_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_String , pub btreeset_opt_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_Vec_u8 , pub hashset_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_u32 , pub hashset_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_String , pub hashset_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Vec_u8 , pub hashset_opt_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_u32 , pub hashset_opt_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_String , pub hashset_opt_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_Vec_u8 , pub hashset_opt_complex_external : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllSetExamples>
                    for example_nested_gen_dict_AllSetExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllSetExamples,
                    ) -> example_nested::gen::dict::AllSetExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllSetExamples {
                            btreeset_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_simple,
                            ),
                            btreeset_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_complex,
                            ),
                            btreeset_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_generic,
                            ),
                            btreeset_opt_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_opt_simple,
                            ),
                            btreeset_opt_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_opt_complex,
                            ),
                            btreeset_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.btreeset_opt_generic,
                            ),
                            hashset_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_simple,
                            ),
                            hashset_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_complex,
                            ),
                            hashset_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_generic,
                            ),
                            hashset_opt_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_opt_simple,
                            ),
                            hashset_opt_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_opt_complex,
                            ),
                            hashset_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_opt_generic,
                            ),
                            hashset_opt_complex_external: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.hashset_opt_complex_external,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllSetExamples>
                    for example_nested_gen_dict_AllSetExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllSetExamples,
                    ) -> *const example_nested_gen_dict_AllSetExamples {
                        ferment::boxed(example_nested_gen_dict_AllSetExamples {
                            btreeset_simple: ferment::FFIConversionTo::ffi_to(obj.btreeset_simple),
                            btreeset_complex: ferment::FFIConversionTo::ffi_to(
                                obj.btreeset_complex,
                            ),
                            btreeset_generic: ferment::FFIConversionTo::ffi_to(
                                obj.btreeset_generic,
                            ),
                            btreeset_opt_simple: ferment::FFIConversionTo::ffi_to(
                                obj.btreeset_opt_simple,
                            ),
                            btreeset_opt_complex: ferment::FFIConversionTo::ffi_to(
                                obj.btreeset_opt_complex,
                            ),
                            btreeset_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.btreeset_opt_generic,
                            ),
                            hashset_simple: ferment::FFIConversionTo::ffi_to(obj.hashset_simple),
                            hashset_complex: ferment::FFIConversionTo::ffi_to(obj.hashset_complex),
                            hashset_generic: ferment::FFIConversionTo::ffi_to(obj.hashset_generic),
                            hashset_opt_simple: ferment::FFIConversionTo::ffi_to(
                                obj.hashset_opt_simple,
                            ),
                            hashset_opt_complex: ferment::FFIConversionTo::ffi_to(
                                obj.hashset_opt_complex,
                            ),
                            hashset_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.hashset_opt_generic,
                            ),
                            hashset_opt_complex_external: ferment::FFIConversionTo::ffi_to(
                                obj.hashset_opt_complex_external,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllSetExamples>
                    for example_nested_gen_dict_AllSetExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllSetExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllSetExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.btreeset_simple);
                            ferment::unbox_any(ffi_ref.btreeset_complex);
                            ferment::unbox_any(ffi_ref.btreeset_generic);
                            ferment::unbox_any(ffi_ref.btreeset_opt_simple);
                            ferment::unbox_any(ffi_ref.btreeset_opt_complex);
                            ferment::unbox_any(ffi_ref.btreeset_opt_generic);
                            ferment::unbox_any(ffi_ref.hashset_simple);
                            ferment::unbox_any(ffi_ref.hashset_complex);
                            ferment::unbox_any(ffi_ref.hashset_generic);
                            ferment::unbox_any(ffi_ref.hashset_opt_simple);
                            ferment::unbox_any(ffi_ref.hashset_opt_complex);
                            ferment::unbox_any(ffi_ref.hashset_opt_generic);
                            ferment::unbox_any(ffi_ref.hashset_opt_complex_external);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_ctor(
                    btreeset_simple: *mut crate::fermented::generics::std_collections_BTreeSet_u32,
                    btreeset_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_String,
                    btreeset_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Vec_u8,
                    btreeset_opt_simple : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_u32,
                    btreeset_opt_complex : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_String,
                    btreeset_opt_generic : * mut crate :: fermented :: generics :: std_collections_BTreeSet_Option_Vec_u8,
                    hashset_simple: *mut crate::fermented::generics::std_collections_HashSet_u32,
                    hashset_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_String,
                    hashset_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Vec_u8,
                    hashset_opt_simple : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_u32,
                    hashset_opt_complex : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_String,
                    hashset_opt_generic : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_Vec_u8,
                    hashset_opt_complex_external : * mut crate :: fermented :: generics :: std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError,
                ) -> *mut example_nested_gen_dict_AllSetExamples {
                    ferment::boxed(example_nested_gen_dict_AllSetExamples {
                        btreeset_simple,
                        btreeset_complex,
                        btreeset_generic,
                        btreeset_opt_simple,
                        btreeset_opt_complex,
                        btreeset_opt_generic,
                        hashset_simple,
                        hashset_complex,
                        hashset_generic,
                        hashset_opt_simple,
                        hashset_opt_complex,
                        hashset_opt_generic,
                        hashset_opt_complex_external,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllSetExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_u32 {
                    (*obj).btreeset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                {
                    (*obj).btreeset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Vec_u8
                {
                    (*obj).btreeset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_opt_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_u32
                {
                    (*obj).btreeset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_opt_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_String
                {
                    (*obj).btreeset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_btreeset_opt_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_Vec_u8
                {
                    (*obj).btreeset_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_u32 {
                    (*obj).hashset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_String
                {
                    (*obj).hashset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Vec_u8
                {
                    (*obj).hashset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_opt_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_u32
                {
                    (*obj).hashset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_opt_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_String
                {
                    (*obj).hashset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_opt_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_Vec_u8
                {
                    (*obj).hashset_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_get_hashset_opt_complex_external < > (obj : * const example_nested_gen_dict_AllSetExamples) -> * mut crate :: fermented :: generics :: std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).hashset_opt_complex_external
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_u32 {
                    (*obj).btreeset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_String
                {
                    (*obj).btreeset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Vec_u8
                {
                    (*obj).btreeset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_opt_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_u32
                {
                    (*obj).btreeset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_opt_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_String
                {
                    (*obj).btreeset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_btreeset_opt_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_BTreeSet_Option_Vec_u8
                {
                    (*obj).btreeset_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_u32 {
                    (*obj).hashset_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_String
                {
                    (*obj).hashset_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Vec_u8
                {
                    (*obj).hashset_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_opt_simple(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_u32
                {
                    (*obj).hashset_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_opt_complex(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_String
                {
                    (*obj).hashset_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_opt_generic(
                    obj: *const example_nested_gen_dict_AllSetExamples,
                ) -> *mut crate::fermented::generics::std_collections_HashSet_Option_Vec_u8
                {
                    (*obj).hashset_opt_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllSetExamples_set_hashset_opt_complex_external < > (obj : * const example_nested_gen_dict_AllSetExamples) -> * mut crate :: fermented :: generics :: std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError{
                    (*obj).hashset_opt_complex_external
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllOptExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllOptExamples {
                    pub opt_complex: *mut std::os::raw::c_char,
                }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllOptExamples>
                    for example_nested_gen_dict_AllOptExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllOptExamples,
                    ) -> example_nested::gen::dict::AllOptExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllOptExamples {
                            opt_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_complex,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllOptExamples>
                    for example_nested_gen_dict_AllOptExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllOptExamples,
                    ) -> *const example_nested_gen_dict_AllOptExamples {
                        ferment::boxed(example_nested_gen_dict_AllOptExamples {
                            opt_complex: ferment::FFIConversionTo::ffi_to_opt(obj.opt_complex),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllOptExamples>
                    for example_nested_gen_dict_AllOptExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllOptExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllOptExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any_opt(ffi_ref.opt_complex);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllOptExamples_ctor(
                    opt_complex: *mut std::os::raw::c_char,
                ) -> *mut example_nested_gen_dict_AllOptExamples {
                    ferment::boxed(example_nested_gen_dict_AllOptExamples { opt_complex })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllOptExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllOptExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllOptExamples_get_opt_complex(
                    obj: *const example_nested_gen_dict_AllOptExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllOptExamples_set_opt_complex(
                    obj: *const example_nested_gen_dict_AllOptExamples,
                ) -> *mut std::os::raw::c_char {
                    (*obj).opt_complex
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllVecExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllVecExamples {
                    pub vec_simple: *mut crate::fermented::generics::Vec_u32,
                    pub vec_complex: *mut crate::fermented::generics::Vec_String,
                    pub vec_generic: *mut crate::fermented::generics::Vec_Vec_u8,
                    pub vec_opt_simple: *mut crate::fermented::generics::Vec_Option_u32,
                    pub vec_opt_complex: *mut crate::fermented::generics::Vec_Option_String,
                    pub vec_opt_generic: *mut crate::fermented::generics::Vec_Option_Vec_u8,
                }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllVecExamples>
                    for example_nested_gen_dict_AllVecExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllVecExamples,
                    ) -> example_nested::gen::dict::AllVecExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllVecExamples {
                            vec_simple: ferment::FFIConversionFrom::ffi_from(ffi_ref.vec_simple),
                            vec_complex: ferment::FFIConversionFrom::ffi_from(ffi_ref.vec_complex),
                            vec_generic: ferment::FFIConversionFrom::ffi_from(ffi_ref.vec_generic),
                            vec_opt_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.vec_opt_simple,
                            ),
                            vec_opt_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.vec_opt_complex,
                            ),
                            vec_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.vec_opt_generic,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllVecExamples>
                    for example_nested_gen_dict_AllVecExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllVecExamples,
                    ) -> *const example_nested_gen_dict_AllVecExamples {
                        ferment::boxed(example_nested_gen_dict_AllVecExamples {
                            vec_simple: ferment::FFIConversionTo::ffi_to(obj.vec_simple),
                            vec_complex: ferment::FFIConversionTo::ffi_to(obj.vec_complex),
                            vec_generic: ferment::FFIConversionTo::ffi_to(obj.vec_generic),
                            vec_opt_simple: ferment::FFIConversionTo::ffi_to(obj.vec_opt_simple),
                            vec_opt_complex: ferment::FFIConversionTo::ffi_to(obj.vec_opt_complex),
                            vec_opt_generic: ferment::FFIConversionTo::ffi_to(obj.vec_opt_generic),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllVecExamples>
                    for example_nested_gen_dict_AllVecExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllVecExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllVecExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.vec_simple);
                            ferment::unbox_any(ffi_ref.vec_complex);
                            ferment::unbox_any(ffi_ref.vec_generic);
                            ferment::unbox_any(ffi_ref.vec_opt_simple);
                            ferment::unbox_any(ffi_ref.vec_opt_complex);
                            ferment::unbox_any(ffi_ref.vec_opt_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_ctor(
                    vec_simple: *mut crate::fermented::generics::Vec_u32,
                    vec_complex: *mut crate::fermented::generics::Vec_String,
                    vec_generic: *mut crate::fermented::generics::Vec_Vec_u8,
                    vec_opt_simple: *mut crate::fermented::generics::Vec_Option_u32,
                    vec_opt_complex: *mut crate::fermented::generics::Vec_Option_String,
                    vec_opt_generic: *mut crate::fermented::generics::Vec_Option_Vec_u8,
                ) -> *mut example_nested_gen_dict_AllVecExamples {
                    ferment::boxed(example_nested_gen_dict_AllVecExamples {
                        vec_simple,
                        vec_complex,
                        vec_generic,
                        vec_opt_simple,
                        vec_opt_complex,
                        vec_opt_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllVecExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_simple(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_u32 {
                    (*obj).vec_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_complex(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_String {
                    (*obj).vec_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_generic(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Vec_u8 {
                    (*obj).vec_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_opt_simple(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_u32 {
                    (*obj).vec_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_opt_complex(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_String {
                    (*obj).vec_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_get_vec_opt_generic(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_Vec_u8 {
                    (*obj).vec_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_simple(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_u32 {
                    (*obj).vec_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_complex(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_String {
                    (*obj).vec_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_generic(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Vec_u8 {
                    (*obj).vec_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_opt_simple(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_u32 {
                    (*obj).vec_opt_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_opt_complex(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_String {
                    (*obj).vec_opt_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllVecExamples_set_vec_opt_generic(
                    obj: *const example_nested_gen_dict_AllVecExamples,
                ) -> *mut crate::fermented::generics::Vec_Option_Vec_u8 {
                    (*obj).vec_opt_generic
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllRwLockExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllRwLockExamples { pub rwlock_simple : * mut crate :: fermented :: generics :: std_sync_RwLock_u32 , pub rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot , pub rwlock_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Vec_u8 , pub rwlock_opt_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot , pub opt_rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_String , pub arc_rw_lock_complex : * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllRwLockExamples>
                    for example_nested_gen_dict_AllRwLockExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllRwLockExamples,
                    ) -> example_nested::gen::dict::AllRwLockExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllRwLockExamples {
                            rwlock_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.rwlock_simple,
                            ),
                            rwlock_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.rwlock_complex,
                            ),
                            rwlock_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.rwlock_generic,
                            ),
                            rwlock_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.rwlock_opt_generic,
                            ),
                            opt_rwlock_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_rwlock_complex,
                            ),
                            arc_rw_lock_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.arc_rw_lock_complex,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllRwLockExamples>
                    for example_nested_gen_dict_AllRwLockExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllRwLockExamples,
                    ) -> *const example_nested_gen_dict_AllRwLockExamples {
                        ferment::boxed(example_nested_gen_dict_AllRwLockExamples {
                            rwlock_simple: ferment::FFIConversionTo::ffi_to(obj.rwlock_simple),
                            rwlock_complex: ferment::FFIConversionTo::ffi_to(obj.rwlock_complex),
                            rwlock_generic: ferment::FFIConversionTo::ffi_to(obj.rwlock_generic),
                            rwlock_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.rwlock_opt_generic,
                            ),
                            opt_rwlock_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_rwlock_complex,
                            ),
                            arc_rw_lock_complex: ferment::FFIConversionTo::ffi_to(
                                obj.arc_rw_lock_complex,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllRwLockExamples>
                    for example_nested_gen_dict_AllRwLockExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllRwLockExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllRwLockExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.rwlock_simple);
                            ferment::unbox_any(ffi_ref.rwlock_complex);
                            ferment::unbox_any(ffi_ref.rwlock_generic);
                            ferment::unbox_any(ffi_ref.rwlock_opt_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_rwlock_complex);
                            ferment::unbox_any(ffi_ref.arc_rw_lock_complex);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_ctor(
                    rwlock_simple: *mut crate::fermented::generics::std_sync_RwLock_u32,
                    rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
                    rwlock_generic: *mut crate::fermented::generics::std_sync_RwLock_Vec_u8,
                    rwlock_opt_generic : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
                    opt_rwlock_complex : * mut crate :: fermented :: generics :: std_sync_RwLock_Option_String,
                    arc_rw_lock_complex : * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut example_nested_gen_dict_AllRwLockExamples {
                    ferment::boxed(example_nested_gen_dict_AllRwLockExamples {
                        rwlock_simple,
                        rwlock_complex,
                        rwlock_generic,
                        rwlock_opt_generic,
                        opt_rwlock_complex,
                        arc_rw_lock_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllRwLockExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_rwlock_simple(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_u32 {
                    (*obj).rwlock_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_rwlock_complex < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_rwlock_generic(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Vec_u8 {
                    (*obj).rwlock_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_rwlock_opt_generic < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_opt_rwlock_complex(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Option_String
                {
                    (*obj).opt_rwlock_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_get_arc_rw_lock_complex < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_rw_lock_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_rwlock_simple(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_u32 {
                    (*obj).rwlock_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_rwlock_complex < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_rwlock_generic(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Vec_u8 {
                    (*obj).rwlock_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_rwlock_opt_generic < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).rwlock_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_opt_rwlock_complex(
                    obj: *const example_nested_gen_dict_AllRwLockExamples,
                ) -> *mut crate::fermented::generics::std_sync_RwLock_Option_String
                {
                    (*obj).opt_rwlock_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRwLockExamples_set_arc_rw_lock_complex < > (obj : * const example_nested_gen_dict_AllRwLockExamples) -> * mut crate :: fermented :: generics :: std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).arc_rw_lock_complex
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllMutexExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllMutexExamples { pub mutex_simple : * mut crate :: fermented :: generics :: std_sync_Mutex_u32 , pub mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot , pub mutex_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Vec_u8 , pub mutex_opt_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot , pub opt_mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_String , pub platform_case : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllMutexExamples>
                    for example_nested_gen_dict_AllMutexExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllMutexExamples,
                    ) -> example_nested::gen::dict::AllMutexExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllMutexExamples {
                            mutex_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.mutex_simple,
                            ),
                            mutex_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.mutex_complex,
                            ),
                            mutex_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.mutex_generic,
                            ),
                            mutex_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.mutex_opt_generic,
                            ),
                            opt_mutex_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_mutex_complex,
                            ),
                            platform_case: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.platform_case,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllMutexExamples>
                    for example_nested_gen_dict_AllMutexExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllMutexExamples,
                    ) -> *const example_nested_gen_dict_AllMutexExamples {
                        ferment::boxed(example_nested_gen_dict_AllMutexExamples {
                            mutex_simple: ferment::FFIConversionTo::ffi_to(obj.mutex_simple),
                            mutex_complex: ferment::FFIConversionTo::ffi_to(obj.mutex_complex),
                            mutex_generic: ferment::FFIConversionTo::ffi_to(obj.mutex_generic),
                            mutex_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.mutex_opt_generic,
                            ),
                            opt_mutex_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_mutex_complex,
                            ),
                            platform_case: ferment::FFIConversionTo::ffi_to(obj.platform_case),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllMutexExamples>
                    for example_nested_gen_dict_AllMutexExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllMutexExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllMutexExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.mutex_simple);
                            ferment::unbox_any(ffi_ref.mutex_complex);
                            ferment::unbox_any(ffi_ref.mutex_generic);
                            ferment::unbox_any(ffi_ref.mutex_opt_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_mutex_complex);
                            ferment::unbox_any(ffi_ref.platform_case);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_ctor(
                    mutex_simple: *mut crate::fermented::generics::std_sync_Mutex_u32,
                    mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot,
                    mutex_generic: *mut crate::fermented::generics::std_sync_Mutex_Vec_u8,
                    mutex_opt_generic : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
                    opt_mutex_complex : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_String,
                    platform_case : * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut example_nested_gen_dict_AllMutexExamples {
                    ferment::boxed(example_nested_gen_dict_AllMutexExamples {
                        mutex_simple,
                        mutex_complex,
                        mutex_generic,
                        mutex_opt_generic,
                        opt_mutex_complex,
                        platform_case,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllMutexExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_mutex_simple(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_u32 {
                    (*obj).mutex_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_mutex_complex < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_mutex_generic(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Vec_u8 {
                    (*obj).mutex_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_mutex_opt_generic < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_opt_mutex_complex(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Option_String {
                    (*obj).opt_mutex_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_get_platform_case < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).platform_case
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_mutex_simple(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_u32 {
                    (*obj).mutex_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_mutex_complex < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_mutex_generic(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Vec_u8 {
                    (*obj).mutex_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_mutex_opt_generic < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).mutex_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_opt_mutex_complex(
                    obj: *const example_nested_gen_dict_AllMutexExamples,
                ) -> *mut crate::fermented::generics::std_sync_Mutex_Option_String {
                    (*obj).opt_mutex_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMutexExamples_set_platform_case < > (obj : * const example_nested_gen_dict_AllMutexExamples) -> * mut crate :: fermented :: generics :: std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).platform_case
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllRefCellExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllRefCellExamples { pub refcell_simple : * mut crate :: fermented :: generics :: std_cell_RefCell_u32 , pub refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot , pub refcell_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Vec_u8 , pub refcell_opt_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot , pub opt_refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_String }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllRefCellExamples>
                    for example_nested_gen_dict_AllRefCellExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllRefCellExamples,
                    ) -> example_nested::gen::dict::AllRefCellExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllRefCellExamples {
                            refcell_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.refcell_simple,
                            ),
                            refcell_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.refcell_complex,
                            ),
                            refcell_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.refcell_generic,
                            ),
                            refcell_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.refcell_opt_generic,
                            ),
                            opt_refcell_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_refcell_complex,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllRefCellExamples>
                    for example_nested_gen_dict_AllRefCellExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllRefCellExamples,
                    ) -> *const example_nested_gen_dict_AllRefCellExamples {
                        ferment::boxed(example_nested_gen_dict_AllRefCellExamples {
                            refcell_simple: ferment::FFIConversionTo::ffi_to(obj.refcell_simple),
                            refcell_complex: ferment::FFIConversionTo::ffi_to(obj.refcell_complex),
                            refcell_generic: ferment::FFIConversionTo::ffi_to(obj.refcell_generic),
                            refcell_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.refcell_opt_generic,
                            ),
                            opt_refcell_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_refcell_complex,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllRefCellExamples>
                    for example_nested_gen_dict_AllRefCellExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllRefCellExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllRefCellExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.refcell_simple);
                            ferment::unbox_any(ffi_ref.refcell_complex);
                            ferment::unbox_any(ffi_ref.refcell_generic);
                            ferment::unbox_any(ffi_ref.refcell_opt_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_refcell_complex);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_ctor(
                    refcell_simple: *mut crate::fermented::generics::std_cell_RefCell_u32,
                    refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot,
                    refcell_generic: *mut crate::fermented::generics::std_cell_RefCell_Vec_u8,
                    refcell_opt_generic : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
                    opt_refcell_complex : * mut crate :: fermented :: generics :: std_cell_RefCell_Option_String,
                ) -> *mut example_nested_gen_dict_AllRefCellExamples {
                    ferment::boxed(example_nested_gen_dict_AllRefCellExamples {
                        refcell_simple,
                        refcell_complex,
                        refcell_generic,
                        refcell_opt_generic,
                        opt_refcell_complex,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllRefCellExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_get_refcell_simple(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_u32 {
                    (*obj).refcell_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_get_refcell_complex < > (obj : * const example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_get_refcell_generic(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Vec_u8 {
                    (*obj).refcell_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_get_refcell_opt_generic < > (obj : * const example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_get_opt_refcell_complex(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Option_String
                {
                    (*obj).opt_refcell_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_set_refcell_simple(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_u32 {
                    (*obj).refcell_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_set_refcell_complex < > (obj : * const example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_set_refcell_generic(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Vec_u8 {
                    (*obj).refcell_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_set_refcell_opt_generic < > (obj : * const example_nested_gen_dict_AllRefCellExamples) -> * mut crate :: fermented :: generics :: std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
                    (*obj).refcell_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllRefCellExamples_set_opt_refcell_complex(
                    obj: *const example_nested_gen_dict_AllRefCellExamples,
                ) -> *mut crate::fermented::generics::std_cell_RefCell_Option_String
                {
                    (*obj).opt_refcell_complex
                }
                #[doc = "FFI-representation of the [`example_nested::gen::dict::AllMapExamples`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_gen_dict_AllMapExamples { pub k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32 , pub k_simple_v_opt_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_u32 , pub k_simple_v_opt_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String , pub k_simple_v_opt_generic_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32 , pub k_simple_v_opt_generic_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String , pub opt_map_k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32 , pub opt_map_k_simple_v_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_String , pub opt_map_k_simple_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u8 , pub opt_map_k_generic_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Vec_u8_values_Vec_u8 , pub map_k_opt_generic_v_opt_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 }
                impl ferment::FFIConversionFrom<example_nested::gen::dict::AllMapExamples>
                    for example_nested_gen_dict_AllMapExamples
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_gen_dict_AllMapExamples,
                    ) -> example_nested::gen::dict::AllMapExamples {
                        let ffi_ref = &*ffi;
                        example_nested::gen::dict::AllMapExamples {
                            k_simple_v_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.k_simple_v_simple,
                            ),
                            k_simple_v_opt_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.k_simple_v_opt_simple,
                            ),
                            k_simple_v_opt_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.k_simple_v_opt_complex,
                            ),
                            k_simple_v_opt_generic_simple: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.k_simple_v_opt_generic_simple,
                            ),
                            k_simple_v_opt_generic_complex: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.k_simple_v_opt_generic_complex,
                            ),
                            opt_map_k_simple_v_simple: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_map_k_simple_v_simple,
                            ),
                            opt_map_k_simple_v_complex: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_map_k_simple_v_complex,
                            ),
                            opt_map_k_simple_v_generic: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_map_k_simple_v_generic,
                            ),
                            opt_map_k_generic_v_generic: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.opt_map_k_generic_v_generic,
                            ),
                            map_k_opt_generic_v_opt_generic: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.map_k_opt_generic_v_opt_generic,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::gen::dict::AllMapExamples>
                    for example_nested_gen_dict_AllMapExamples
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::gen::dict::AllMapExamples,
                    ) -> *const example_nested_gen_dict_AllMapExamples {
                        ferment::boxed(example_nested_gen_dict_AllMapExamples {
                            k_simple_v_simple: ferment::FFIConversionTo::ffi_to(
                                obj.k_simple_v_simple,
                            ),
                            k_simple_v_opt_simple: ferment::FFIConversionTo::ffi_to(
                                obj.k_simple_v_opt_simple,
                            ),
                            k_simple_v_opt_complex: ferment::FFIConversionTo::ffi_to(
                                obj.k_simple_v_opt_complex,
                            ),
                            k_simple_v_opt_generic_simple: ferment::FFIConversionTo::ffi_to(
                                obj.k_simple_v_opt_generic_simple,
                            ),
                            k_simple_v_opt_generic_complex: ferment::FFIConversionTo::ffi_to(
                                obj.k_simple_v_opt_generic_complex,
                            ),
                            opt_map_k_simple_v_simple: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_map_k_simple_v_simple,
                            ),
                            opt_map_k_simple_v_complex: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_map_k_simple_v_complex,
                            ),
                            opt_map_k_simple_v_generic: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_map_k_simple_v_generic,
                            ),
                            opt_map_k_generic_v_generic: ferment::FFIConversionTo::ffi_to_opt(
                                obj.opt_map_k_generic_v_generic,
                            ),
                            map_k_opt_generic_v_opt_generic: ferment::FFIConversionTo::ffi_to(
                                obj.map_k_opt_generic_v_opt_generic,
                            ),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::gen::dict::AllMapExamples>
                    for example_nested_gen_dict_AllMapExamples
                {
                    unsafe fn destroy(ffi: *mut example_nested_gen_dict_AllMapExamples) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_gen_dict_AllMapExamples {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.k_simple_v_simple);
                            ferment::unbox_any(ffi_ref.k_simple_v_opt_simple);
                            ferment::unbox_any(ffi_ref.k_simple_v_opt_complex);
                            ferment::unbox_any(ffi_ref.k_simple_v_opt_generic_simple);
                            ferment::unbox_any(ffi_ref.k_simple_v_opt_generic_complex);
                            ferment::unbox_any_opt(ffi_ref.opt_map_k_simple_v_simple);
                            ferment::unbox_any_opt(ffi_ref.opt_map_k_simple_v_complex);
                            ferment::unbox_any_opt(ffi_ref.opt_map_k_simple_v_generic);
                            ferment::unbox_any_opt(ffi_ref.opt_map_k_generic_v_generic);
                            ferment::unbox_any(ffi_ref.map_k_opt_generic_v_opt_generic);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_ctor(
                    k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32,
                    k_simple_v_opt_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_u32,
                    k_simple_v_opt_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String,
                    k_simple_v_opt_generic_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32,
                    k_simple_v_opt_generic_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String,
                    opt_map_k_simple_v_simple : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_u32,
                    opt_map_k_simple_v_complex : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_String,
                    opt_map_k_simple_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Vec_u8,
                    opt_map_k_generic_v_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Vec_u8_values_Vec_u8,
                    map_k_opt_generic_v_opt_generic : * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
                ) -> *mut example_nested_gen_dict_AllMapExamples {
                    ferment::boxed(example_nested_gen_dict_AllMapExamples {
                        k_simple_v_simple,
                        k_simple_v_opt_simple,
                        k_simple_v_opt_complex,
                        k_simple_v_opt_generic_simple,
                        k_simple_v_opt_generic_complex,
                        opt_map_k_simple_v_simple,
                        opt_map_k_simple_v_complex,
                        opt_map_k_simple_v_generic,
                        opt_map_k_generic_v_generic,
                        map_k_opt_generic_v_opt_generic,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_destroy(
                    ffi: *mut example_nested_gen_dict_AllMapExamples,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_k_simple_v_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Option_u32
                {
                    (*obj).k_simple_v_opt_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_complex < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String{
                    (*obj).k_simple_v_opt_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_generic_simple < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32{
                    (*obj).k_simple_v_opt_generic_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_k_simple_v_opt_generic_complex < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String{
                    (*obj).k_simple_v_opt_generic_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).opt_map_k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_complex(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_String
                {
                    (*obj).opt_map_k_simple_v_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_opt_map_k_simple_v_generic(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u8
                {
                    (*obj).opt_map_k_simple_v_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_opt_map_k_generic_v_generic(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_Vec_u8_values_Vec_u8
                {
                    (*obj).opt_map_k_generic_v_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_get_map_k_opt_generic_v_opt_generic < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8{
                    (*obj).map_k_opt_generic_v_opt_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_k_simple_v_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Option_u32
                {
                    (*obj).k_simple_v_opt_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_complex < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_String{
                    (*obj).k_simple_v_opt_complex
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_generic_simple < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_u32{
                    (*obj).k_simple_v_opt_generic_simple
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_k_simple_v_opt_generic_complex < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_Option_Vec_String{
                    (*obj).k_simple_v_opt_generic_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_simple(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_u32
                {
                    (*obj).opt_map_k_simple_v_simple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_complex(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_String
                {
                    (*obj).opt_map_k_simple_v_complex
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_opt_map_k_simple_v_generic(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_u32_values_Vec_u8
                {
                    (*obj).opt_map_k_simple_v_generic
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_opt_map_k_generic_v_generic(
                    obj: *const example_nested_gen_dict_AllMapExamples,
                ) -> *mut crate::fermented::generics::std_collections_Map_keys_Vec_u8_values_Vec_u8
                {
                    (*obj).opt_map_k_generic_v_generic
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_gen_dict_AllMapExamples_set_map_k_opt_generic_v_opt_generic < > (obj : * const example_nested_gen_dict_AllMapExamples) -> * mut crate :: fermented :: generics :: std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8{
                    (*obj).map_k_opt_generic_v_opt_generic
                }
            }
        }
        pub mod model {
            use crate as example_nested;
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::model::TestModLevelOptSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_nested_model_TestModLevelOptSnapshot {
                VO (* mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl ferment::FFIConversionFrom<example_nested::model::TestModLevelOptSnapshot>
                for example_nested_model_TestModLevelOptSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_model_TestModLevelOptSnapshot,
                ) -> example_nested::model::TestModLevelOptSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                            example_nested::model::TestModLevelOptSnapshot::VO(
                                ferment::FFIConversionFrom::ffi_from_opt(*o_0),
                            )
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::model::TestModLevelOptSnapshot>
                for example_nested_model_TestModLevelOptSnapshot
            {
                unsafe fn ffi_to_const(
                    obj: example_nested::model::TestModLevelOptSnapshot,
                ) -> *const example_nested_model_TestModLevelOptSnapshot {
                    ferment::boxed(match obj {
                        example_nested::model::TestModLevelOptSnapshot::VO(o_0) => {
                            example_nested_model_TestModLevelOptSnapshot::VO(
                                ferment::FFIConversionTo::ffi_to_opt(o_0),
                            )
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::model::TestModLevelOptSnapshot>
                for example_nested_model_TestModLevelOptSnapshot
            {
                unsafe fn destroy(ffi: *mut example_nested_model_TestModLevelOptSnapshot) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_model_TestModLevelOptSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_nested_model_TestModLevelOptSnapshot::VO(o_0) => {
                                ferment::unbox_any_opt(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelOptSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut example_nested_model_TestModLevelOptSnapshot {
                ferment::boxed(example_nested_model_TestModLevelOptSnapshot::VO(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelOptSnapshot_destroy(
                ffi: *mut example_nested_model_TestModLevelOptSnapshot,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod ferment_example {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::model::ferment_example::get_crazy_case`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_ferment_example_get_crazy_case(
                ) -> *mut crate::fermented::types::example_nested::model::example_nested_model_Quorum
                {
                    let obj = example_nested::model::ferment_example::get_crazy_case();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`example_nested::model::ferment_example::get_rotated_quorum`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_ferment_example_get_rotated_quorum(
                ) -> *mut crate::fermented::types::example_nested::model::example_nested_model_Quorum
                {
                    let obj = example_nested::model::ferment_example::get_rotated_quorum();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
            }
            #[doc = "FFI-representation of the [`example_nested::model::LLMQParams`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct example_nested_model_LLMQParams {
                pub r#type: *mut std::os::raw::c_char,
                pub known_confirmed_at_height: *mut u32,
            }
            impl ferment::FFIConversionFrom<example_nested::model::LLMQParams>
                for example_nested_model_LLMQParams
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_model_LLMQParams,
                ) -> example_nested::model::LLMQParams {
                    let ffi_ref = &*ffi;
                    example_nested::model::LLMQParams {
                        r#type: ferment::FFIConversionFrom::ffi_from(ffi_ref.r#type),
                        known_confirmed_at_height: ferment::from_opt_primitive(
                            ffi_ref.known_confirmed_at_height,
                        ),
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::model::LLMQParams>
                for example_nested_model_LLMQParams
            {
                unsafe fn ffi_to_const(
                    obj: example_nested::model::LLMQParams,
                ) -> *const example_nested_model_LLMQParams {
                    ferment::boxed(example_nested_model_LLMQParams {
                        r#type: ferment::FFIConversionTo::ffi_to(obj.r#type),
                        known_confirmed_at_height: ferment::to_opt_primitive(
                            obj.known_confirmed_at_height,
                        ),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::model::LLMQParams>
                for example_nested_model_LLMQParams
            {
                unsafe fn destroy(ffi: *mut example_nested_model_LLMQParams) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_model_LLMQParams {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        <std::os::raw::c_char as ferment::FFIConversionDestroy<String>>::destroy(
                            ffi_ref.r#type,
                        );
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_ctor(
                r#type: *mut std::os::raw::c_char,
                known_confirmed_at_height: *mut u32,
            ) -> *mut example_nested_model_LLMQParams {
                ferment::boxed(example_nested_model_LLMQParams {
                    r#type,
                    known_confirmed_at_height,
                })
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_destroy(
                ffi: *mut example_nested_model_LLMQParams,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_get_type(
                obj: *const example_nested_model_LLMQParams,
            ) -> *mut std::os::raw::c_char {
                (*obj).r#type
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_get_known_confirmed_at_height(
                obj: *const example_nested_model_LLMQParams,
            ) -> *mut u32 {
                (*obj).known_confirmed_at_height
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_set_type(
                obj: *const example_nested_model_LLMQParams,
            ) -> *mut std::os::raw::c_char {
                (*obj).r#type
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_LLMQParams_set_known_confirmed_at_height(
                obj: *const example_nested_model_LLMQParams,
            ) -> *mut u32 {
                (*obj).known_confirmed_at_height
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::model::TestModLevelVecSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_nested_model_TestModLevelVecSnapshot {
                VO (* mut crate :: fermented :: generics :: Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode) }
            impl ferment::FFIConversionFrom<example_nested::model::TestModLevelVecSnapshot>
                for example_nested_model_TestModLevelVecSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_model_TestModLevelVecSnapshot,
                ) -> example_nested::model::TestModLevelVecSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                            example_nested::model::TestModLevelVecSnapshot::VO(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::model::TestModLevelVecSnapshot>
                for example_nested_model_TestModLevelVecSnapshot
            {
                unsafe fn ffi_to_const(
                    obj: example_nested::model::TestModLevelVecSnapshot,
                ) -> *const example_nested_model_TestModLevelVecSnapshot {
                    ferment::boxed(match obj {
                        example_nested::model::TestModLevelVecSnapshot::VO(o_0) => {
                            example_nested_model_TestModLevelVecSnapshot::VO(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::model::TestModLevelVecSnapshot>
                for example_nested_model_TestModLevelVecSnapshot
            {
                unsafe fn destroy(ffi: *mut example_nested_model_TestModLevelVecSnapshot) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_model_TestModLevelVecSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_nested_model_TestModLevelVecSnapshot::VO(o_0) => {
                                ferment::unbox_any(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelVecSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: generics :: Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode,
            ) -> *mut example_nested_model_TestModLevelVecSnapshot {
                ferment::boxed(example_nested_model_TestModLevelVecSnapshot::VO(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelVecSnapshot_destroy(
                ffi: *mut example_nested_model_TestModLevelVecSnapshot,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod quorum {
                use crate as example_nested;
                pub mod quorum_type {
                    use crate as example_nested;
                    #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::model::quorum::quorum_type::QuorumType`]\"`]"]
                    #[repr(C)]
                    #[derive(Clone)]
                    #[non_exhaustive]
                    pub enum example_nested_model_quorum_quorum_type_QuorumType {
                        Normal,
                        Rotated,
                    }
                    impl
                        ferment::FFIConversionFrom<
                            example_nested::model::quorum::quorum_type::QuorumType,
                        > for example_nested_model_quorum_quorum_type_QuorumType
                    {
                        unsafe fn ffi_from_const(
                            ffi: *const example_nested_model_quorum_quorum_type_QuorumType,
                        ) -> example_nested::model::quorum::quorum_type::QuorumType
                        {
                            let ffi_ref = &*ffi;
                            match ffi_ref {
                                example_nested_model_quorum_quorum_type_QuorumType::Normal => {
                                    example_nested::model::quorum::quorum_type::QuorumType::Normal
                                }
                                example_nested_model_quorum_quorum_type_QuorumType::Rotated => {
                                    example_nested::model::quorum::quorum_type::QuorumType::Rotated
                                }
                            }
                        }
                    }
                    impl
                        ferment::FFIConversionTo<
                            example_nested::model::quorum::quorum_type::QuorumType,
                        > for example_nested_model_quorum_quorum_type_QuorumType
                    {
                        unsafe fn ffi_to_const(
                            obj: example_nested::model::quorum::quorum_type::QuorumType,
                        ) -> *const example_nested_model_quorum_quorum_type_QuorumType
                        {
                            ferment::boxed(match obj {
                                example_nested::model::quorum::quorum_type::QuorumType::Normal => {
                                    example_nested_model_quorum_quorum_type_QuorumType::Normal
                                }
                                example_nested::model::quorum::quorum_type::QuorumType::Rotated => {
                                    example_nested_model_quorum_quorum_type_QuorumType::Rotated
                                }
                                _ => unreachable!("This is unreachable"),
                            })
                        }
                    }
                    impl
                        ferment::FFIConversionDestroy<
                            example_nested::model::quorum::quorum_type::QuorumType,
                        > for example_nested_model_quorum_quorum_type_QuorumType
                    {
                        unsafe fn destroy(
                            ffi: *mut example_nested_model_quorum_quorum_type_QuorumType,
                        ) {
                            ferment::unbox_any(ffi);
                        }
                    }
                    impl Drop for example_nested_model_quorum_quorum_type_QuorumType {
                        fn drop(&mut self) {
                            unsafe {
                                match self {
                                    example_nested_model_quorum_quorum_type_QuorumType::Normal => {}
                                    example_nested_model_quorum_quorum_type_QuorumType::Rotated => {
                                    }
                                    _ => unreachable!("This is unreachable"),
                                };
                            }
                        }
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_nested_model_quorum_quorum_type_QuorumType_Normal_ctor(
                    ) -> *mut example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment::boxed(
                            example_nested_model_quorum_quorum_type_QuorumType::Normal {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_nested_model_quorum_quorum_type_QuorumType_Rotated_ctor(
                    ) -> *mut example_nested_model_quorum_quorum_type_QuorumType
                    {
                        ferment::boxed(
                            example_nested_model_quorum_quorum_type_QuorumType::Rotated {},
                        )
                    }
                    #[no_mangle]
                    pub unsafe extern "C" fn example_nested_model_quorum_quorum_type_QuorumType_destroy(
                        ffi: *mut example_nested_model_quorum_quorum_type_QuorumType,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
            }
            pub mod snapshot {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::model::snapshot::LLMQSnapshotSkipMode`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                #[non_exhaustive]
                pub enum example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    NoSkipping = 0,
                    SkipFirst = 1,
                    SkipExcept = 2,
                    SkipAll = 3,
                }
                impl
                    ferment::FFIConversionFrom<
                        example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    > for example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) -> example_nested::model::snapshot::LLMQSnapshotSkipMode {
                        let ffi_ref = &*ffi;
                        match ffi_ref {
                            example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping => {
                                example_nested::model::snapshot::LLMQSnapshotSkipMode::NoSkipping
                            }
                            example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst => {
                                example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipFirst
                            }
                            example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept => {
                                example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipExcept
                            }
                            example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll => {
                                example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipAll
                            }
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshotSkipMode>
                    for example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    ) -> *const example_nested_model_snapshot_LLMQSnapshotSkipMode
                    {
                        ferment::boxed(match obj {
                            example_nested::model::snapshot::LLMQSnapshotSkipMode::NoSkipping => {
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping
                            }
                            example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipFirst => {
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst
                            }
                            example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipExcept => {
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept
                            }
                            example_nested::model::snapshot::LLMQSnapshotSkipMode::SkipAll => {
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll
                            }
                            _ => unreachable!("This is unreachable"),
                        })
                    }
                }
                impl
                    ferment::FFIConversionDestroy<
                        example_nested::model::snapshot::LLMQSnapshotSkipMode,
                    > for example_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn destroy(
                        ffi: *mut example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    fn drop(&mut self) {
                        unsafe {
                            match self {
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping => {}
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst => {}
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept => {}
                                example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll => {}
                                _ => unreachable!("This is unreachable"),
                            };
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshotSkipMode_NoSkipping_ctor(
                ) -> *mut example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    ferment::boxed(
                        example_nested_model_snapshot_LLMQSnapshotSkipMode::NoSkipping {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipFirst_ctor(
                ) -> *mut example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    ferment::boxed(example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipFirst {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipExcept_ctor(
                ) -> *mut example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    ferment::boxed(
                        example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipExcept {},
                    )
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshotSkipMode_SkipAll_ctor(
                ) -> *mut example_nested_model_snapshot_LLMQSnapshotSkipMode {
                    ferment::boxed(example_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll {})
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
                    ffi: *mut example_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`example_nested::model::snapshot::LLMQSnapshot`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_model_snapshot_LLMQSnapshot { pub member_list : * mut crate :: fermented :: generics :: Vec_u8 , pub skip_list : * mut crate :: fermented :: generics :: Vec_i32 , pub skip_list_mode : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode , pub option_vec : * mut crate :: fermented :: generics :: Vec_u8 }
                impl ferment::FFIConversionFrom<example_nested::model::snapshot::LLMQSnapshot>
                    for example_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_model_snapshot_LLMQSnapshot,
                    ) -> example_nested::model::snapshot::LLMQSnapshot {
                        let ffi_ref = &*ffi;
                        example_nested::model::snapshot::LLMQSnapshot {
                            member_list: ferment::FFIConversionFrom::ffi_from(ffi_ref.member_list),
                            skip_list: ferment::FFIConversionFrom::ffi_from(ffi_ref.skip_list),
                            skip_list_mode: ferment::FFIConversionFrom::ffi_from(
                                ffi_ref.skip_list_mode,
                            ),
                            option_vec: ferment::FFIConversionFrom::ffi_from_opt(
                                ffi_ref.option_vec,
                            ),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::model::snapshot::LLMQSnapshot>
                    for example_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::model::snapshot::LLMQSnapshot,
                    ) -> *const example_nested_model_snapshot_LLMQSnapshot {
                        ferment::boxed(example_nested_model_snapshot_LLMQSnapshot {
                            member_list: ferment::FFIConversionTo::ffi_to(obj.member_list),
                            skip_list: ferment::FFIConversionTo::ffi_to(obj.skip_list),
                            skip_list_mode: ferment::FFIConversionTo::ffi_to(obj.skip_list_mode),
                            option_vec: ferment::FFIConversionTo::ffi_to_opt(obj.option_vec),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::model::snapshot::LLMQSnapshot>
                    for example_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn destroy(ffi: *mut example_nested_model_snapshot_LLMQSnapshot) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_model_snapshot_LLMQSnapshot {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.member_list);
                            ferment::unbox_any(ffi_ref.skip_list);
                            ferment::unbox_any(ffi_ref.skip_list_mode);
                            ferment::unbox_any_opt(ffi_ref.option_vec);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_ctor(
                    member_list: *mut crate::fermented::generics::Vec_u8,
                    skip_list: *mut crate::fermented::generics::Vec_i32,
                    skip_list_mode : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode,
                    option_vec: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut example_nested_model_snapshot_LLMQSnapshot {
                    ferment::boxed(example_nested_model_snapshot_LLMQSnapshot {
                        member_list,
                        skip_list,
                        skip_list_mode,
                        option_vec,
                    })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_destroy(
                    ffi: *mut example_nested_model_snapshot_LLMQSnapshot,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_get_member_list(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_get_skip_list(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_get_skip_list_mode < > (obj : * const example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_get_option_vec(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_set_member_list(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_set_skip_list(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_set_skip_list_mode < > (obj : * const example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_snapshot_LLMQSnapshot_set_option_vec(
                    obj: *const example_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
            }
            pub mod some_inner_2 {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::model::some_inner_2::get_normal_quorum`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_some_inner_2_get_normal_quorum(
                ) -> *mut crate::fermented::types::example_nested::model::example_nested_model_Quorum
                {
                    let obj = example_nested::model::some_inner_2::get_normal_quorum();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
            }
            pub mod some_inner {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::model::some_inner::get_normal_quorum`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_some_inner_get_normal_quorum(
                ) -> *mut crate::fermented::types::example_nested::model::example_nested_model_Quorum
                {
                    let obj = example_nested::model::some_inner::get_normal_quorum();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
            }
            #[doc = "FFI-representation of the [`example_nested::model::Quorum`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub struct example_nested_model_Quorum { pub llmq_type : * mut crate :: fermented :: types :: example_nested :: model :: quorum :: quorum_type :: example_nested_model_quorum_quorum_type_QuorumType }
            impl ferment::FFIConversionFrom<example_nested::model::Quorum> for example_nested_model_Quorum {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_model_Quorum,
                ) -> example_nested::model::Quorum {
                    let ffi_ref = &*ffi;
                    example_nested::model::Quorum {
                        llmq_type: ferment::FFIConversionFrom::ffi_from(ffi_ref.llmq_type),
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::model::Quorum> for example_nested_model_Quorum {
                unsafe fn ffi_to_const(
                    obj: example_nested::model::Quorum,
                ) -> *const example_nested_model_Quorum {
                    ferment::boxed(example_nested_model_Quorum {
                        llmq_type: ferment::FFIConversionTo::ffi_to(obj.llmq_type),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::model::Quorum> for example_nested_model_Quorum {
                unsafe fn destroy(ffi: *mut example_nested_model_Quorum) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_model_Quorum {
                fn drop(&mut self) {
                    unsafe {
                        let ffi_ref = self;
                        ferment::unbox_any(ffi_ref.llmq_type);
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_Quorum_ctor(
                llmq_type : * mut crate :: fermented :: types :: example_nested :: model :: quorum :: quorum_type :: example_nested_model_quorum_quorum_type_QuorumType,
            ) -> *mut example_nested_model_Quorum {
                ferment::boxed(example_nested_model_Quorum { llmq_type })
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_Quorum_destroy(
                ffi: *mut example_nested_model_Quorum,
            ) {
                ferment::unbox_any(ffi);
            }
            #[no_mangle]            pub unsafe extern "C" fn example_nested_model_Quorum_get_llmq_type < > (obj : * const example_nested_model_Quorum) -> * mut crate :: fermented :: types :: example_nested :: model :: quorum :: quorum_type :: example_nested_model_quorum_quorum_type_QuorumType{
                (*obj).llmq_type
            }
            #[no_mangle]            pub unsafe extern "C" fn example_nested_model_Quorum_set_llmq_type < > (obj : * const example_nested_model_Quorum) -> * mut crate :: fermented :: types :: example_nested :: model :: quorum :: quorum_type :: example_nested_model_quorum_quorum_type_QuorumType{
                (*obj).llmq_type
            }
            pub mod callback {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::model::callback::find_current_block_classic`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_find_current_block_classic(
                    _callback: example_nested::model::callback::ClassicCallback,
                ) {
                    let obj =
                        example_nested::model::callback::find_current_block_classic(_callback);
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::find_current_block_desc`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_find_current_block_desc(
                    _callback: crate::fermented::generics::Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String,
                ) {
                    let obj = example_nested::model::callback::find_current_block_desc(
                        move |o_0, o_1| unsafe { _callback.call(o_0, o_1) },
                    );
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::lookup_block_hash_by_height`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_lookup_block_hash_by_height(
                    _callback: crate::fermented::generics::Fn_ARGS_u32_RTRN_Option_u8,
                ) {
                    let obj = example_nested::model::callback::lookup_block_hash_by_height(
                        move |o_0| unsafe { _callback.call(o_0) },
                    );
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::lookup_merkle_root_by_hash`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_lookup_merkle_root_by_hash(
                    _callback: crate::fermented::generics::Fn_ARGS_Arr_u8_32_RTRN_Option_u8,
                ) {
                    let obj = example_nested::model::callback::lookup_merkle_root_by_hash(
                        move |o_0| unsafe { _callback.call(o_0) },
                    );
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::should_process_diff_in_range2`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_should_process_diff_in_range2(
                    _callback : crate :: fermented :: generics :: Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
                ) {
                    let obj = example_nested::model::callback::should_process_diff_in_range2(
                        move |o_0, o_1| unsafe { _callback.call(o_0, o_1) },
                    );
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::find_current_block_desc_mut`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_find_current_block_desc_mut(
                    _callback : crate :: fermented :: generics :: FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String,
                ) {
                    let obj = example_nested::model::callback::find_current_block_desc_mut(
                        move |o_0, o_1| unsafe { _callback.call(o_0, o_1) },
                    );
                }
                #[doc = "FFI-representation of the [`example_nested::model::callback::setup_two_callbacks`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_callback_setup_two_callbacks(
                    _callback1 : crate :: fermented :: generics :: Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
                    _callback2 : crate :: fermented :: generics :: Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
                ) {
                    let obj = example_nested::model::callback::setup_two_callbacks(
                        move |o_0, o_1| unsafe { _callback1.call(o_0, o_1) },
                        move |o_0| unsafe { _callback2.call(o_0) },
                    );
                }
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`example_nested::model::TestModLevelSnapshot`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            #[non_exhaustive]
            pub enum example_nested_model_TestModLevelSnapshot {
                VO (* mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot) }
            impl ferment::FFIConversionFrom<example_nested::model::TestModLevelSnapshot>
                for example_nested_model_TestModLevelSnapshot
            {
                unsafe fn ffi_from_const(
                    ffi: *const example_nested_model_TestModLevelSnapshot,
                ) -> example_nested::model::TestModLevelSnapshot {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                            example_nested::model::TestModLevelSnapshot::VO(
                                ferment::FFIConversionFrom::ffi_from(*o_0),
                            )
                        }
                    }
                }
            }
            impl ferment::FFIConversionTo<example_nested::model::TestModLevelSnapshot>
                for example_nested_model_TestModLevelSnapshot
            {
                unsafe fn ffi_to_const(
                    obj: example_nested::model::TestModLevelSnapshot,
                ) -> *const example_nested_model_TestModLevelSnapshot {
                    ferment::boxed(match obj {
                        example_nested::model::TestModLevelSnapshot::VO(o_0) => {
                            example_nested_model_TestModLevelSnapshot::VO(
                                ferment::FFIConversionTo::ffi_to(o_0),
                            )
                        }
                        _ => unreachable!("This is unreachable"),
                    })
                }
            }
            impl ferment::FFIConversionDestroy<example_nested::model::TestModLevelSnapshot>
                for example_nested_model_TestModLevelSnapshot
            {
                unsafe fn destroy(ffi: *mut example_nested_model_TestModLevelSnapshot) {
                    ferment::unbox_any(ffi);
                }
            }
            impl Drop for example_nested_model_TestModLevelSnapshot {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            example_nested_model_TestModLevelSnapshot::VO(o_0) => {
                                ferment::unbox_any(*o_0);
                            }
                            _ => unreachable!("This is unreachable"),
                        };
                    }
                }
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelSnapshot_VO_ctor(
                o_o_0 : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
            ) -> *mut example_nested_model_TestModLevelSnapshot {
                ferment::boxed(example_nested_model_TestModLevelSnapshot::VO(o_o_0))
            }
            #[no_mangle]
            pub unsafe extern "C" fn example_nested_model_TestModLevelSnapshot_destroy(
                ffi: *mut example_nested_model_TestModLevelSnapshot,
            ) {
                ferment::unbox_any(ffi);
            }
            pub mod tuples {
                use crate as example_nested;
                #[doc = "FFI-representation of the [`example_nested::model::tuples::get_tuple_simple`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_get_tuple_simple(
                ) -> *mut crate::fermented::generics::Tuple_u32_u32 {
                    let obj = example_nested::model::tuples::get_tuple_simple();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`example_nested::model::tuples::get_tuple_simple_complex`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_get_tuple_simple_complex(
                ) -> *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID
                {
                    let obj = example_nested::model::tuples::get_tuple_simple_complex();
                    ferment::FFIConversionTo::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`example_nested::model::tuples::get_hash_id_form_snapshot`]"]
                #[no_mangle]                pub unsafe extern "C" fn example_nested_model_tuples_get_hash_id_form_snapshot (_snapshot : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID{
                    let obj = example_nested::model::tuples::get_hash_id_form_snapshot(
                        ferment::FFIConversionFrom::ffi_from(_snapshot),
                    );
                    ferment::FFIConversionTo::ffi_to(obj)
                }
                #[doc = "FFI-representation of the [`example_nested::model::tuples::get_tuple_complex_complex`]"]
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_get_tuple_complex_complex(
                    tuple: *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID,
                ) -> u32 {
                    let obj = example_nested::model::tuples::get_tuple_complex_complex(
                        ferment::FFIConversionFrom::ffi_from(tuple),
                    );
                    obj
                }
                #[doc = "FFI-representation of the [`example_nested::model::tuples::StructWithTuple`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_model_tuples_StructWithTuple {
                    pub tuple:
                        *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID,
                }
                impl ferment::FFIConversionFrom<example_nested::model::tuples::StructWithTuple>
                    for example_nested_model_tuples_StructWithTuple
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_model_tuples_StructWithTuple,
                    ) -> example_nested::model::tuples::StructWithTuple {
                        let ffi_ref = &*ffi;
                        example_nested::model::tuples::StructWithTuple {
                            tuple: ferment::FFIConversionFrom::ffi_from(ffi_ref.tuple),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::model::tuples::StructWithTuple>
                    for example_nested_model_tuples_StructWithTuple
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::model::tuples::StructWithTuple,
                    ) -> *const example_nested_model_tuples_StructWithTuple {
                        ferment::boxed(example_nested_model_tuples_StructWithTuple {
                            tuple: ferment::FFIConversionTo::ffi_to(obj.tuple),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::model::tuples::StructWithTuple>
                    for example_nested_model_tuples_StructWithTuple
                {
                    unsafe fn destroy(ffi: *mut example_nested_model_tuples_StructWithTuple) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_model_tuples_StructWithTuple {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.tuple);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_StructWithTuple_ctor(
                    tuple: *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID,
                ) -> *mut example_nested_model_tuples_StructWithTuple {
                    ferment::boxed(example_nested_model_tuples_StructWithTuple { tuple })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_StructWithTuple_destroy(
                    ffi: *mut example_nested_model_tuples_StructWithTuple,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_StructWithTuple_get_tuple(
                    obj: *const example_nested_model_tuples_StructWithTuple,
                ) -> *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID
                {
                    (*obj).tuple
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_StructWithTuple_set_tuple(
                    obj: *const example_nested_model_tuples_StructWithTuple,
                ) -> *mut crate::fermented::generics::Tuple_u32_example_simple_nested_HashID
                {
                    (*obj).tuple
                }
                #[doc = "FFI-representation of the [`example_nested::model::tuples::TransUser`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct example_nested_model_tuples_TransUser { pub transition : * mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition }
                impl ferment::FFIConversionFrom<example_nested::model::tuples::TransUser>
                    for example_nested_model_tuples_TransUser
                {
                    unsafe fn ffi_from_const(
                        ffi: *const example_nested_model_tuples_TransUser,
                    ) -> example_nested::model::tuples::TransUser {
                        let ffi_ref = &*ffi;
                        example_nested::model::tuples::TransUser {
                            transition: ferment::FFIConversionFrom::ffi_from(ffi_ref.transition),
                        }
                    }
                }
                impl ferment::FFIConversionTo<example_nested::model::tuples::TransUser>
                    for example_nested_model_tuples_TransUser
                {
                    unsafe fn ffi_to_const(
                        obj: example_nested::model::tuples::TransUser,
                    ) -> *const example_nested_model_tuples_TransUser {
                        ferment::boxed(example_nested_model_tuples_TransUser {
                            transition: ferment::FFIConversionTo::ffi_to(obj.transition),
                        })
                    }
                }
                impl ferment::FFIConversionDestroy<example_nested::model::tuples::TransUser>
                    for example_nested_model_tuples_TransUser
                {
                    unsafe fn destroy(ffi: *mut example_nested_model_tuples_TransUser) {
                        ferment::unbox_any(ffi);
                    }
                }
                impl Drop for example_nested_model_tuples_TransUser {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment::unbox_any(ffi_ref.transition);
                        }
                    }
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_TransUser_ctor(
                    transition : * mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition,
                ) -> *mut example_nested_model_tuples_TransUser {
                    ferment::boxed(example_nested_model_tuples_TransUser { transition })
                }
                #[no_mangle]
                pub unsafe extern "C" fn example_nested_model_tuples_TransUser_destroy(
                    ffi: *mut example_nested_model_tuples_TransUser,
                ) {
                    ferment::unbox_any(ffi);
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_model_tuples_TransUser_get_transition < > (obj : * const example_nested_model_tuples_TransUser) -> * mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition{
                    (*obj).transition
                }
                #[no_mangle]                pub unsafe extern "C" fn example_nested_model_tuples_TransUser_set_transition < > (obj : * const example_nested_model_tuples_TransUser) -> * mut crate :: fermented :: types :: example_nested :: state_transition :: state_transitions :: contract :: data_contract_create_transition :: example_nested_state_transition_state_transitions_contract_data_contract_create_transition_DataContractCreateTransition{
                    (*obj).transition
                }
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
    unreachable_patterns,
    unused_braces,
    unused_imports,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables
)]
pub mod generics {
    use crate as example_nested;
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::generics::Vec_u8,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>>
        for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
        ) -> std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>>
        for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>,
        ) -> *const std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_opt_complex_group(obj.keys().cloned()),
                values: ferment::to_opt_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<Option<Vec<u8>>, Option<Vec<u8>>>>
        for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8_ctor(
        count: usize,
        keys: *mut *mut crate::fermented::generics::Vec_u8,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
        ferment::boxed(
            std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8 {
                count,
                keys,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_Option_Vec_u8_values_Option_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment::FFIConversionFrom<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner().expect("Err")),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>,
        > for std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::sync::Mutex<Option<String>>> for std_sync_Mutex_Option_String {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Mutex_Option_String,
        ) -> std::sync::Mutex<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Mutex<Option<String>>> for std_sync_Mutex_Option_String {
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<Option<String>>,
        ) -> *const std_sync_Mutex_Option_String {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Mutex<Option<String>>>
        for std_sync_Mutex_Option_String
    {
        unsafe fn destroy(ffi: *mut std_sync_Mutex_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_Mutex_Option_String {
        ferment::boxed(std_sync_Mutex_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_String_destroy(
        ffi: *mut std_sync_Mutex_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment::FFIConversionFrom<Vec<u32>> for Vec_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_u32) -> Vec<u32> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<u32>> for Vec_u32 {
        unsafe fn ffi_to_const(obj: Vec<u32>) -> *const Vec_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<u32>> for Vec_u32 {
        unsafe fn destroy(ffi: *mut Vec_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_u32 {
        type Value = Vec<u32>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_ctor(count: usize, values: *mut u32) -> *mut Vec_u32 {
        ferment::boxed(Vec_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u32_destroy(ffi: *mut Vec_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<Vec<Option<String>>> for Vec_Option_String {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_String) -> Vec<Option<String>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<Option<String>>> for Vec_Option_String {
        unsafe fn ffi_to_const(obj: Vec<Option<String>>) -> *const Vec_Option_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<Option<String>>> for Vec_Option_String {
        unsafe fn destroy(ffi: *mut Vec_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_Option_String {
        type Value = Vec<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Vec_Option_String {
        ferment::boxed(Vec_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_String_destroy(ffi: *mut Vec_Option_String) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Vec_u8 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, Vec<u8>>>
        for std_collections_Map_keys_u32_values_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Vec_u8,
        ) -> std::collections::BTreeMap<u32, Vec<u8>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, Vec<u8>>>
        for std_collections_Map_keys_u32_values_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Vec<u8>>,
        ) -> *const std_collections_Map_keys_u32_values_Vec_u8 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, Vec<u8>>>
        for std_collections_Map_keys_u32_values_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Vec_u8_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_u32_values_Vec_u8 {
        ferment::boxed(std_collections_Map_keys_u32_values_Vec_u8 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<Vec<Option<Vec<u8>>>> for Vec_Option_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_Vec_u8) -> Vec<Option<Vec<u8>>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<Option<Vec<u8>>>> for Vec_Option_Vec_u8 {
        unsafe fn ffi_to_const(obj: Vec<Option<Vec<u8>>>) -> *const Vec_Option_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<Option<Vec<u8>>>> for Vec_Option_Vec_u8 {
        unsafe fn destroy(ffi: *mut Vec_Option_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_Option_Vec_u8 {
        type Value = Vec<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Vec_Option_Vec_u8 {
        ferment::boxed(Vec_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_Vec_u8_destroy(ffi: *mut Vec_Option_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Option_String {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<Result<String, Option<String>>>
        for Result_ok_String_err_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_String,
        ) -> Result<String, Option<String>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                ffi_ref.error,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<Result<String, Option<String>>>
        for Result_ok_String_err_Option_String
    {
        unsafe fn ffi_to_const(
            obj: Result<String, Option<String>>,
        ) -> *const Result_ok_String_err_Option_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::FFIConversionTo::ffi_to(o),
                    |o| ferment::FFIConversionTo::ffi_to_opt(o),
                );
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<String, Option<String>>>
        for Result_ok_String_err_Option_String
    {
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_String_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut std::os::raw::c_char,
    ) -> *mut Result_ok_String_err_Option_String {
        ferment::boxed(Result_ok_String_err_Option_String { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_String_destroy(
        ffi: *mut Result_ok_String_err_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::sync::RwLock<Vec<u8>>> for std_sync_RwLock_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_Vec_u8) -> std::sync::RwLock<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::RwLock<Vec<u8>>> for std_sync_RwLock_Vec_u8 {
        unsafe fn ffi_to_const(obj: std::sync::RwLock<Vec<u8>>) -> *const std_sync_RwLock_Vec_u8 {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner().expect("Err")),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::RwLock<Vec<u8>>> for std_sync_RwLock_Vec_u8 {
        unsafe fn destroy(ffi: *mut std_sync_RwLock_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_RwLock_Vec_u8 {
        ferment::boxed(std_sync_RwLock_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Vec_u8_destroy(ffi: *mut std_sync_RwLock_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: Mutex :: new (ferment :: FFIConversionFrom :: ffi_from_opt (ffi_ref . obj)) } }
    impl ferment :: FFIConversionTo < std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to_opt (obj . into_inner () . expect ("Err")) }) } }
    impl ferment :: FFIConversionDestroy < std :: sync :: Mutex < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . obj) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_Mutex_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_u32 {
        pub obj: u32,
    }
    impl ferment::FFIConversionFrom<std::cell::RefCell<u32>> for std_cell_RefCell_u32 {
        unsafe fn ffi_from_const(ffi: *const std_cell_RefCell_u32) -> std::cell::RefCell<u32> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::cell::RefCell<u32>> for std_cell_RefCell_u32 {
        unsafe fn ffi_to_const(obj: std::cell::RefCell<u32>) -> *const std_cell_RefCell_u32 {
            ferment::boxed(Self {
                obj: obj.into_inner(),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::cell::RefCell<u32>> for std_cell_RefCell_u32 {
        unsafe fn destroy(ffi: *mut std_cell_RefCell_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_u32_ctor(obj: u32) -> *mut std_cell_RefCell_u32 {
        ferment::boxed(std_cell_RefCell_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_u32_destroy(ffi: *mut std_cell_RefCell_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_Vec_u8) -> std::sync::Mutex<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
        unsafe fn ffi_to_const(obj: std::sync::Mutex<Vec<u8>>) -> *const std_sync_Mutex_Vec_u8 {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner().expect("Err")),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Mutex<Vec<u8>>> for std_sync_Mutex_Vec_u8 {
        unsafe fn destroy(ffi: *mut std_sync_Mutex_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_Mutex_Vec_u8 {
        ferment::boxed(std_sync_Mutex_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Vec_u8_destroy(ffi: *mut std_sync_Mutex_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::rc::Rc<Vec<u8>>> for std_rc_Rc_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_rc_Rc_Vec_u8) -> std::rc::Rc<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::rc::Rc<Vec<u8>>> for std_rc_Rc_Vec_u8 {
        unsafe fn ffi_to_const(obj: std::rc::Rc<Vec<u8>>) -> *const std_rc_Rc_Vec_u8 {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to((*obj).clone()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::rc::Rc<Vec<u8>>> for std_rc_Rc_Vec_u8 {
        unsafe fn destroy(ffi: *mut std_rc_Rc_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_rc_Rc_Vec_u8 {
        ferment::boxed(std_rc_Rc_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Vec_u8_destroy(ffi: *mut std_rc_Rc_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode }
    impl ferment::FFIConversionFrom<Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode>>
        for Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode,
        ) -> Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode>>
        for Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        unsafe fn ffi_to_const(
            obj: Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode>,
        ) -> *const Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode>>
        for Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode
    {
        unsafe fn destroy(ffi: *mut Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        type Value = Vec<example_nested::model::snapshot::LLMQSnapshotSkipMode>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) -> *mut Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode {
        ferment::boxed(Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
        ffi: *mut Vec_example_nested_model_snapshot_LLMQSnapshotSkipMode,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_std_time_Duration {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut example_nested::std_time_Duration2,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<String, std::time::Duration>>
        for std_collections_Map_keys_String_values_std_time_Duration
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_std_time_Duration,
        ) -> std::collections::BTreeMap<String, std::time::Duration> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<String, std::time::Duration>>
        for std_collections_Map_keys_String_values_std_time_Duration
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<String, std::time::Duration>,
        ) -> *const std_collections_Map_keys_String_values_std_time_Duration {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_complex_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<String, std::time::Duration>>
        for std_collections_Map_keys_String_values_std_time_Duration
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_String_values_std_time_Duration) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_std_time_Duration {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_std_time_Duration_ctor(
        count: usize,
        keys: *mut *mut std::os::raw::c_char,
        values: *mut *mut example_nested::std_time_Duration2,
    ) -> *mut std_collections_Map_keys_String_values_std_time_Duration {
        ferment::boxed(std_collections_Map_keys_String_values_std_time_Duration {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_std_time_Duration_destroy(
        ffi: *mut std_collections_Map_keys_String_values_std_time_Duration,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel { caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: types :: example_nested :: entry :: example_nested_entry_SomeModel , destructor : unsafe extern "C" fn (* mut crate :: fermented :: types :: example_nested :: entry :: example_nested_entry_SomeModel) }
    impl Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel {
        pub unsafe fn call(&self, o_0: u32) -> example_nested::entry::SomeModel {
            let ffi_result = (self.caller)(o_0);
            let result = < crate :: fermented :: types :: example_nested :: entry :: example_nested_entry_SomeModel as ferment :: FFIConversionFrom < example_nested :: entry :: SomeModel >> :: ffi_from (ffi_result) ;
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send for Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel {}
    unsafe impl Sync for Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel_ctor(
        caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: types :: example_nested :: entry :: example_nested_entry_SomeModel,
        destructor: unsafe extern "C" fn(
            *mut crate::fermented::types::example_nested::entry::example_nested_entry_SomeModel,
        ),
    ) -> *mut Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel {
        ferment::boxed(Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel_destroy(
        ffi: *mut Fn_ARGS_u32_RTRN_example_nested_entry_SomeModel,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Arr_u8_32 {
        caller: unsafe extern "C" fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_u32_RTRN_Arr_u8_32 {
        pub unsafe fn call(&self, o_0: u32) -> [u8; 32] {
            let ffi_result = (self.caller)(o_0);
            let result = <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<
                [u8; 32],
            >>::ffi_from(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send for Fn_ARGS_u32_RTRN_Arr_u8_32 {}
    unsafe impl Sync for Fn_ARGS_u32_RTRN_Arr_u8_32 {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Arr_u8_32_ctor(
        caller: unsafe extern "C" fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    ) -> *mut Fn_ARGS_u32_RTRN_Arr_u8_32 {
        ferment::boxed(Fn_ARGS_u32_RTRN_Arr_u8_32 { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Arr_u8_32_destroy(
        ffi: *mut Fn_ARGS_u32_RTRN_Arr_u8_32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::sync::Arc<Option<String>>> for std_sync_Arc_Option_String {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_Option_String,
        ) -> std::sync::Arc<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Arc<Option<String>>> for std_sync_Arc_Option_String {
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<Option<String>>,
        ) -> *const std_sync_Arc_Option_String {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt((*obj).clone()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Arc<Option<String>>> for std_sync_Arc_Option_String {
        unsafe fn destroy(ffi: *mut std_sync_Arc_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_Arc_Option_String {
        ferment::boxed(std_sync_Arc_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_String_destroy(
        ffi: *mut std_sync_Arc_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_example_nested_model_Quorum_32 {
        pub count: usize,
        pub values:
            *mut *mut crate::fermented::types::example_nested::model::example_nested_model_Quorum,
    }
    impl ferment::FFIConversionFrom<[example_nested::model::Quorum; 32]>
        for Arr_example_nested_model_Quorum_32
    {
        unsafe fn ffi_from_const(
            ffi: *const Arr_example_nested_model_Quorum_32,
        ) -> [example_nested::model::Quorum; 32] {
            ferment::FFIVecConversion::decode(&*ffi).try_into().unwrap()
        }
    }
    impl ferment::FFIConversionTo<[example_nested::model::Quorum; 32]>
        for Arr_example_nested_model_Quorum_32
    {
        unsafe fn ffi_to_const(
            obj: [example_nested::model::Quorum; 32],
        ) -> *const Arr_example_nested_model_Quorum_32 {
            ferment::FFIVecConversion::encode(obj.to_vec())
        }
    }
    impl ferment::FFIConversionDestroy<[example_nested::model::Quorum; 32]>
        for Arr_example_nested_model_Quorum_32
    {
        unsafe fn destroy(ffi: *mut Arr_example_nested_model_Quorum_32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Arr_example_nested_model_Quorum_32 {
        type Value = Vec<example_nested::model::Quorum>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Arr_example_nested_model_Quorum_32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_example_nested_model_Quorum_32_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: example_nested :: model :: example_nested_model_Quorum,
    ) -> *mut Arr_example_nested_model_Quorum_32 {
        ferment::boxed(Arr_example_nested_model_Quorum_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_example_nested_model_Quorum_32_destroy(
        ffi: *mut Arr_example_nested_model_Quorum_32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<Option<u32>>>
        for std_collections_HashSet_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_u32,
        ) -> std::collections::HashSet<Option<u32>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<Option<u32>>>
        for std_collections_HashSet_Option_u32
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<u32>>,
        ) -> *const std_collections_HashSet_Option_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<Option<u32>>>
        for std_collections_HashSet_Option_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_Option_u32 {
        type Value = std::collections::HashSet<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut std_collections_HashSet_Option_u32 {
        ferment::boxed(std_collections_HashSet_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_u32_destroy(
        ffi: *mut std_collections_HashSet_Option_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_example_nested_model_snapshot_LLMQSnapshot { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl ferment::FFIConversionFrom<Vec<example_nested::model::snapshot::LLMQSnapshot>>
        for Vec_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const Vec_example_nested_model_snapshot_LLMQSnapshot,
        ) -> Vec<example_nested::model::snapshot::LLMQSnapshot> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<example_nested::model::snapshot::LLMQSnapshot>>
        for Vec_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: Vec<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const Vec_example_nested_model_snapshot_LLMQSnapshot {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<example_nested::model::snapshot::LLMQSnapshot>>
        for Vec_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut Vec_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_example_nested_model_snapshot_LLMQSnapshot {
        type Value = Vec<example_nested::model::snapshot::LLMQSnapshot>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_example_nested_model_snapshot_LLMQSnapshot_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut Vec_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(Vec_example_nested_model_snapshot_LLMQSnapshot { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut Vec_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl ferment :: FFIConversionFrom < Result < Option < std :: sync :: Arc < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) -> Result < Option < std :: sync :: Arc < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | ferment :: FFIConversionFrom :: ffi_from_opt (o) , ffi_ref . error , | o | ferment :: FFIConversionFrom :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < Result < Option < std :: sync :: Arc < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_to_const (obj : Result < Option < std :: sync :: Arc < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to_opt (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } }
    impl ferment :: FFIConversionDestroy < Result < Option < std :: sync :: Arc < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn destroy (ffi : * mut Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: generics :: std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
        ferment :: boxed (Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_u32_err_u32 {
        pub ok: *mut u32,
        pub error: *mut u32,
    }
    impl ferment::FFIConversionFrom<Result<u32, u32>> for Result_ok_u32_err_u32 {
        unsafe fn ffi_from_const(ffi: *const Result_ok_u32_err_u32) -> Result<u32, u32> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(ffi_ref.ok, |o| *o, ffi_ref.error, |o| *o)
        }
    }
    impl ferment::FFIConversionTo<Result<u32, u32>> for Result_ok_u32_err_u32 {
        unsafe fn ffi_to_const(obj: Result<u32, u32>) -> *const Result_ok_u32_err_u32 {
            ferment::boxed({
                let (ok, error) =
                    ferment::to_result(obj, |o| ferment::boxed(o), |o| ferment::boxed(o));
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<u32, u32>> for Result_ok_u32_err_u32 {
        unsafe fn destroy(ffi: *mut Result_ok_u32_err_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::destroy_opt_primitive(self.ok);
                ferment::destroy_opt_primitive(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_ctor(
        ok: *mut u32,
        error: *mut u32,
    ) -> *mut Result_ok_u32_err_u32 {
        ferment::boxed(Result_ok_u32_err_u32 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_u32_destroy(ffi: *mut Result_ok_u32_err_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: generics :: Vec_example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl ferment :: FFIConversionFrom < Result < Option < Vec < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) -> Result < Option < Vec < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | ferment :: FFIConversionFrom :: ffi_from_opt (o) , ffi_ref . error , | o | ferment :: FFIConversionFrom :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < Result < Option < Vec < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_to_const (obj : Result < Option < Vec < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to_opt (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } }
    impl ferment :: FFIConversionDestroy < Result < Option < Vec < example_nested :: model :: snapshot :: LLMQSnapshot > > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn destroy (ffi : * mut Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: generics :: Vec_example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
        ferment :: boxed (Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_Vec_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment::FFIConversionFrom<Vec<Option<u32>>> for Vec_Option_u32 {
        unsafe fn ffi_from_const(ffi: *const Vec_Option_u32) -> Vec<Option<u32>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<Option<u32>>> for Vec_Option_u32 {
        unsafe fn ffi_to_const(obj: Vec<Option<u32>>) -> *const Vec_Option_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<Option<u32>>> for Vec_Option_u32 {
        unsafe fn destroy(ffi: *mut Vec_Option_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_Option_u32 {
        type Value = Vec<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut Vec_Option_u32 {
        ferment::boxed(Vec_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Option_u32_destroy(ffi: *mut Vec_Option_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl ferment::FFIConversionFrom<std::sync::Arc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Arc<example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Arc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to((*obj).clone()),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<std::sync::Arc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Arc_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<Option<String>>>
        for std_collections_BTreeSet_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_String,
        ) -> std::collections::BTreeSet<Option<String>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<Option<String>>>
        for std_collections_BTreeSet_Option_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<String>>,
        ) -> *const std_collections_BTreeSet_Option_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<Option<String>>>
        for std_collections_BTreeSet_Option_String
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_Option_String {
        type Value = std::collections::BTreeSet<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_BTreeSet_Option_String {
        ferment::boxed(std_collections_BTreeSet_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_String_destroy(
        ffi: *mut std_collections_BTreeSet_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<Vec<u8>>>
        for std_collections_BTreeSet_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Vec_u8,
        ) -> std::collections::BTreeSet<Vec<u8>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<Vec<u8>>>
        for std_collections_BTreeSet_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Vec<u8>>,
        ) -> *const std_collections_BTreeSet_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<Vec<u8>>>
        for std_collections_BTreeSet_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_Vec_u8 {
        type Value = std::collections::BTreeSet<Vec<u8>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_BTreeSet_Vec_u8 {
        ferment::boxed(std_collections_BTreeSet_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Vec_u8_destroy(
        ffi: *mut std_collections_BTreeSet_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<Option<Vec<u8>>>>
        for std_collections_BTreeSet_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_Vec_u8,
        ) -> std::collections::BTreeSet<Option<Vec<u8>>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<Option<Vec<u8>>>>
        for std_collections_BTreeSet_Option_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<Vec<u8>>>,
        ) -> *const std_collections_BTreeSet_Option_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<Option<Vec<u8>>>>
        for std_collections_BTreeSet_Option_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_Option_Vec_u8 {
        type Value = std::collections::BTreeSet<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_BTreeSet_Option_Vec_u8 {
        ferment::boxed(std_collections_BTreeSet_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_Vec_u8_destroy(
        ffi: *mut std_collections_BTreeSet_Option_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_u32 {
        pub obj: u32,
    }
    impl ferment::FFIConversionFrom<std::sync::RwLock<u32>> for std_sync_RwLock_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_RwLock_u32) -> std::sync::RwLock<u32> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::sync::RwLock<u32>> for std_sync_RwLock_u32 {
        unsafe fn ffi_to_const(obj: std::sync::RwLock<u32>) -> *const std_sync_RwLock_u32 {
            ferment::boxed(Self {
                obj: obj.into_inner().expect("Err"),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::RwLock<u32>> for std_sync_RwLock_u32 {
        unsafe fn destroy(ffi: *mut std_sync_RwLock_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_u32_ctor(obj: u32) -> *mut std_sync_RwLock_u32 {
        ferment::boxed(std_sync_RwLock_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_u32_destroy(ffi: *mut std_sync_RwLock_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        caller: unsafe extern "C" fn(
            u32,
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    }
    impl Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub unsafe fn call(&self, o_0: u32, o_1: [u8; 32]) -> Option<String> {
            let ffi_result = (self.caller)(o_0, ferment::FFIConversionTo::ffi_to(o_1));
            if ffi_result.is_null() {
                None
            } else {
                let result =
                    <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from_opt(
                        ffi_result,
                    );
                (self.destructor)(ffi_result);
                result
            }
        }
    }
    unsafe impl Send for Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {}
    unsafe impl Sync for Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(
        caller: unsafe extern "C" fn(
            u32,
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    ) -> *mut Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        ferment::boxed(Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(
        ffi: *mut Fn_ARGS_u32_Arr_u8_32_RTRN_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::rc::Rc<Option<String>>> for std_rc_Rc_Option_String {
        unsafe fn ffi_from_const(
            ffi: *const std_rc_Rc_Option_String,
        ) -> std::rc::Rc<Option<String>> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::rc::Rc<Option<String>>> for std_rc_Rc_Option_String {
        unsafe fn ffi_to_const(obj: std::rc::Rc<Option<String>>) -> *const std_rc_Rc_Option_String {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt((*obj).clone()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::rc::Rc<Option<String>>> for std_rc_Rc_Option_String {
        unsafe fn destroy(ffi: *mut std_rc_Rc_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_rc_Rc_Option_String {
        ferment::boxed(std_rc_Rc_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_String_destroy(ffi: *mut std_rc_Rc_Option_String) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: Arc :: new (ferment :: FFIConversionFrom :: ffi_from_opt (ffi_ref . obj)) } }
    impl ferment :: FFIConversionTo < std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to_opt ((* obj) . clone ()) }) } }
    impl ferment :: FFIConversionDestroy < std :: sync :: Arc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . obj) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_Arc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<Vec<Vec<u8>>> for Vec_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_Vec_u8) -> Vec<Vec<u8>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<Vec<u8>>> for Vec_Vec_u8 {
        unsafe fn ffi_to_const(obj: Vec<Vec<u8>>) -> *const Vec_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<Vec<u8>>> for Vec_Vec_u8 {
        unsafe fn destroy(ffi: *mut Vec_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_Vec_u8 {
        type Value = Vec<Vec<u8>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Vec_Vec_u8 {
        ferment::boxed(Vec_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_Vec_u8_destroy(ffi: *mut Vec_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, String>>
        for std_collections_Map_keys_u32_values_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_String,
        ) -> std::collections::BTreeMap<u32, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, String>>
        for std_collections_Map_keys_u32_values_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, String>,
        ) -> *const std_collections_Map_keys_u32_values_String {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, String>>
        for std_collections_Map_keys_u32_values_String
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_u32_values_String {
        ferment::boxed(std_collections_Map_keys_u32_values_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        caller: unsafe extern "C" fn(
            u32,
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    }
    impl FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        pub unsafe fn call(&self, o_0: u32, o_1: [u8; 32]) -> Option<String> {
            let ffi_result = (self.caller)(o_0, ferment::FFIConversionTo::ffi_to(o_1));
            if ffi_result.is_null() {
                None
            } else {
                let result =
                    <std::os::raw::c_char as ferment::FFIConversionFrom<String>>::ffi_from_opt(
                        ffi_result,
                    );
                (self.destructor)(ffi_result);
                result
            }
        }
    }
    unsafe impl Send for FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {}
    unsafe impl Sync for FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {}
    #[no_mangle]
    pub unsafe extern "C" fn FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_ctor(
        caller: unsafe extern "C" fn(
            u32,
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut std::os::raw::c_char,
        destructor: unsafe extern "C" fn(*mut std::os::raw::c_char),
    ) -> *mut FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String {
        ferment::boxed(FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String_destroy(
        ffi: *mut FnMut_ARGS_u32_Arr_u8_32_RTRN_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_String_String {
        pub o_0: *mut std::os::raw::c_char,
        pub o_1: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<(String, String)> for Tuple_String_String {
        unsafe fn ffi_from_const(ffi: *const Tuple_String_String) -> (String, String) {
            let ffi_ref = &*ffi;
            (
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0),
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1),
            )
        }
    }
    impl ferment::FFIConversionTo<(String, String)> for Tuple_String_String {
        unsafe fn ffi_to_const(obj: (String, String)) -> *const Tuple_String_String {
            ferment::boxed(Self {
                o_0: ferment::FFIConversionTo::ffi_to(obj.0),
                o_1: ferment::FFIConversionTo::ffi_to(obj.1),
            })
        }
    }
    impl ferment::FFIConversionDestroy<(String, String)> for Tuple_String_String {
        unsafe fn destroy(ffi: *mut Tuple_String_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_String_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_String_ctor(
        o_0: *mut std::os::raw::c_char,
        o_1: *mut std::os::raw::c_char,
    ) -> *mut Tuple_String_String {
        ferment::boxed(Tuple_String_String { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_String_destroy(ffi: *mut Tuple_String_String) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<u32>> for std_collections_BTreeSet_u32 {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_u32,
        ) -> std::collections::BTreeSet<u32> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<u32>> for std_collections_BTreeSet_u32 {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<u32>,
        ) -> *const std_collections_BTreeSet_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<u32>>
        for std_collections_BTreeSet_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_u32 {
        type Value = std::collections::BTreeSet<u32>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_u32_ctor(
        count: usize,
        values: *mut u32,
    ) -> *mut std_collections_BTreeSet_u32 {
        ferment::boxed(std_collections_BTreeSet_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_u32_destroy(
        ffi: *mut std_collections_BTreeSet_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_Option_u32 {
        pub count: usize,
        pub values: *mut *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<Option<u32>>>
        for std_collections_BTreeSet_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_Option_u32,
        ) -> std::collections::BTreeSet<Option<u32>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<Option<u32>>>
        for std_collections_BTreeSet_Option_u32
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<Option<u32>>,
        ) -> *const std_collections_BTreeSet_Option_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<Option<u32>>>
        for std_collections_BTreeSet_Option_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_Option_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_Option_u32 {
        type Value = std::collections::BTreeSet<Option<u32>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_u32_ctor(
        count: usize,
        values: *mut *mut u32,
    ) -> *mut std_collections_BTreeSet_Option_u32 {
        ferment::boxed(std_collections_BTreeSet_Option_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_Option_u32_destroy(
        ffi: *mut std_collections_BTreeSet_Option_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::cell::RefCell<Option<String>>>
        for std_cell_RefCell_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_Option_String,
        ) -> std::cell::RefCell<Option<String>> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::cell::RefCell<Option<String>>>
        for std_cell_RefCell_Option_String
    {
        unsafe fn ffi_to_const(
            obj: std::cell::RefCell<Option<String>>,
        ) -> *const std_cell_RefCell_Option_String {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt(obj.into_inner()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::cell::RefCell<Option<String>>>
        for std_cell_RefCell_Option_String
    {
        unsafe fn destroy(ffi: *mut std_cell_RefCell_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_cell_RefCell_Option_String {
        ferment::boxed(std_cell_RefCell_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_String_destroy(
        ffi: *mut std_cell_RefCell_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { pub ok : * mut u32 , pub error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl
        ferment::FFIConversionFrom<
            Result<u32, example_simple::errors::protocol_error::ProtocolError>,
        > for Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
        ) -> Result<u32, example_simple::errors::protocol_error::ProtocolError> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| *o,
                ffi_ref.error,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl
        ferment::FFIConversionTo<Result<u32, example_simple::errors::protocol_error::ProtocolError>>
        for Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_to_const(
            obj: Result<u32, example_simple::errors::protocol_error::ProtocolError>,
        ) -> *const Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::boxed(o),
                    |o| ferment::FFIConversionTo::ffi_to(o),
                );
                Self { ok, error }
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            Result<u32, example_simple::errors::protocol_error::ProtocolError>,
        > for Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn destroy(
            ffi: *mut Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                ferment::destroy_opt_primitive(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_ctor(
        ok: *mut u32,
        error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError,
    ) -> *mut Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError {
        ferment::boxed(
            Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { ok, error },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi: *mut Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_u32 {
        pub obj: u32,
    }
    impl ferment::FFIConversionFrom<std::sync::Arc<u32>> for std_sync_Arc_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Arc_u32) -> std::sync::Arc<u32> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::sync::Arc<u32>> for std_sync_Arc_u32 {
        unsafe fn ffi_to_const(obj: std::sync::Arc<u32>) -> *const std_sync_Arc_u32 {
            ferment::boxed(Self { obj: *obj })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Arc<u32>> for std_sync_Arc_u32 {
        unsafe fn destroy(ffi: *mut std_sync_Arc_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_u32_ctor(obj: u32) -> *mut std_sync_Arc_u32 {
        ferment::boxed(std_sync_Arc_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_u32_destroy(ffi: *mut std_sync_Arc_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_String_32 {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<[String; 32]> for Arr_String_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_String_32) -> [String; 32] {
            ferment::FFIVecConversion::decode(&*ffi).try_into().unwrap()
        }
    }
    impl ferment::FFIConversionTo<[String; 32]> for Arr_String_32 {
        unsafe fn ffi_to_const(obj: [String; 32]) -> *const Arr_String_32 {
            ferment::FFIVecConversion::encode(obj.to_vec())
        }
    }
    impl ferment::FFIConversionDestroy<[String; 32]> for Arr_String_32 {
        unsafe fn destroy(ffi: *mut Arr_String_32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Arr_String_32 {
        type Value = Vec<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Arr_String_32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_String_32_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Arr_String_32 {
        ferment::boxed(Arr_String_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_String_32_destroy(ffi: *mut Arr_String_32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_String {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<String, String>>
        for std_collections_Map_keys_String_values_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_String,
        ) -> std::collections::BTreeMap<String, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<String, String>>
        for std_collections_Map_keys_String_values_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<String, String>,
        ) -> *const std_collections_Map_keys_String_values_String {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_complex_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<String, String>>
        for std_collections_Map_keys_String_values_String
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_String_values_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_ctor(
        count: usize,
        keys: *mut *mut std::os::raw::c_char,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_String_values_String {
        ferment::boxed(std_collections_Map_keys_String_values_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_String_destroy(
        ffi: *mut std_collections_Map_keys_String_values_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {
        caller: unsafe extern "C" fn(
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {
        pub unsafe fn call(&self, o_0: [u8; 32]) -> Option<[u8; 32]> {
            let ffi_result = (self.caller)(ferment::FFIConversionTo::ffi_to(o_0));
            let result = <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<
                [u8; 32],
            >>::ffi_from_opt(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send for Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {}
    unsafe impl Sync for Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_RTRN_Option_u8_ctor(
        caller: unsafe extern "C" fn(
            *mut crate::fermented::generics::Arr_u8_32,
        ) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    ) -> *mut Fn_ARGS_Arr_u8_32_RTRN_Option_u8 {
        ferment::boxed(Fn_ARGS_Arr_u8_32_RTRN_Option_u8 { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_RTRN_Option_u8_destroy(
        ffi: *mut Fn_ARGS_Arr_u8_32_RTRN_Option_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl ferment :: FFIConversionFrom < Result < example_nested :: model :: snapshot :: LLMQSnapshot , Option < example_simple :: errors :: protocol_error :: ProtocolError > > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError) -> Result < example_nested :: model :: snapshot :: LLMQSnapshot , Option < example_simple :: errors :: protocol_error :: ProtocolError > > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | ferment :: FFIConversionFrom :: ffi_from (o) , ffi_ref . error , | o | ferment :: FFIConversionFrom :: ffi_from_opt (o)) } }
    impl ferment :: FFIConversionTo < Result < example_nested :: model :: snapshot :: LLMQSnapshot , Option < example_simple :: errors :: protocol_error :: ProtocolError > > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_to_const (obj : Result < example_nested :: model :: snapshot :: LLMQSnapshot , Option < example_simple :: errors :: protocol_error :: ProtocolError > >) -> * const Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to (o) , | o | ferment :: FFIConversionTo :: ffi_to_opt (o)) ; Self { ok , error } }) } }
    impl ferment :: FFIConversionDestroy < Result < example_nested :: model :: snapshot :: LLMQSnapshot , Option < example_simple :: errors :: protocol_error :: ProtocolError > > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { unsafe fn destroy (ffi : * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError) -> * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError{
        ferment :: boxed (Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_Option_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError) }
    impl Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError {
        pub unsafe fn call(
            &self,
            o_0: u32,
        ) -> Result<u32, example_simple::errors::protocol_error::ProtocolError> {
            let ffi_result = (self.caller)(o_0);
            let result = < crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as ferment :: FFIConversionFrom < Result < u32 , example_simple :: errors :: protocol_error :: ProtocolError > >> :: ffi_from (ffi_result) ;
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send
        for Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
    }
    unsafe impl Sync
        for Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_ctor(
        caller : unsafe extern "C" fn (u32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
        destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError),
    ) -> *mut Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError
    {
        ferment::boxed(
            Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError {
                caller,
                destructor,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Fn_ARGS_u32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { pub ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < Result < example_nested :: model :: snapshot :: LLMQSnapshot , example_nested :: model :: snapshot :: LLMQSnapshot > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot) -> Result < example_nested :: model :: snapshot :: LLMQSnapshot , example_nested :: model :: snapshot :: LLMQSnapshot > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | ferment :: FFIConversionFrom :: ffi_from (o) , ffi_ref . error , | o | ferment :: FFIConversionFrom :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < Result < example_nested :: model :: snapshot :: LLMQSnapshot , example_nested :: model :: snapshot :: LLMQSnapshot > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : Result < example_nested :: model :: snapshot :: LLMQSnapshot , example_nested :: model :: snapshot :: LLMQSnapshot >) -> * const Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } }
    impl ferment :: FFIConversionDestroy < Result < example_nested :: model :: snapshot :: LLMQSnapshot , example_nested :: model :: snapshot :: LLMQSnapshot > > for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot_ctor (ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot) -> * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut Result_ok_example_nested_model_snapshot_LLMQSnapshot_err_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl ferment::FFIConversionFrom<std::rc::Rc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::rc::Rc<example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::rc::Rc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::rc::Rc<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to((*obj).clone()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::rc::Rc<example_nested::model::snapshot::LLMQSnapshot>>
        for std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_rc_Rc_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::sync::Arc<Vec<u8>>> for std_sync_Arc_Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Arc_Vec_u8) -> std::sync::Arc<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Arc<Vec<u8>>> for std_sync_Arc_Vec_u8 {
        unsafe fn ffi_to_const(obj: std::sync::Arc<Vec<u8>>) -> *const std_sync_Arc_Vec_u8 {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to((*obj).clone()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Arc<Vec<u8>>> for std_sync_Arc_Vec_u8 {
        unsafe fn destroy(ffi: *mut std_sync_Arc_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_sync_Arc_Vec_u8 {
        ferment::boxed(std_sync_Arc_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_Vec_u8_destroy(ffi: *mut std_sync_Arc_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, u32>>
        for std_collections_Map_keys_u32_values_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_u32,
        ) -> std::collections::BTreeMap<u32, u32> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(ffi_ref.count, ffi_ref.keys, ffi_ref.values, |o| o, |o| o)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, u32>>
        for std_collections_Map_keys_u32_values_u32
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, u32>,
        ) -> *const std_collections_Map_keys_u32_values_u32 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_primitive_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, u32>>
        for std_collections_Map_keys_u32_values_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut u32,
    ) -> *mut std_collections_Map_keys_u32_values_u32 {
        ferment::boxed(std_collections_Map_keys_u32_values_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_std_time_Duration_std_time_Duration {
        pub o_0: *mut example_nested::std_time_Duration2,
        pub o_1: *mut example_nested::std_time_Duration2,
    }
    impl ferment::FFIConversionFrom<(std::time::Duration, std::time::Duration)>
        for Tuple_std_time_Duration_std_time_Duration
    {
        unsafe fn ffi_from_const(
            ffi: *const Tuple_std_time_Duration_std_time_Duration,
        ) -> (std::time::Duration, std::time::Duration) {
            let ffi_ref = &*ffi;
            (
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0),
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1),
            )
        }
    }
    impl ferment::FFIConversionTo<(std::time::Duration, std::time::Duration)>
        for Tuple_std_time_Duration_std_time_Duration
    {
        unsafe fn ffi_to_const(
            obj: (std::time::Duration, std::time::Duration),
        ) -> *const Tuple_std_time_Duration_std_time_Duration {
            ferment::boxed(Self {
                o_0: ferment::FFIConversionTo::ffi_to(obj.0),
                o_1: ferment::FFIConversionTo::ffi_to(obj.1),
            })
        }
    }
    impl ferment::FFIConversionDestroy<(std::time::Duration, std::time::Duration)>
        for Tuple_std_time_Duration_std_time_Duration
    {
        unsafe fn destroy(ffi: *mut Tuple_std_time_Duration_std_time_Duration) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_std_time_Duration_std_time_Duration {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_std_time_Duration_std_time_Duration_ctor(
        o_0: *mut example_nested::std_time_Duration2,
        o_1: *mut example_nested::std_time_Duration2,
    ) -> *mut Tuple_std_time_Duration_std_time_Duration {
        ferment::boxed(Tuple_std_time_Duration_std_time_Duration { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_std_time_Duration_std_time_Duration_destroy(
        ffi: *mut Tuple_std_time_Duration_std_time_Duration,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub count : usize , pub keys : * mut u32 , pub values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment::FFIConversionFrom<
            std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>,
        > for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>
        {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl
        ferment::FFIConversionTo<
            std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>,
        > for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot
        {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::collections::BTreeMap<u32, example_nested::model::snapshot::LLMQSnapshot>,
        > for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(
            ffi : * mut std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor(
        count: usize,
        keys: *mut u32,
        values : * mut * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(
            std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot {
                count,
                keys,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_String_Vec_String {
        pub o_0: *mut std::os::raw::c_char,
        pub o_1: *mut crate::fermented::generics::Vec_String,
    }
    impl ferment::FFIConversionFrom<(String, Vec<String>)> for Tuple_String_Vec_String {
        unsafe fn ffi_from_const(ffi: *const Tuple_String_Vec_String) -> (String, Vec<String>) {
            let ffi_ref = &*ffi;
            (
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_0),
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1),
            )
        }
    }
    impl ferment::FFIConversionTo<(String, Vec<String>)> for Tuple_String_Vec_String {
        unsafe fn ffi_to_const(obj: (String, Vec<String>)) -> *const Tuple_String_Vec_String {
            ferment::boxed(Self {
                o_0: ferment::FFIConversionTo::ffi_to(obj.0),
                o_1: ferment::FFIConversionTo::ffi_to(obj.1),
            })
        }
    }
    impl ferment::FFIConversionDestroy<(String, Vec<String>)> for Tuple_String_Vec_String {
        unsafe fn destroy(ffi: *mut Tuple_String_Vec_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_String_Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_0);
                ferment::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_Vec_String_ctor(
        o_0: *mut std::os::raw::c_char,
        o_1: *mut crate::fermented::generics::Vec_String,
    ) -> *mut Tuple_String_Vec_String {
        ferment::boxed(Tuple_String_Vec_String { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_String_Vec_String_destroy(ffi: *mut Tuple_String_Vec_String) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Option_Vec_u8 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<Result<String, Option<Vec<u8>>>>
        for Result_ok_String_err_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_Vec_u8,
        ) -> Result<String, Option<Vec<u8>>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                ffi_ref.error,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<Result<String, Option<Vec<u8>>>>
        for Result_ok_String_err_Option_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: Result<String, Option<Vec<u8>>>,
        ) -> *const Result_ok_String_err_Option_Vec_u8 {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::FFIConversionTo::ffi_to(o),
                    |o| ferment::FFIConversionTo::ffi_to_opt(o),
                );
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<String, Option<Vec<u8>>>>
        for Result_ok_String_err_Option_Vec_u8
    {
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_Vec_u8_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Result_ok_String_err_Option_Vec_u8 {
        ferment::boxed(Result_ok_String_err_Option_Vec_u8 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_Vec_u8_destroy(
        ffi: *mut Result_ok_String_err_Option_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Vec_u8 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<Result<String, Vec<u8>>> for Result_ok_String_err_Vec_u8 {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Vec_u8,
        ) -> Result<String, Vec<u8>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                ffi_ref.error,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<Result<String, Vec<u8>>> for Result_ok_String_err_Vec_u8 {
        unsafe fn ffi_to_const(obj: Result<String, Vec<u8>>) -> *const Result_ok_String_err_Vec_u8 {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::FFIConversionTo::ffi_to(o),
                    |o| ferment::FFIConversionTo::ffi_to(o),
                );
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<String, Vec<u8>>> for Result_ok_String_err_Vec_u8 {
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Vec_u8_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Result_ok_String_err_Vec_u8 {
        ferment::boxed(Result_ok_String_err_Vec_u8 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Vec_u8_destroy(
        ffi: *mut Result_ok_String_err_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_Option_String {
        pub obj: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::sync::RwLock<Option<String>>>
        for std_sync_RwLock_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_RwLock_Option_String,
        ) -> std::sync::RwLock<Option<String>> {
            let ffi_ref = &*ffi;
            std::sync::RwLock::new(ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::RwLock<Option<String>>> for std_sync_RwLock_Option_String {
        unsafe fn ffi_to_const(
            obj: std::sync::RwLock<Option<String>>,
        ) -> *const std_sync_RwLock_Option_String {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::RwLock<Option<String>>>
        for std_sync_RwLock_Option_String
    {
        unsafe fn destroy(ffi: *mut std_sync_RwLock_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_RwLock_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_String_ctor(
        obj: *mut std::os::raw::c_char,
    ) -> *mut std_sync_RwLock_Option_String {
        ferment::boxed(std_sync_RwLock_Option_String { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_String_destroy(
        ffi: *mut std_sync_RwLock_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment::FFIConversionFrom<
            std::cell::RefCell<example_nested::model::snapshot::LLMQSnapshot>,
        > for std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::cell::RefCell<example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::cell::RefCell<example_nested::model::snapshot::LLMQSnapshot>>
        for std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::cell::RefCell<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner()),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::cell::RefCell<example_nested::model::snapshot::LLMQSnapshot>,
        > for std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_cell_RefCell_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, Option<u32>>>
        for std_collections_Map_keys_u32_values_Option_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_u32,
        ) -> std::collections::BTreeMap<u32, Option<u32>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::from_opt_primitive(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, Option<u32>>>
        for std_collections_Map_keys_u32_values_Option_u32
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<u32>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_u32 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_opt_primitive_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, Option<u32>>>
        for std_collections_Map_keys_u32_values_Option_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut u32,
    ) -> *mut std_collections_Map_keys_u32_values_Option_u32 {
        ferment::boxed(std_collections_Map_keys_u32_values_Option_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment::FFIConversionFrom<
            std::sync::Arc<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>,
        > for std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Arc<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>
        {
            let ffi_ref = &*ffi;
            std::sync::Arc::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl
        ferment::FFIConversionTo<
            std::sync::Arc<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>,
        > for std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::sync::Arc<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>,
        ) -> *const std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
        {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(std::sync::RwLock::new(
                    obj.read().expect("Poisoned").clone(),
                )),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::sync::Arc<std::sync::RwLock<example_nested::model::snapshot::LLMQSnapshot>>,
        > for std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(
            ffi: *mut std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: generics :: std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(
            std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot { obj },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Arc_std_sync_RwLock_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<Option<Vec<u8>>>>
        for std_collections_HashSet_Option_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_Vec_u8,
        ) -> std::collections::HashSet<Option<Vec<u8>>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<Option<Vec<u8>>>>
        for std_collections_HashSet_Option_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<Vec<u8>>>,
        ) -> *const std_collections_HashSet_Option_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<Option<Vec<u8>>>>
        for std_collections_HashSet_Option_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_Option_Vec_u8 {
        type Value = std::collections::HashSet<Option<Vec<u8>>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_HashSet_Option_Vec_u8 {
        ferment::boxed(std_collections_HashSet_Option_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_Vec_u8_destroy(
        ffi: *mut std_collections_HashSet_Option_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_Vec_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_String,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, Option<Vec<String>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_Vec_String,
        ) -> std::collections::BTreeMap<u32, Option<Vec<String>>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, Option<Vec<String>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<Vec<String>>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_Vec_String {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_opt_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, Option<Vec<String>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_String
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_String,
    ) -> *mut std_collections_Map_keys_u32_values_Option_Vec_String {
        ferment::boxed(std_collections_Map_keys_u32_values_Option_Vec_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_String {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<Result<String, String>> for Result_ok_String_err_String {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_String,
        ) -> Result<String, String> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                ffi_ref.error,
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<Result<String, String>> for Result_ok_String_err_String {
        unsafe fn ffi_to_const(obj: Result<String, String>) -> *const Result_ok_String_err_String {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::FFIConversionTo::ffi_to(o),
                    |o| ferment::FFIConversionTo::ffi_to(o),
                );
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<String, String>> for Result_ok_String_err_String {
        unsafe fn destroy(ffi: *mut Result_ok_String_err_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::unbox_any_opt(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_String_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut std::os::raw::c_char,
    ) -> *mut Result_ok_String_err_String {
        ferment::boxed(Result_ok_String_err_String { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_String_destroy(
        ffi: *mut Result_ok_String_err_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_String {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, Option<String>>>
        for std_collections_Map_keys_u32_values_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_String,
        ) -> std::collections::BTreeMap<u32, Option<String>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, Option<String>>>
        for std_collections_Map_keys_u32_values_Option_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<String>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_String {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_opt_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, Option<String>>>
        for std_collections_Map_keys_u32_values_Option_String
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_String_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_Map_keys_u32_values_Option_String {
        ferment::boxed(std_collections_Map_keys_u32_values_Option_String {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_String_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_String_values_example_simple_nested_HashID {
        pub count: usize,
        pub keys: *mut *mut std::os::raw::c_char,
        pub values:
            *mut *mut crate::fermented::types::example_simple::nested::example_simple_nested_HashID,
    }
    impl
        ferment::FFIConversionFrom<
            std::collections::BTreeMap<String, example_simple::nested::HashID>,
        > for std_collections_Map_keys_String_values_example_simple_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_String_values_example_simple_nested_HashID,
        ) -> std::collections::BTreeMap<String, example_simple::nested::HashID> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl
        ferment::FFIConversionTo<std::collections::BTreeMap<String, example_simple::nested::HashID>>
        for std_collections_Map_keys_String_values_example_simple_nested_HashID
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<String, example_simple::nested::HashID>,
        ) -> *const std_collections_Map_keys_String_values_example_simple_nested_HashID {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_complex_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::collections::BTreeMap<String, example_simple::nested::HashID>,
        > for std_collections_Map_keys_String_values_example_simple_nested_HashID
    {
        unsafe fn destroy(
            ffi: *mut std_collections_Map_keys_String_values_example_simple_nested_HashID,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_String_values_example_simple_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_example_simple_nested_HashID_ctor(
        count: usize,
        keys: *mut *mut std::os::raw::c_char,
        values : * mut * mut crate :: fermented :: types :: example_simple :: nested :: example_simple_nested_HashID,
    ) -> *mut std_collections_Map_keys_String_values_example_simple_nested_HashID {
        ferment::boxed(
            std_collections_Map_keys_String_values_example_simple_nested_HashID {
                count,
                keys,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_String_values_example_simple_nested_HashID_destroy(
        ffi: *mut std_collections_Map_keys_String_values_example_simple_nested_HashID,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_String_err_Option_u32 {
        pub ok: *mut std::os::raw::c_char,
        pub error: *mut u32,
    }
    impl ferment::FFIConversionFrom<Result<String, Option<u32>>> for Result_ok_String_err_Option_u32 {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_String_err_Option_u32,
        ) -> Result<String, Option<u32>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_result(
                ffi_ref.ok,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                ffi_ref.error,
                |o| ferment::from_opt_primitive(o),
            )
        }
    }
    impl ferment::FFIConversionTo<Result<String, Option<u32>>> for Result_ok_String_err_Option_u32 {
        unsafe fn ffi_to_const(
            obj: Result<String, Option<u32>>,
        ) -> *const Result_ok_String_err_Option_u32 {
            ferment::boxed({
                let (ok, error) = ferment::to_result(
                    obj,
                    |o| ferment::FFIConversionTo::ffi_to(o),
                    |o| ferment::to_opt_primitive(o),
                );
                Self { ok, error }
            })
        }
    }
    impl ferment::FFIConversionDestroy<Result<String, Option<u32>>>
        for Result_ok_String_err_Option_u32
    {
        unsafe fn destroy(ffi: *mut Result_ok_String_err_Option_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_String_err_Option_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.ok);
                ferment::destroy_opt_primitive(self.error);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_u32_ctor(
        ok: *mut std::os::raw::c_char,
        error: *mut u32,
    ) -> *mut Result_ok_String_err_Option_u32 {
        ferment::boxed(Result_ok_String_err_Option_u32 { ok, error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_String_err_Option_u32_destroy(
        ffi: *mut Result_ok_String_err_Option_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        pub count: usize,
        pub keys: *mut *mut crate::fermented::generics::Vec_u8,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<Vec<u8>, Vec<u8>>>
        for std_collections_Map_keys_Vec_u8_values_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_Vec_u8_values_Vec_u8,
        ) -> std::collections::BTreeMap<Vec<u8>, Vec<u8>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| ferment::FFIConversionFrom::ffi_from(o),
                |o| ferment::FFIConversionFrom::ffi_from(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<Vec<u8>, Vec<u8>>>
        for std_collections_Map_keys_Vec_u8_values_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<Vec<u8>, Vec<u8>>,
        ) -> *const std_collections_Map_keys_Vec_u8_values_Vec_u8 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_complex_group(obj.keys().cloned()),
                values: ferment::to_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<Vec<u8>, Vec<u8>>>
        for std_collections_Map_keys_Vec_u8_values_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_Vec_u8_values_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Vec_u8_values_Vec_u8_ctor(
        count: usize,
        keys: *mut *mut crate::fermented::generics::Vec_u8,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_Map_keys_Vec_u8_values_Vec_u8 {
        ferment::boxed(std_collections_Map_keys_Vec_u8_values_Vec_u8 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_Vec_u8_values_Vec_u8_destroy(
        ffi: *mut std_collections_Map_keys_Vec_u8_values_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_u32_u32 {
        pub o_0: u32,
        pub o_1: u32,
    }
    impl ferment::FFIConversionFrom<(u32, u32)> for Tuple_u32_u32 {
        unsafe fn ffi_from_const(ffi: *const Tuple_u32_u32) -> (u32, u32) {
            let ffi_ref = &*ffi;
            (ffi_ref.o_0, ffi_ref.o_1)
        }
    }
    impl ferment::FFIConversionTo<(u32, u32)> for Tuple_u32_u32 {
        unsafe fn ffi_to_const(obj: (u32, u32)) -> *const Tuple_u32_u32 {
            ferment::boxed(Self {
                o_0: obj.0,
                o_1: obj.1,
            })
        }
    }
    impl ferment::FFIConversionDestroy<(u32, u32)> for Tuple_u32_u32 {
        unsafe fn destroy(ffi: *mut Tuple_u32_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_u32_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_u32_ctor(o_0: u32, o_1: u32) -> *mut Tuple_u32_u32 {
        ferment::boxed(Tuple_u32_u32 { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_u32_destroy(ffi: *mut Tuple_u32_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_u32_RTRN_Option_u8 {
        caller: unsafe extern "C" fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    }
    impl Fn_ARGS_u32_RTRN_Option_u8 {
        pub unsafe fn call(&self, o_0: u32) -> Option<[u8; 32]> {
            let ffi_result = (self.caller)(o_0);
            let result = <crate::fermented::generics::Arr_u8_32 as ferment::FFIConversionFrom<
                [u8; 32],
            >>::ffi_from_opt(ffi_result);
            (self.destructor)(ffi_result);
            result
        }
    }
    unsafe impl Send for Fn_ARGS_u32_RTRN_Option_u8 {}
    unsafe impl Sync for Fn_ARGS_u32_RTRN_Option_u8 {}
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Option_u8_ctor(
        caller: unsafe extern "C" fn(u32) -> *mut crate::fermented::generics::Arr_u8_32,
        destructor: unsafe extern "C" fn(*mut crate::fermented::generics::Arr_u8_32),
    ) -> *mut Fn_ARGS_u32_RTRN_Option_u8 {
        ferment::boxed(Fn_ARGS_u32_RTRN_Option_u8 { caller, destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_u32_RTRN_Option_u8_destroy(
        ffi: *mut Fn_ARGS_u32_RTRN_Option_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_u32 {
        pub obj: u32,
    }
    impl ferment::FFIConversionFrom<std::sync::Mutex<u32>> for std_sync_Mutex_u32 {
        unsafe fn ffi_from_const(ffi: *const std_sync_Mutex_u32) -> std::sync::Mutex<u32> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::sync::Mutex<u32>> for std_sync_Mutex_u32 {
        unsafe fn ffi_to_const(obj: std::sync::Mutex<u32>) -> *const std_sync_Mutex_u32 {
            ferment::boxed(Self {
                obj: obj.into_inner().expect("Err"),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::sync::Mutex<u32>> for std_sync_Mutex_u32 {
        unsafe fn destroy(ffi: *mut std_sync_Mutex_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_u32_ctor(obj: u32) -> *mut std_sync_Mutex_u32 {
        ferment::boxed(std_sync_Mutex_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_u32_destroy(ffi: *mut std_sync_Mutex_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Vec_u8 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<Vec<u8>>>
        for std_collections_HashSet_Vec_u8
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Vec_u8,
        ) -> std::collections::HashSet<Vec<u8>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<Vec<u8>>>
        for std_collections_HashSet_Vec_u8
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Vec<u8>>,
        ) -> *const std_collections_HashSet_Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<Vec<u8>>>
        for std_collections_HashSet_Vec_u8
    {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_Vec_u8 {
        type Value = std::collections::HashSet<Vec<u8>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Vec_u8_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_collections_HashSet_Vec_u8 {
        ferment::boxed(std_collections_HashSet_Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Vec_u8_destroy(
        ffi: *mut std_collections_HashSet_Vec_u8,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_Vec_u8_32 {
        pub count: usize,
        pub values: *mut *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<[Vec<u8>; 32]> for Arr_Vec_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_Vec_u8_32) -> [Vec<u8>; 32] {
            ferment::FFIVecConversion::decode(&*ffi).try_into().unwrap()
        }
    }
    impl ferment::FFIConversionTo<[Vec<u8>; 32]> for Arr_Vec_u8_32 {
        unsafe fn ffi_to_const(obj: [Vec<u8>; 32]) -> *const Arr_Vec_u8_32 {
            ferment::FFIVecConversion::encode(obj.to_vec())
        }
    }
    impl ferment::FFIConversionDestroy<[Vec<u8>; 32]> for Arr_Vec_u8_32 {
        unsafe fn destroy(ffi: *mut Arr_Vec_u8_32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Arr_Vec_u8_32 {
        type Value = Vec<Vec<u8>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Arr_Vec_u8_32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_Vec_u8_32_ctor(
        count: usize,
        values: *mut *mut crate::fermented::generics::Vec_u8,
    ) -> *mut Arr_Vec_u8_32 {
        ferment::boxed(Arr_Vec_u8_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_Vec_u8_32_destroy(ffi: *mut Arr_Vec_u8_32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: sync :: RwLock :: new (ferment :: FFIConversionFrom :: ffi_from_opt (ffi_ref . obj)) } }
    impl ferment :: FFIConversionTo < std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to_opt (obj . into_inner () . expect ("Err")) }) } }
    impl ferment :: FFIConversionDestroy < std :: sync :: RwLock < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . obj) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_sync_RwLock_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<Option<String>>>
        for std_collections_HashSet_Option_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_Option_String,
        ) -> std::collections::HashSet<Option<String>> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<Option<String>>>
        for std_collections_HashSet_Option_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<Option<String>>,
        ) -> *const std_collections_HashSet_Option_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<Option<String>>>
        for std_collections_HashSet_Option_String
    {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_Option_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_Option_String {
        type Value = std::collections::HashSet<Option<String>>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_HashSet_Option_String {
        ferment::boxed(std_collections_HashSet_Option_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_String_destroy(
        ffi: *mut std_collections_HashSet_Option_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_u32 {
        pub count: usize,
        pub values: *mut u32,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<u32>> for std_collections_HashSet_u32 {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_u32,
        ) -> std::collections::HashSet<u32> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<u32>> for std_collections_HashSet_u32 {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<u32>,
        ) -> *const std_collections_HashSet_u32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<u32>> for std_collections_HashSet_u32 {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_u32 {
        type Value = std::collections::HashSet<u32>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_u32_ctor(
        count: usize,
        values: *mut u32,
    ) -> *mut std_collections_HashSet_u32 {
        ferment::boxed(std_collections_HashSet_u32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_u32_destroy(
        ffi: *mut std_collections_HashSet_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_u32 {
        pub obj: u32,
    }
    impl ferment::FFIConversionFrom<std::rc::Rc<u32>> for std_rc_Rc_u32 {
        unsafe fn ffi_from_const(ffi: *const std_rc_Rc_u32) -> std::rc::Rc<u32> {
            let ffi_ref = &*ffi;
            std::rc::Rc::new(ffi_ref.obj)
        }
    }
    impl ferment::FFIConversionTo<std::rc::Rc<u32>> for std_rc_Rc_u32 {
        unsafe fn ffi_to_const(obj: std::rc::Rc<u32>) -> *const std_rc_Rc_u32 {
            ferment::boxed(Self { obj: *obj })
        }
    }
    impl ferment::FFIConversionDestroy<std::rc::Rc<u32>> for std_rc_Rc_u32 {
        unsafe fn destroy(ffi: *mut std_rc_Rc_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_rc_Rc_u32 {
        fn drop(&mut self) {
            unsafe {}
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_u32_ctor(obj: u32) -> *mut std_rc_Rc_u32 {
        ferment::boxed(std_rc_Rc_u32 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_u32_destroy(ffi: *mut std_rc_Rc_u32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<Vec<String>> for Vec_String {
        unsafe fn ffi_from_const(ffi: *const Vec_String) -> Vec<String> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<String>> for Vec_String {
        unsafe fn ffi_to_const(obj: Vec<String>) -> *const Vec_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<String>> for Vec_String {
        unsafe fn destroy(ffi: *mut Vec_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_String {
        type Value = Vec<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut Vec_String {
        ferment::boxed(Vec_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_String_destroy(ffi: *mut Vec_String) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: rc :: Rc :: new (ferment :: FFIConversionFrom :: ffi_from_opt (ffi_ref . obj)) } }
    impl ferment :: FFIConversionTo < std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to_opt ((* obj) . clone ()) }) } }
    impl ferment :: FFIConversionDestroy < std :: rc :: Rc < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . obj) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_rc_Rc_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_i32 {
        pub count: usize,
        pub values: *mut i32,
    }
    impl ferment::FFIConversionFrom<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_from_const(ffi: *const Vec_i32) -> Vec<i32> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<i32>> for Vec_i32 {
        unsafe fn ffi_to_const(obj: Vec<i32>) -> *const Vec_i32 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<i32>> for Vec_i32 {
        unsafe fn destroy(ffi: *mut Vec_i32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_i32 {
        type Value = Vec<i32>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_i32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(count: usize, values: *mut i32) -> *mut Vec_i32 {
        ferment::boxed(Vec_i32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_Map_keys_u32_values_Option_Vec_u32 {
        pub count: usize,
        pub keys: *mut u32,
        pub values: *mut *mut crate::fermented::generics::Vec_u32,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeMap<u32, Option<Vec<u32>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_u32
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_Map_keys_u32_values_Option_Vec_u32,
        ) -> std::collections::BTreeMap<u32, Option<Vec<u32>>> {
            let ffi_ref = &*ffi;
            ferment::fold_to_map(
                ffi_ref.count,
                ffi_ref.keys,
                ffi_ref.values,
                |o| o,
                |o| ferment::FFIConversionFrom::ffi_from_opt(o),
            )
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeMap<u32, Option<Vec<u32>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_u32
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeMap<u32, Option<Vec<u32>>>,
        ) -> *const std_collections_Map_keys_u32_values_Option_Vec_u32 {
            ferment::boxed(Self {
                count: obj.len(),
                keys: ferment::to_primitive_group(obj.keys().cloned()),
                values: ferment::to_opt_complex_group(obj.values().cloned()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeMap<u32, Option<Vec<u32>>>>
        for std_collections_Map_keys_u32_values_Option_Vec_u32
    {
        unsafe fn destroy(ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_u32) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_collections_Map_keys_u32_values_Option_Vec_u32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.keys, self.count);
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_u32_ctor(
        count: usize,
        keys: *mut u32,
        values: *mut *mut crate::fermented::generics::Vec_u32,
    ) -> *mut std_collections_Map_keys_u32_values_Option_Vec_u32 {
        ferment::boxed(std_collections_Map_keys_u32_values_Option_Vec_u32 {
            count,
            keys,
            values,
        })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_Map_keys_u32_values_Option_Vec_u32_destroy(
        ffi: *mut std_collections_Map_keys_u32_values_Option_Vec_u32,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot }
    impl ferment :: FFIConversionFrom < std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_from_const (ffi : * const std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > { let ffi_ref = & * ffi ; std :: cell :: RefCell :: new (ferment :: FFIConversionFrom :: ffi_from_opt (ffi_ref . obj)) } }
    impl ferment :: FFIConversionTo < std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn ffi_to_const (obj : std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > >) -> * const std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { ferment :: boxed (Self { obj : ferment :: FFIConversionTo :: ffi_to_opt (obj . into_inner ()) }) } }
    impl ferment :: FFIConversionDestroy < std :: cell :: RefCell < Option < std :: collections :: BTreeMap < u32 , example_nested :: model :: snapshot :: LLMQSnapshot > > > > for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { unsafe fn destroy (ffi : * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . obj) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_ctor (obj : * mut crate :: fermented :: generics :: std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot) -> * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot{
        ferment :: boxed (std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi : * mut std_cell_RefCell_Option_std_collections_Map_keys_u32_values_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl ferment::FFIConversionFrom<std::sync::Mutex<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Mutex<example_nested::model::snapshot::LLMQSnapshot> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::sync::Mutex<example_nested::model::snapshot::LLMQSnapshot>>
        for std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<example_nested::model::snapshot::LLMQSnapshot>,
        ) -> *const std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner().expect("Err")),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::sync::Mutex<example_nested::model::snapshot::LLMQSnapshot>,
        > for std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(ffi: *mut std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Mutex_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::HashSet<String>>
        for std_collections_HashSet_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_HashSet_String,
        ) -> std::collections::HashSet<String> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::HashSet<String>>
        for std_collections_HashSet_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<String>,
        ) -> *const std_collections_HashSet_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::HashSet<String>>
        for std_collections_HashSet_String
    {
        unsafe fn destroy(ffi: *mut std_collections_HashSet_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_HashSet_String {
        type Value = std::collections::HashSet<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_HashSet_String {
        ferment::boxed(std_collections_HashSet_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_String_destroy(
        ffi: *mut std_collections_HashSet_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError { pub count : usize , pub values : * mut * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl
        ferment::FFIConversionFrom<
            std::collections::HashSet<
                Option<example_simple::errors::protocol_error::ProtocolError>,
            >,
        > for std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_from_const(
            ffi : * const std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError,
        ) -> std::collections::HashSet<Option<example_simple::errors::protocol_error::ProtocolError>>
        {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl
        ferment::FFIConversionTo<
            std::collections::HashSet<
                Option<example_simple::errors::protocol_error::ProtocolError>,
            >,
        > for std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn ffi_to_const(
            obj: std::collections::HashSet<
                Option<example_simple::errors::protocol_error::ProtocolError>,
            >,
        ) -> *const std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
        {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::collections::HashSet<
                Option<example_simple::errors::protocol_error::ProtocolError>,
            >,
        > for std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
    {
        unsafe fn destroy(
            ffi : * mut std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion
        for std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
    {
        type Value = std::collections::HashSet<
            Option<example_simple::errors::protocol_error::ProtocolError>,
        >;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_opt_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_opt_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError_ctor(
        count: usize,
        values : * mut * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError,
    ) -> *mut std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError
    {
        ferment::boxed(
            std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError {
                count,
                values,
            },
        )
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi: *mut std_collections_HashSet_Option_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Tuple_u32_example_simple_nested_HashID {
        pub o_0: u32,
        pub o_1: *mut crate::fermented::types::example_simple::nested::example_simple_nested_HashID,
    }
    impl ferment::FFIConversionFrom<(u32, example_simple::nested::HashID)>
        for Tuple_u32_example_simple_nested_HashID
    {
        unsafe fn ffi_from_const(
            ffi: *const Tuple_u32_example_simple_nested_HashID,
        ) -> (u32, example_simple::nested::HashID) {
            let ffi_ref = &*ffi;
            (
                ffi_ref.o_0,
                ferment::FFIConversionFrom::ffi_from(ffi_ref.o_1),
            )
        }
    }
    impl ferment::FFIConversionTo<(u32, example_simple::nested::HashID)>
        for Tuple_u32_example_simple_nested_HashID
    {
        unsafe fn ffi_to_const(
            obj: (u32, example_simple::nested::HashID),
        ) -> *const Tuple_u32_example_simple_nested_HashID {
            ferment::boxed(Self {
                o_0: obj.0,
                o_1: ferment::FFIConversionTo::ffi_to(obj.1),
            })
        }
    }
    impl ferment::FFIConversionDestroy<(u32, example_simple::nested::HashID)>
        for Tuple_u32_example_simple_nested_HashID
    {
        unsafe fn destroy(ffi: *mut Tuple_u32_example_simple_nested_HashID) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for Tuple_u32_example_simple_nested_HashID {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.o_1);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_example_simple_nested_HashID_ctor(
        o_0: u32,
        o_1: *mut crate::fermented::types::example_simple::nested::example_simple_nested_HashID,
    ) -> *mut Tuple_u32_example_simple_nested_HashID {
        ferment::boxed(Tuple_u32_example_simple_nested_HashID { o_0, o_1 })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Tuple_u32_example_simple_nested_HashID_destroy(
        ffi: *mut Tuple_u32_example_simple_nested_HashID,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Vec_u8 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_from_const(ffi: *const Vec_u8) -> Vec<u8> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<Vec<u8>> for Vec_u8 {
        unsafe fn ffi_to_const(obj: Vec<u8>) -> *const Vec_u8 {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<Vec<u8>> for Vec_u8 {
        unsafe fn destroy(ffi: *mut Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Vec_u8 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment::boxed(Vec_u8 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { pub ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , pub error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError }
    impl ferment :: FFIConversionFrom < Result < Option < example_nested :: model :: snapshot :: LLMQSnapshot > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) -> Result < Option < example_nested :: model :: snapshot :: LLMQSnapshot > , example_simple :: errors :: protocol_error :: ProtocolError > { let ffi_ref = & * ffi ; ferment :: fold_to_result (ffi_ref . ok , | o | ferment :: FFIConversionFrom :: ffi_from_opt (o) , ffi_ref . error , | o | ferment :: FFIConversionFrom :: ffi_from (o)) } }
    impl ferment :: FFIConversionTo < Result < Option < example_nested :: model :: snapshot :: LLMQSnapshot > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn ffi_to_const (obj : Result < Option < example_nested :: model :: snapshot :: LLMQSnapshot > , example_simple :: errors :: protocol_error :: ProtocolError >) -> * const Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ferment :: boxed ({ let (ok , error) = ferment :: to_result (obj , | o | ferment :: FFIConversionTo :: ffi_to_opt (o) , | o | ferment :: FFIConversionTo :: ffi_to (o)) ; Self { ok , error } }) } }
    impl ferment :: FFIConversionDestroy < Result < Option < example_nested :: model :: snapshot :: LLMQSnapshot > , example_simple :: errors :: protocol_error :: ProtocolError > > for Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { unsafe fn destroy (ffi : * mut Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError) { ferment :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { fn drop (& mut self) { unsafe { ferment :: unbox_any_opt (self . ok) ; ferment :: unbox_any_opt (self . error) ; } } }
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot , error : * mut crate :: fermented :: types :: example_simple :: errors :: protocol_error :: example_simple_errors_protocol_error_ProtocolError) -> * mut Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError{
        ferment :: boxed (Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError { ok , error })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_example_nested_model_snapshot_LLMQSnapshot_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_collections_BTreeSet_String {
        pub count: usize,
        pub values: *mut *mut std::os::raw::c_char,
    }
    impl ferment::FFIConversionFrom<std::collections::BTreeSet<String>>
        for std_collections_BTreeSet_String
    {
        unsafe fn ffi_from_const(
            ffi: *const std_collections_BTreeSet_String,
        ) -> std::collections::BTreeSet<String> {
            ferment::FFIVecConversion::decode(&*ffi)
        }
    }
    impl ferment::FFIConversionTo<std::collections::BTreeSet<String>>
        for std_collections_BTreeSet_String
    {
        unsafe fn ffi_to_const(
            obj: std::collections::BTreeSet<String>,
        ) -> *const std_collections_BTreeSet_String {
            ferment::FFIVecConversion::encode(obj)
        }
    }
    impl ferment::FFIConversionDestroy<std::collections::BTreeSet<String>>
        for std_collections_BTreeSet_String
    {
        unsafe fn destroy(ffi: *mut std_collections_BTreeSet_String) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for std_collections_BTreeSet_String {
        type Value = std::collections::BTreeSet<String>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_complex_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_complex_group(obj.into_iter()),
            })
        }
    }
    impl Drop for std_collections_BTreeSet_String {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_ctor(
        count: usize,
        values: *mut *mut std::os::raw::c_char,
    ) -> *mut std_collections_BTreeSet_String {
        ferment::boxed(std_collections_BTreeSet_String { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_collections_BTreeSet_String_destroy(
        ffi: *mut std_collections_BTreeSet_String,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_cell_RefCell_Vec_u8 {
        pub obj: *mut crate::fermented::generics::Vec_u8,
    }
    impl ferment::FFIConversionFrom<std::cell::RefCell<Vec<u8>>> for std_cell_RefCell_Vec_u8 {
        unsafe fn ffi_from_const(
            ffi: *const std_cell_RefCell_Vec_u8,
        ) -> std::cell::RefCell<Vec<u8>> {
            let ffi_ref = &*ffi;
            std::cell::RefCell::new(ferment::FFIConversionFrom::ffi_from(ffi_ref.obj))
        }
    }
    impl ferment::FFIConversionTo<std::cell::RefCell<Vec<u8>>> for std_cell_RefCell_Vec_u8 {
        unsafe fn ffi_to_const(obj: std::cell::RefCell<Vec<u8>>) -> *const std_cell_RefCell_Vec_u8 {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to(obj.into_inner()),
            })
        }
    }
    impl ferment::FFIConversionDestroy<std::cell::RefCell<Vec<u8>>> for std_cell_RefCell_Vec_u8 {
        unsafe fn destroy(ffi: *mut std_cell_RefCell_Vec_u8) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_cell_RefCell_Vec_u8 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Vec_u8_ctor(
        obj: *mut crate::fermented::generics::Vec_u8,
    ) -> *mut std_cell_RefCell_Vec_u8 {
        ferment::boxed(std_cell_RefCell_Vec_u8 { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_cell_RefCell_Vec_u8_destroy(ffi: *mut std_cell_RefCell_Vec_u8) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Arr_u8_32 {
        pub count: usize,
        pub values: *mut u8,
    }
    impl ferment::FFIConversionFrom<[u8; 32]> for Arr_u8_32 {
        unsafe fn ffi_from_const(ffi: *const Arr_u8_32) -> [u8; 32] {
            ferment::FFIVecConversion::decode(&*ffi).try_into().unwrap()
        }
    }
    impl ferment::FFIConversionTo<[u8; 32]> for Arr_u8_32 {
        unsafe fn ffi_to_const(obj: [u8; 32]) -> *const Arr_u8_32 {
            ferment::FFIVecConversion::encode(obj.to_vec())
        }
    }
    impl ferment::FFIConversionDestroy<[u8; 32]> for Arr_u8_32 {
        unsafe fn destroy(ffi: *mut Arr_u8_32) {
            ferment::unbox_any(ffi);
        }
    }
    impl ferment::FFIVecConversion for Arr_u8_32 {
        type Value = Vec<u8>;
        unsafe fn decode(&self) -> Self::Value {
            ferment::from_primitive_group(self.values, self.count)
        }
        unsafe fn encode(obj: Self::Value) -> *mut Self {
            ferment::boxed(Self {
                count: obj.len(),
                values: ferment::to_primitive_group(obj.into_iter()),
            })
        }
    }
    impl Drop for Arr_u8_32 {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_vec_ptr(self.values, self.count);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_ctor(count: usize, values: *mut u8) -> *mut Arr_u8_32 {
        ferment::boxed(Arr_u8_32 { count, values })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Arr_u8_32_destroy(ffi: *mut Arr_u8_32) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { caller : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32 , * mut crate :: fermented :: generics :: Arr_u8_32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError) }
    impl Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { pub unsafe fn call (& self , o_0 : [u8 ; 32] , o_1 : [u8 ; 32]) -> Result < u32 , example_simple :: errors :: protocol_error :: ProtocolError > { let ffi_result = (self . caller) (ferment :: FFIConversionTo :: ffi_to (o_0) , ferment :: FFIConversionTo :: ffi_to (o_1)) ; let result = < crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError as ferment :: FFIConversionFrom < Result < u32 , example_simple :: errors :: protocol_error :: ProtocolError > >> :: ffi_from (ffi_result) ; (self . destructor) (ffi_result) ; result } }
    unsafe impl Send for Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { }
    unsafe impl Sync for Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { }
    #[no_mangle]    pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_ctor (caller : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Arr_u8_32 , * mut crate :: fermented :: generics :: Arr_u8_32) -> * mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError , destructor : unsafe extern "C" fn (* mut crate :: fermented :: generics :: Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError)) -> * mut Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError{
        ferment :: boxed (Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError { caller , destructor })
    }
    #[no_mangle]
    pub unsafe extern "C" fn Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError_destroy(
        ffi : * mut Fn_ARGS_Arr_u8_32_Arr_u8_32_RTRN_Result_ok_u32_err_example_simple_errors_protocol_error_ProtocolError,
    ) {
        ferment::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot { pub obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot }
    impl
        ferment::FFIConversionFrom<
            std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>,
        > for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_from_const(
            ffi: *const std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
        ) -> std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>> {
            let ffi_ref = &*ffi;
            std::sync::Mutex::new(
                ferment::FFIConversionFrom::ffi_from_opt(ffi_ref.obj).map(Box::new),
            )
        }
    }
    impl
        ferment::FFIConversionTo<
            std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>,
        > for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn ffi_to_const(
            obj: std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>,
        ) -> *const std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
            ferment::boxed(Self {
                obj: ferment::FFIConversionTo::ffi_to_opt(obj.into_inner().expect("Err")),
            })
        }
    }
    impl
        ferment::FFIConversionDestroy<
            std::sync::Mutex<Option<Box<example_nested::model::snapshot::LLMQSnapshot>>>,
        > for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot
    {
        unsafe fn destroy(
            ffi: *mut std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
        ) {
            ferment::unbox_any(ffi);
        }
    }
    impl Drop for std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
        fn drop(&mut self) {
            unsafe {
                ferment::unbox_any_opt(self.obj);
            }
        }
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot_ctor(
        obj : * mut crate :: fermented :: types :: example_nested :: model :: snapshot :: example_nested_model_snapshot_LLMQSnapshot,
    ) -> *mut std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot {
        ferment::boxed(std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot { obj })
    }
    #[no_mangle]
    pub unsafe extern "C" fn std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot_destroy(
        ffi: *mut std_sync_Mutex_Option_Box_example_nested_model_snapshot_LLMQSnapshot,
    ) {
        ferment::unbox_any(ffi);
    }
}
