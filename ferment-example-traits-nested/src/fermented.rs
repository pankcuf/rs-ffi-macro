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
    pub mod ferment_example_traits {
        pub mod nested {
            #[doc = "FFI-representation of the [`address_with_script_pubkey`]"]
            #[doc = r" # Safety"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_traits_nested_address_with_script_pubkey(
                script: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut std::os::raw::c_char {
                let obj = ferment_example_traits::nested::address_with_script_pubkey(
                    ferment_interfaces::FFIConversion::ffi_from(script),
                );
                ferment_interfaces::FFIConversion::ffi_to_opt(obj)
            }
            #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_traits::nested::ProtocolError`]\"`]"]
            #[repr(C)]
            #[derive(Clone)]
            pub enum ferment_example_traits_nested_ProtocolError {
                IdentifierError(*mut std::os::raw::c_char),
                Unknown(*mut crate::fermented::generics::Vec_u8),
            }
            impl ferment_interfaces::FFIConversion<ferment_example_traits::nested::ProtocolError>
                for ferment_example_traits_nested_ProtocolError
            {
                unsafe fn ffi_from_const(
                    ffi: *const ferment_example_traits_nested_ProtocolError,
                ) -> ferment_example_traits::nested::ProtocolError {
                    let ffi_ref = &*ffi;
                    match ffi_ref {
                        ferment_example_traits_nested_ProtocolError::IdentifierError(o_0) => {
                            ferment_example_traits::nested::ProtocolError::IdentifierError(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                        ferment_example_traits_nested_ProtocolError::Unknown(o_0) => {
                            ferment_example_traits::nested::ProtocolError::Unknown(
                                ferment_interfaces::FFIConversion::ffi_from(*o_0),
                            )
                        }
                    }
                }
                unsafe fn ffi_to_const(
                    obj: ferment_example_traits::nested::ProtocolError,
                ) -> *const ferment_example_traits_nested_ProtocolError {
                    ferment_interfaces::boxed(match obj {
                        ferment_example_traits::nested::ProtocolError::IdentifierError(o_0) => {
                            ferment_example_traits_nested_ProtocolError::IdentifierError(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                        ferment_example_traits::nested::ProtocolError::Unknown(o_0) => {
                            ferment_example_traits_nested_ProtocolError::Unknown(
                                ferment_interfaces::FFIConversion::ffi_to(o_0),
                            )
                        }
                    })
                }
                unsafe fn destroy(ffi: *mut ferment_example_traits_nested_ProtocolError) {
                    ferment_interfaces::unbox_any(ffi);
                }
            }
            impl Drop for ferment_example_traits_nested_ProtocolError {
                fn drop(&mut self) {
                    unsafe {
                        match self {
                            ferment_example_traits_nested_ProtocolError::IdentifierError(o_0) => {
                                <std::os::raw::c_char as ferment_interfaces::FFIConversion<
                                    String,
                                >>::destroy(*o_0)
                            }
                            ferment_example_traits_nested_ProtocolError::Unknown(o_0) => {
                                ferment_interfaces::unbox_any(*o_0);
                            }
                        };
                    }
                }
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_traits_nested_ProtocolError_IdentifierError_ctor(
                o_0: *mut std::os::raw::c_char,
            ) -> *mut ferment_example_traits_nested_ProtocolError {
                ferment_interfaces::boxed(
                    ferment_example_traits_nested_ProtocolError::IdentifierError(o_0),
                )
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_traits_nested_ProtocolError_Unknown_ctor(
                o_0: *mut crate::fermented::generics::Vec_u8,
            ) -> *mut ferment_example_traits_nested_ProtocolError {
                ferment_interfaces::boxed(ferment_example_traits_nested_ProtocolError::Unknown(o_0))
            }
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn ferment_example_traits_nested_ProtocolError_destroy(
                ffi: *mut ferment_example_traits_nested_ProtocolError,
            ) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        pub mod from_proof {
            pub mod from_proof {
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_from_proof_from_proof_FromProof_VTable { pub maybe_from_proof : unsafe extern "C" fn (request : * mut I , response : * mut O , platform_version : u32) -> * mut crate :: fermented :: generics :: Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError , pub from_proof : unsafe extern "C" fn (request : * mut I , response : * mut O , platform_version : u32) -> * mut crate :: fermented :: generics :: Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError }
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_from_proof_from_proof_FromProof {
                    pub object: *const (),
                    pub vtable:
                        *const ferment_example_traits_from_proof_from_proof_FromProof_VTable,
                }
            }
        }
        pub mod transport {
            pub mod transport_request {
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::GetDocumentsRequest`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_GetDocumentsRequest {
                    pub version: u32,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::GetDocumentsRequest,
                    > for ferment_example_traits_transport_transport_request_GetDocumentsRequest
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_traits_transport_transport_request_GetDocumentsRequest,
                    ) -> ferment_example_traits::transport::transport_request::GetDocumentsRequest
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::GetDocumentsRequest {
                            version: ffi_ref.version,
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj : ferment_example_traits :: transport :: transport_request :: GetDocumentsRequest,
                    ) -> *const ferment_example_traits_transport_transport_request_GetDocumentsRequest
                    {
                        ferment_interfaces :: boxed (ferment_example_traits_transport_transport_request_GetDocumentsRequest { version : obj . version })
                    }
                    unsafe fn destroy(
                        ffi : * mut ferment_example_traits_transport_transport_request_GetDocumentsRequest,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_GetDocumentsRequest {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsRequest_ctor(
                    version: u32,
                ) -> *mut ferment_example_traits_transport_transport_request_GetDocumentsRequest
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_GetDocumentsRequest {
                            version,
                        },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsRequest_destroy(
                    ffi : * mut ferment_example_traits_transport_transport_request_GetDocumentsRequest,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsRequest_get_version(
                    obj : * const ferment_example_traits_transport_transport_request_GetDocumentsRequest,
                ) -> u32 {
                    (*obj).version
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsRequest_set_version(
                    obj : * mut ferment_example_traits_transport_transport_request_GetDocumentsRequest,
                    value: u32,
                ) {
                    (*obj).version = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::DocumentQuery`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_DocumentQuery {
                    pub version: u32,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::DocumentQuery,
                    > for ferment_example_traits_transport_transport_request_DocumentQuery
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_traits_transport_transport_request_DocumentQuery,
                    ) -> ferment_example_traits::transport::transport_request::DocumentQuery
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::DocumentQuery {
                            version: ffi_ref.version,
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits::transport::transport_request::DocumentQuery,
                    ) -> *const ferment_example_traits_transport_transport_request_DocumentQuery
                    {
                        ferment_interfaces::boxed(
                            ferment_example_traits_transport_transport_request_DocumentQuery {
                                version: obj.version,
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_transport_transport_request_DocumentQuery,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_DocumentQuery {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_DocumentQuery_ctor(
                    version: u32,
                ) -> *mut ferment_example_traits_transport_transport_request_DocumentQuery
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_DocumentQuery {
                            version,
                        },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_DocumentQuery_destroy(
                    ffi: *mut ferment_example_traits_transport_transport_request_DocumentQuery,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_DocumentQuery_get_version(
                    obj: *const ferment_example_traits_transport_transport_request_DocumentQuery,
                ) -> u32 {
                    (*obj).version
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_DocumentQuery_set_version(
                    obj: *mut ferment_example_traits_transport_transport_request_DocumentQuery,
                    value: u32,
                ) {
                    (*obj).version = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::Uri`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_Uri {
                    pub scheme: *mut std::os::raw::c_char,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::Uri,
                    > for ferment_example_traits_transport_transport_request_Uri
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_traits_transport_transport_request_Uri,
                    ) -> ferment_example_traits::transport::transport_request::Uri
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::Uri {
                            scheme: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.scheme),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits::transport::transport_request::Uri,
                    ) -> *const ferment_example_traits_transport_transport_request_Uri
                    {
                        ferment_interfaces::boxed(
                            ferment_example_traits_transport_transport_request_Uri {
                                scheme: ferment_interfaces::FFIConversion::ffi_to(obj.scheme),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_transport_transport_request_Uri,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_Uri {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            < std :: os :: raw :: c_char as ferment_interfaces :: FFIConversion < String >> :: destroy (ffi_ref . scheme) ;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Uri_ctor(
                    scheme: *mut std::os::raw::c_char,
                ) -> *mut ferment_example_traits_transport_transport_request_Uri {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_Uri { scheme },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Uri_destroy(
                    ffi: *mut ferment_example_traits_transport_transport_request_Uri,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Uri_get_scheme(
                    obj: *const ferment_example_traits_transport_transport_request_Uri,
                ) -> *mut std::os::raw::c_char {
                    (*obj).scheme
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Uri_set_scheme(
                    obj: *mut ferment_example_traits_transport_transport_request_Uri,
                    value: *mut std::os::raw::c_char,
                ) {
                    (*obj).scheme = value;
                }
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_traits::transport::transport_request::Status`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_traits_transport_transport_request_Status {
                    Error,
                    Success,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::Status,
                    > for ferment_example_traits_transport_transport_request_Status
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_traits_transport_transport_request_Status,
                    ) -> ferment_example_traits::transport::transport_request::Status
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_traits_transport_transport_request_Status :: Error => ferment_example_traits :: transport :: transport_request :: Status :: Error , ferment_example_traits_transport_transport_request_Status :: Success => ferment_example_traits :: transport :: transport_request :: Status :: Success }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits::transport::transport_request::Status,
                    ) -> *const ferment_example_traits_transport_transport_request_Status
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example_traits :: transport :: transport_request :: Status :: Error => ferment_example_traits_transport_transport_request_Status :: Error , ferment_example_traits :: transport :: transport_request :: Status :: Success => ferment_example_traits_transport_transport_request_Status :: Success })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_transport_transport_request_Status,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_Status {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_traits_transport_transport_request_Status :: Error => { } , ferment_example_traits_transport_transport_request_Status :: Success => { } } ;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Status_Error_ctor(
                ) -> *mut ferment_example_traits_transport_transport_request_Status
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_Status::Error,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Status_Success_ctor(
                ) -> *mut ferment_example_traits_transport_transport_request_Status
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_Status::Success,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Status_destroy(
                    ffi: *mut ferment_example_traits_transport_transport_request_Status,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::Identifier`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_Identifier(u32);
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::Identifier,
                    > for ferment_example_traits_transport_transport_request_Identifier
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_traits_transport_transport_request_Identifier,
                    ) -> ferment_example_traits::transport::transport_request::Identifier
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::Identifier(ffi_ref.0)
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits::transport::transport_request::Identifier,
                    ) -> *const ferment_example_traits_transport_transport_request_Identifier
                    {
                        ferment_interfaces::boxed(
                            ferment_example_traits_transport_transport_request_Identifier(obj.0),
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_transport_transport_request_Identifier,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_Identifier {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Identifier_ctor(
                    o_0: u32,
                ) -> *mut ferment_example_traits_transport_transport_request_Identifier
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_Identifier(o_0),
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Identifier_destroy(
                    ffi: *mut ferment_example_traits_transport_transport_request_Identifier,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Identifier_get_0(
                    obj: *const ferment_example_traits_transport_transport_request_Identifier,
                ) -> u32 {
                    (*obj).0
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_Identifier_set_0(
                    obj: *mut ferment_example_traits_transport_transport_request_Identifier,
                    value: u32,
                ) {
                    (*obj).0 = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::GetDocumentsResponse`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_GetDocumentsResponse {
                    pub version: u32,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::GetDocumentsResponse,
                    > for ferment_example_traits_transport_transport_request_GetDocumentsResponse
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_traits_transport_transport_request_GetDocumentsResponse,
                    ) -> ferment_example_traits::transport::transport_request::GetDocumentsResponse
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::GetDocumentsResponse {
                            version: ffi_ref.version,
                        }
                    }                    unsafe fn ffi_to_const (obj : ferment_example_traits :: transport :: transport_request :: GetDocumentsResponse) -> * const ferment_example_traits_transport_transport_request_GetDocumentsResponse{
                        ferment_interfaces :: boxed (ferment_example_traits_transport_transport_request_GetDocumentsResponse { version : obj . version })
                    }
                    unsafe fn destroy(
                        ffi : * mut ferment_example_traits_transport_transport_request_GetDocumentsResponse,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_GetDocumentsResponse {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsResponse_ctor(
                    version: u32,
                ) -> *mut ferment_example_traits_transport_transport_request_GetDocumentsResponse
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_GetDocumentsResponse {
                            version,
                        },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsResponse_destroy(
                    ffi : * mut ferment_example_traits_transport_transport_request_GetDocumentsResponse,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsResponse_get_version(
                    obj : * const ferment_example_traits_transport_transport_request_GetDocumentsResponse,
                ) -> u32 {
                    (*obj).version
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_GetDocumentsResponse_set_version(
                    obj : * mut ferment_example_traits_transport_transport_request_GetDocumentsResponse,
                    value: u32,
                ) {
                    (*obj).version = value;
                }
                #[doc = "FFI-representation of the [`ferment_example_traits::transport::transport_request::CoreGrpcClient`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_transport_transport_request_CoreGrpcClient { pub uri : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Uri }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits::transport::transport_request::CoreGrpcClient,
                    > for ferment_example_traits_transport_transport_request_CoreGrpcClient
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_traits_transport_transport_request_CoreGrpcClient,
                    ) -> ferment_example_traits::transport::transport_request::CoreGrpcClient
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits::transport::transport_request::CoreGrpcClient {
                            uri: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.uri),
                        }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits::transport::transport_request::CoreGrpcClient,
                    ) -> *const ferment_example_traits_transport_transport_request_CoreGrpcClient
                    {
                        ferment_interfaces::boxed(
                            ferment_example_traits_transport_transport_request_CoreGrpcClient {
                                uri: ferment_interfaces::FFIConversion::ffi_to(obj.uri),
                            },
                        )
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_transport_transport_request_CoreGrpcClient,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_transport_transport_request_CoreGrpcClient {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.uri);
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_CoreGrpcClient_ctor(
                    uri : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Uri,
                ) -> *mut ferment_example_traits_transport_transport_request_CoreGrpcClient
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_transport_transport_request_CoreGrpcClient { uri },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_CoreGrpcClient_destroy(
                    ffi: *mut ferment_example_traits_transport_transport_request_CoreGrpcClient,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_CoreGrpcClient_get_uri (obj : * const ferment_example_traits_transport_transport_request_CoreGrpcClient) -> * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Uri{
                    (*obj).uri
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_transport_transport_request_CoreGrpcClient_set_uri(
                    obj: *mut ferment_example_traits_transport_transport_request_CoreGrpcClient,
                    value : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Uri,
                ) {
                    (*obj).uri = value;
                }
            }
        }
    }
    pub mod ferment_example_traits_nested {
        use crate as ferment_example_traits_nested;
        pub mod some_package {
            use crate as ferment_example_traits_nested;
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_traits_nested_some_package_SomeTrait_VTable {}
            #[repr(C)]
            #[derive(Clone)]
            pub struct ferment_example_traits_nested_some_package_SomeTrait {
                pub object: *const (),
                pub vtable: *const ferment_example_traits_nested_some_package_SomeTrait_VTable,
            }
        }
        pub mod model {
            use crate as ferment_example_traits_nested;
            pub mod snapshot {
                use crate as ferment_example_traits_nested;
                #[doc = "FFI-representation of the [`# doc = \"FFI-representation of the [`ferment_example_traits_nested::model::snapshot::LLMQSnapshotSkipMode`]\"`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub enum ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode {
                    NoSkipping = 0,
                    SkipFirst = 1,
                    SkipExcept = 2,
                    SkipAll = 3,
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits_nested::model::snapshot::LLMQSnapshotSkipMode,
                    > for ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    unsafe fn ffi_from_const(
                        ffi : * const ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) -> ferment_example_traits_nested::model::snapshot::LLMQSnapshotSkipMode
                    {
                        let ffi_ref = &*ffi;
                        match ffi_ref { ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll }
                    }
                    unsafe fn ffi_to_const(
                        obj: ferment_example_traits_nested::model::snapshot::LLMQSnapshotSkipMode,
                    ) -> *const ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                    {
                        ferment_interfaces :: boxed (match obj { ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: NoSkipping => ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping , ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipFirst => ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst , ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipExcept => ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept , ferment_example_traits_nested :: model :: snapshot :: LLMQSnapshotSkipMode :: SkipAll => ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll })
                    }
                    unsafe fn destroy(
                        ffi: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode {
                    fn drop(&mut self) {
                        unsafe {
                            match self { ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping => { } , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst => { } , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept => { } , ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipAll => { } } ;
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode_NoSkipping_ctor(
                ) -> *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces :: boxed (ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: NoSkipping)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode_SkipFirst_ctor(
                ) -> *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces :: boxed (ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipFirst)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode_SkipExcept_ctor(
                ) -> *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces :: boxed (ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode :: SkipExcept)
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode_SkipAll_ctor(
                ) -> *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode::SkipAll,
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode_destroy(
                    ffi: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = "FFI-representation of the [`ferment_example_traits_nested::model::snapshot::LLMQSnapshot`]"]
                #[repr(C)]
                #[derive(Clone)]
                pub struct ferment_example_traits_nested_model_snapshot_LLMQSnapshot {
                    pub member_list: *mut crate::fermented::generics::Vec_u8,
                    pub skip_list: *mut crate::fermented::generics::Vec_i32,
                    pub skip_list_mode: *mut crate::fermented::types::ferment_example_traits_nested::model::snapshot::ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                    pub option_vec: *mut crate::fermented::generics::Vec_u8
                }
                impl
                    ferment_interfaces::FFIConversion<
                        ferment_example_traits_nested::model::snapshot::LLMQSnapshot,
                    > for ferment_example_traits_nested_model_snapshot_LLMQSnapshot
                {
                    unsafe fn ffi_from_const(
                        ffi: *const ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    ) -> ferment_example_traits_nested::model::snapshot::LLMQSnapshot
                    {
                        let ffi_ref = &*ffi;
                        ferment_example_traits_nested::model::snapshot::LLMQSnapshot {
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
                        obj: ferment_example_traits_nested::model::snapshot::LLMQSnapshot,
                    ) -> *const ferment_example_traits_nested_model_snapshot_LLMQSnapshot
                    {
                        ferment_interfaces::boxed(
                            ferment_example_traits_nested_model_snapshot_LLMQSnapshot {
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
                        ffi: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    ) {
                        ferment_interfaces::unbox_any(ffi);
                    }
                }
                impl Drop for ferment_example_traits_nested_model_snapshot_LLMQSnapshot {
                    fn drop(&mut self) {
                        unsafe {
                            let ffi_ref = self;
                            ferment_interfaces::unbox_any(ffi_ref.member_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list);
                            ferment_interfaces::unbox_any(ffi_ref.skip_list_mode);
                            if !ffi_ref.option_vec.is_null() {
                                ferment_interfaces::unbox_any(ffi_ref.option_vec);
                            };
                        }
                    }
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_ctor(
                    member_list: *mut crate::fermented::generics::Vec_u8,
                    skip_list: *mut crate::fermented::generics::Vec_i32,
                    skip_list_mode : * mut crate :: fermented :: types :: ferment_example_traits_nested :: model :: snapshot :: ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                    option_vec: *mut crate::fermented::generics::Vec_u8,
                ) -> *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot
                {
                    ferment_interfaces::boxed(
                        ferment_example_traits_nested_model_snapshot_LLMQSnapshot {
                            member_list,
                            skip_list,
                            skip_list_mode,
                            option_vec,
                        },
                    )
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_destroy(
                    ffi: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                ) {
                    ferment_interfaces::unbox_any(ffi);
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_get_member_list(
                    obj: *const ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).member_list
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_get_skip_list(
                    obj: *const ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_i32 {
                    (*obj).skip_list
                }
                #[doc = r" # Safety"]
                #[no_mangle]                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_get_skip_list_mode (obj : * const ferment_example_traits_nested_model_snapshot_LLMQSnapshot) -> * mut crate :: fermented :: types :: ferment_example_traits_nested :: model :: snapshot :: ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode{
                    (*obj).skip_list_mode
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_get_option_vec(
                    obj: *const ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                ) -> *mut crate::fermented::generics::Vec_u8 {
                    (*obj).option_vec
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_set_member_list(
                    obj: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).member_list = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_set_skip_list(
                    obj: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_i32,
                ) {
                    (*obj).skip_list = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_set_skip_list_mode(
                    obj: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    value : * mut crate :: fermented :: types :: ferment_example_traits_nested :: model :: snapshot :: ferment_example_traits_nested_model_snapshot_LLMQSnapshotSkipMode,
                ) {
                    (*obj).skip_list_mode = value;
                }
                #[doc = r" # Safety"]
                #[no_mangle]
                pub unsafe extern "C" fn ferment_example_traits_nested_model_snapshot_LLMQSnapshot_set_option_vec(
                    obj: *mut ferment_example_traits_nested_model_snapshot_LLMQSnapshot,
                    value: *mut crate::fermented::generics::Vec_u8,
                ) {
                    (*obj).option_vec = value;
                }
            }
        }
        #[doc = "FFI-representation of the [`ferment_example_traits_nested::SomeStruct`]"]
        #[repr(C)]
        #[derive(Clone)]
        pub struct ferment_example_traits_nested_SomeStruct {
            pub name: *mut std::os::raw::c_char,
        }
        impl ferment_interfaces::FFIConversion<ferment_example_traits_nested::SomeStruct>
            for ferment_example_traits_nested_SomeStruct
        {
            unsafe fn ffi_from_const(
                ffi: *const ferment_example_traits_nested_SomeStruct,
            ) -> ferment_example_traits_nested::SomeStruct {
                let ffi_ref = &*ffi;
                ferment_example_traits_nested::SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.name),
                }
            }
            unsafe fn ffi_to_const(
                obj: ferment_example_traits_nested::SomeStruct,
            ) -> *const ferment_example_traits_nested_SomeStruct {
                ferment_interfaces::boxed(ferment_example_traits_nested_SomeStruct {
                    name: ferment_interfaces::FFIConversion::ffi_to(obj.name),
                })
            }
            unsafe fn destroy(ffi: *mut ferment_example_traits_nested_SomeStruct) {
                ferment_interfaces::unbox_any(ffi);
            }
        }
        impl Drop for ferment_example_traits_nested_SomeStruct {
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
        pub unsafe extern "C" fn ferment_example_traits_nested_SomeStruct_ctor(
            name: *mut std::os::raw::c_char,
        ) -> *mut ferment_example_traits_nested_SomeStruct {
            ferment_interfaces::boxed(ferment_example_traits_nested_SomeStruct { name })
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_traits_nested_SomeStruct_destroy(
            ffi: *mut ferment_example_traits_nested_SomeStruct,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_traits_nested_SomeStruct_get_name(
            obj: *const ferment_example_traits_nested_SomeStruct,
        ) -> *mut std::os::raw::c_char {
            (*obj).name
        }
        #[doc = r" # Safety"]
        #[no_mangle]
        pub unsafe extern "C" fn ferment_example_traits_nested_SomeStruct_set_name(
            obj: *mut ferment_example_traits_nested_SomeStruct,
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
    use crate as ferment_example_traits_nested;
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
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_ctor(count: usize, values: *mut i32) -> *mut Vec_i32 {
        ferment_interfaces::boxed(Vec_i32 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_i32_destroy(ffi: *mut Vec_i32) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { pub ok : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_FromProof , pub error : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < ferment_example_traits :: from_proof :: from_proof :: FromProof , ferment_example_traits :: from_proof :: from_proof :: ProtocolError > > for Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError) -> Result < ferment_example_traits :: from_proof :: from_proof :: FromProof , ferment_example_traits :: from_proof :: from_proof :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < ferment_example_traits :: from_proof :: from_proof :: FromProof , ferment_example_traits :: from_proof :: from_proof :: ProtocolError >) -> * const Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError_ctor (ok : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_FromProof , error : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_ProtocolError) -> * mut Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { ok , error })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError_destroy(
        ffi : * mut Result_ok_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError,
    ) {
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
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_ctor(count: usize, values: *mut u8) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { pub ok : * mut Option < ferment_example_traits :: from_proof :: from_proof :: FromProof > , pub error : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_ProtocolError }
    impl ferment_interfaces :: FFIConversion < Result < Option < ferment_example_traits :: from_proof :: from_proof :: FromProof > , ferment_example_traits :: from_proof :: from_proof :: ProtocolError > > for Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { unsafe fn ffi_from_const (ffi : * const Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError) -> Result < Option < ferment_example_traits :: from_proof :: from_proof :: FromProof > , ferment_example_traits :: from_proof :: from_proof :: ProtocolError > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < Option < ferment_example_traits :: from_proof :: from_proof :: FromProof > , ferment_example_traits :: from_proof :: from_proof :: ProtocolError >) -> * const Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { ferment_interfaces :: boxed ({ let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; Self { ok , error } }) } unsafe fn destroy (ffi : * mut Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError_ctor (ok : * mut Option < ferment_example_traits :: from_proof :: from_proof :: FromProof > , error : * mut crate :: fermented :: types :: ferment_example_traits :: from_proof :: from_proof :: ferment_example_traits_from_proof_from_proof_ProtocolError) -> * mut Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError{
        ferment_interfaces :: boxed (Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError { ok , error })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError_destroy(
        ffi : * mut Result_ok_Option_ferment_example_traits_from_proof_from_proof_FromProof_err_ferment_example_traits_from_proof_from_proof_ProtocolError,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
}
