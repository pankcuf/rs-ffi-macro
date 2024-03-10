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
        use crate as ferment_example_traits;
        pub mod nested {
            use crate as ferment_example_traits;
            #[doc = "FFI-representation of the [`address_with_script_pubkey`]"]
            #[doc = r" # Safety"]
            #[doc = r" # Safety"]
            #[no_mangle]
            pub unsafe extern "C" fn address_with_script_pubkey(
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
        pub mod transport {
            use crate as ferment_example_traits;
            pub mod transport_request {
                use crate as ferment_example_traits;
                #[repr(C)]
                #[derive(Clone)]
                pub struct SomeOtherTrait_VTable {
                    pub some_other_method: unsafe extern "C" fn(
                        obj: *mut ferment_example_traits_transport_transport_request_SomeOtherTrait,
                    ),
                }
                #[repr(C)]
                #[derive(Clone)]
                pub struct SomeOtherTrait {
                    pub object: *const (),
                    pub vtable: *const SomeOtherTrait_VTable,
                }
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportRequest_VTable { pub execute_transport : unsafe extern "C" fn (obj : * mut ferment_example_traits_transport_transport_request_TransportRequest , client : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: TransportRequest :: ferment_example_traits_transport_transport_request_TransportRequest_Client) -> * mut crate :: fermented :: generics :: Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error }
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportRequest {
                    pub object: *const (),
                    pub vtable: *const TransportRequest_VTable,
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
                #[repr(C)]
                #[derive(Clone)]
                pub struct CanRetry_VTable {
                    pub can_retry: unsafe extern "C" fn(
                        obj: *mut ferment_example_traits_transport_transport_request_CanRetry,
                    ) -> bool,
                }
                #[repr(C)]
                #[derive(Clone)]
                pub struct CanRetry {
                    pub object: *const (),
                    pub vtable: *const CanRetry_VTable,
                }
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportResponse_VTable {}
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportResponse {
                    pub object: *const (),
                    pub vtable: *const TransportResponse_VTable,
                }
                #[repr(C)]
                #[derive(Clone)]
                pub struct Query_VTable { pub query : unsafe extern "C" fn (obj : * mut ferment_example_traits_transport_transport_request_Query , prove : bool) -> * mut crate :: fermented :: generics :: Result_ok_T_err_ferment_example_traits_transport_transport_request_Status }
                #[repr(C)]
                #[derive(Clone)]
                pub struct Query {
                    pub object: *const (),
                    pub vtable: *const Query_VTable,
                }
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
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportClient_VTable { pub with_uri : unsafe extern "C" fn (uri : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Uri) -> * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_TransportClient }
                #[repr(C)]
                #[derive(Clone)]
                pub struct TransportClient {
                    pub object: *const (),
                    pub vtable: *const TransportClient_VTable,
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
    pub struct Result_ok_T_err_ferment_example_traits_transport_transport_request_Status { pub ok : * mut crate :: fermented :: types :: T , pub error : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Status }
    impl
        ferment_interfaces::FFIConversion<
            Result<T, ferment_example_traits::transport::transport_request::Status>,
        > for Result_ok_T_err_ferment_example_traits_transport_transport_request_Status
    {
        unsafe fn ffi_from_const(
            ffi: *const Result_ok_T_err_ferment_example_traits_transport_transport_request_Status,
        ) -> Result<T, ferment_example_traits::transport::transport_request::Status> {
            let ffi_ref = &*ffi;
            ferment_interfaces::fold_to_result(
                ffi_ref.ok,
                ffi_ref.error,
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
                |o| ferment_interfaces::FFIConversion::ffi_from(o),
            )
        }
        unsafe fn ffi_to_const(
            obj: Result<T, ferment_example_traits::transport::transport_request::Status>,
        ) -> *const Result_ok_T_err_ferment_example_traits_transport_transport_request_Status
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
            ffi: *mut Result_ok_T_err_ferment_example_traits_transport_transport_request_Status,
        ) {
            ferment_interfaces::unbox_any(ffi);
        }
    }
    impl Drop for Result_ok_T_err_ferment_example_traits_transport_transport_request_Status {
        fn drop(&mut self) {
            unsafe {
                if !self.ok.is_null() {
                    ferment_interfaces::unbox_any(self.ok);
                }
                if !self.error.is_null() {
                    ferment_interfaces::unbox_any(self.error);
                };
            }
        }
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_T_err_ferment_example_traits_transport_transport_request_Status_ctor(
        ok: *mut crate::fermented::types::T,
        error : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: ferment_example_traits_transport_transport_request_Status,
    ) -> *mut Result_ok_T_err_ferment_example_traits_transport_transport_request_Status {
        ferment_interfaces::boxed(
            Result_ok_T_err_ferment_example_traits_transport_transport_request_Status { ok, error },
        )
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_T_err_ferment_example_traits_transport_transport_request_Status_destroy(
        ffi: *mut Result_ok_T_err_ferment_example_traits_transport_transport_request_Status,
    ) {
        ferment_interfaces::unbox_any(ffi);
    }
    #[repr(C)]
    #[derive(Clone)]
    pub struct Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error { pub ok : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: TransportRequest :: ferment_example_traits_transport_transport_request_TransportRequest_Response , pub error : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: TransportClient :: ferment_example_traits_transport_transport_request_TransportClient_Error }
    impl ferment_interfaces :: FFIConversion < Result < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Response , < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Client as ferment_example_traits :: transport :: transport_request :: TransportClient > :: Error > > for Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error { unsafe fn ffi_from_const (ffi : * const Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error) -> Result < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Response , < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Client as ferment_example_traits :: transport :: transport_request :: TransportClient > :: Error > { let ffi_ref = & * ffi ; ferment_interfaces :: fold_to_result (ffi_ref . ok , ffi_ref . error , | o | ferment_interfaces :: FFIConversion :: ffi_from (o) , | o | ferment_interfaces :: FFIConversion :: ffi_from (o)) } unsafe fn ffi_to_const (obj : Result < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Response , < ferment_example_traits :: transport :: transport_request :: TransportRequest :: Client as ferment_example_traits :: transport :: transport_request :: TransportClient > :: Error >) -> * const Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error { let (ok , error) = match obj { Ok (o) => (ferment_interfaces :: FFIConversion :: ffi_to (o) , std :: ptr :: null_mut ()) , Err (o) => (std :: ptr :: null_mut () , ferment_interfaces :: FFIConversion :: ffi_to (o)) } ; ferment_interfaces :: boxed (Self { ok , error }) } unsafe fn destroy (ffi : * mut Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error) { ferment_interfaces :: unbox_any (ffi) ; ; } }
    impl Drop for Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error { fn drop (& mut self) { unsafe { if ! self . ok . is_null () { ferment_interfaces :: unbox_any (self . ok) ; } if ! self . error . is_null () { ferment_interfaces :: unbox_any (self . error) ; } ; } } }
    #[doc = r" # Safety"]
    #[no_mangle]    pub unsafe extern "C" fn Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error_ctor (ok : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: TransportRequest :: ferment_example_traits_transport_transport_request_TransportRequest_Response , error : * mut crate :: fermented :: types :: ferment_example_traits :: transport :: transport_request :: TransportClient :: ferment_example_traits_transport_transport_request_TransportClient_Error) -> * mut Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error{
        ferment_interfaces :: boxed (Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error { ok , error })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error_destroy(
        ffi : * mut Result_ok_ferment_example_traits_transport_transport_request_TransportRequest_Response_err_ferment_example_traits_transport_transport_request_TransportClient_Error,
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
            let ffi_ref = &*ffi;
            ferment_interfaces::FFIVecConversion::decode(ffi_ref)
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
    pub unsafe extern "C" fn Vec_u8_ctor(values: *mut u8, count: usize) -> *mut Vec_u8 {
        ferment_interfaces::boxed(Vec_u8 { count, values })
    }
    #[doc = r" # Safety"]
    #[no_mangle]
    pub unsafe extern "C" fn Vec_u8_destroy(ffi: *mut Vec_u8) {
        ferment_interfaces::unbox_any(ffi);
    }
}
