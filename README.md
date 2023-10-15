# ferment
Proc macro for FFI (work in progress)

Allows to generate an FFI-compliant equivalent for rust types (structures, enums, arrays, functions).

The project is a rust-workspace consisting several crates:
1. `ferment-interfaces`: A traits that provide conversion methods from/to FFI-compatible types and some helper functions and structures
2. `ferment-macro`: a procedural macro that just catch target code as syn-based item.
3. `ferment-example`: provides basic example.
4. `ferment`: a tool for morphing FFI-compatible syntax trees that uses the power of the `syn` crate.

A procedural macro consists of 1 macros:

1. `export` - for structures / enums / functions / types

**Usage**
Crate is not published yet, so use it for example locally

```toml
ferment-interfaces = { path = "../../ferment/ferment-interfaces" }
ferment-macro = { path = "../../ferment/ferment-macro" }
```

Using the macro implies using `cbindgen` with a configuration like (has taken from actual apple-bindings):

```rust
extern crate cbindgen;

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config::from_file("./cbindgen.toml").expect("Error config");
    // Here we must list the names of the crates from which the generated structures will be exported in order to include them in the final C-header
    let includes = vec![/**/];
    config.language = cbindgen::Language::C;
    config.parse = cbindgen::Config {
        language: cbindgen::Language::C,
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Some(includes.clone()),
            extra_bindings: includes.clone(),
            expand: cbindgen::ParseExpandConfig {
                crates: includes.clone(),
                all_features: false,
                default_features: false,
                features: None,
                profile: cbindgen::Profile::Debug,
            },
            ..Default::default()
        },
        enumeration: cbindgen::EnumConfig {
            prefix_with_name: true,
            ..Default::default()
        },
        braces: cbindgen::Braces::SameLine,
        line_length: 80,
        tab_width: 4,
        documentation_style: cbindgen::DocumentationStyle::C,
        include_guard: Some("dash_shared_core_h".to_string()),
        ..Default::default()
    };
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("target/bindings.h");
}
```

**Examples**

For the structure labeled with `ferment:export`

```rust
#[derive(Clone)]
#[ferment:export]
pub struct LLMQSnapshot {
    pub member_list: Vec<u8>,
    pub skip_list: Vec<i32>,
    pub skip_list_mode: crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode,
}
```
the following code with FFI-compatible fields and corresponding from/to conversions will be generated:
```rust
#[repr(C)] 
#[derive(Clone, Debug)] 
pub struct LLMQSnapshotFFI {
    pub member_list: *mut crate::fermented::generics::Vec_u8_FFI, 
    pub skip_list: *mut crate::fermented::generics::Vec_i32_FFI, 
    pub skip_list_mode: *mut crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipModeFFI,
} 
impl ferment_interfaces::FFIConversion<LLMQSnapshot> for LLMQSnapshotFFI {
    unsafe fn ffi_from(ffi: *mut LLMQSnapshotFFI) -> LLMQSnapshot {
        let ffi_ref = &*ffi; 
        LLMQSnapshot { 
            member_list: { 
                let vec = &*ffi_ref.member_list; 
                std::slice::from_raw_parts(vec.values as *const u8, vec.count).to_vec()
            }, 
            skip_list: { 
                let vec = &*ffi_ref.skip_list; 
                std::slice::from_raw_parts(vec.values as *const i32, vec.count).to_vec()
            }, 
            skip_list_mode: ferment_interfaces::FFIConversion::ffi_from(ffi_ref.skip_list_mode),
        }
    } 
    unsafe fn ffi_to(obj: LLMQSnapshot) -> *mut LLMQSnapshotFFI {
        ferment_interfaces::boxed(LLMQSnapshotFFI { 
            member_list: ferment_interfaces::boxed({ 
                let vec = obj.member_list;
                crate::fermented::generics::Vec_u8_FFI { 
                    count: vec.len(), 
                    values: ferment_interfaces::boxed_vec(vec.clone())
                } 
            }), 
            skip_list: ferment_interfaces::boxed({ 
                let vec = obj.skip_list;
                crate::fermented::generics::Vec_i32_FFI {
                    count: vec.len(), 
                    values: ferment_interfaces::boxed_vec(vec.clone())
                }
            }), 
            skip_list_mode: ferment_interfaces::FFIConversion::ffi_to(obj.skip_list_mode),
        })
    } 
}
impl Drop for LLMQSnapshotFFI {
    fn drop(&mut self) {
        unsafe {
            let ffi_ref = self;
            ferment_interfaces::unbox_any(ffi_ref.member_list);
            ferment_interfaces::unbox_any(ffi_ref.skip_list);
            <crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipModeFFI as ferment_interfaces::FFIConversion<crate::common::llmq_snapshot_skip_mode::LLMQSnapshotSkipMode>>::destroy(ffi_ref.skip_list_mode) ;
        }
    }
}
```

For the function labeled with `export`

```rust
#[ferment::export]
pub fn address_with_script_pubkey(script: Vec<u8>, chain_type: crate::chain::common::chain_type::ChainType) -> Option<String> {
    address::with_script_pub_key(&script, &chain_type.script_map())
}
```
the following code will be generated:
```rust
#[no_mangle] 
pub unsafe extern "C" fn ffi_address_with_script_pubkey(
    script: *mut ferment_interfaces::Vec_u8_FFI, 
    chain_type: *mut crate::chain::common::chain_type::ChainTypeFFI) 
    -> *mut std::os::raw::c_char {
    let obj = address_with_script_pubkey(
        {
            let vec = &*script;
            std::slice::from_raw_parts(vec.values as *const u8, vec.count).to_vec()
        }, 
        ferment_interfaces::FFIConversion::ffi_from(chain_type)
    );
    ferment_interfaces::FFIConversion::ffi_to_opt(obj)
}
```

For type aliases labeled with `export`

```rust
#[ferment::export]
pub type HashID = [u8; 32];
```
the following code will be generated:
```rust
#[repr(C)]
#[derive(Clone, Debug)] 
pub struct HashID_FFI(*mut [u8; 32]); 

impl ferment_interfaces::FFIConversion<HashID> for HashID_FFI {
    unsafe fn ffi_from(ffi : * mut HashID_FFI) -> HashID { 
        let ffi_ref = &*ffi; 
        *ffi_ref.0
    } 
    unsafe fn ffi_to(obj : HashID) -> *mut HashID_FFI { 
        ferment_interfaces::boxed(HashID_FFI(ferment_interfaces::boxed(obj))) 
    }
} 
impl Drop for HashID_FFI {
    fn drop(&mut self) { 
        unsafe { 
            ferment_interfaces::unbox_any(self.0);
        }
    }
}
```

Current limitations:
- doesn't work with traits and &self
- We should mark all structures that involved into export with the macro definition
- There is some difficulty with handling type aliases. Therefore, if possible, they should be avoided. Because, in order to guarantee that it can be processed, one has to wrap it in an unnamed struct. Which is, for most cases, less efficient than using the type it uses directly. That is, `pub type KeyID = u32` becomes `pub struct KeyIDFFI(u32)` The alternative is to store a hardcoded dictionary with them.
Another alternative is to write a separate build script that collects these types before running the macro to generate this dictionary on the fly. But for now, this is too much of a complication. 

Generic mangling rules

Conversion follows some mangling rules and gives the name for ffi structure. 
Examples for translated names:
- `Vec<u8>` -> `Vec_u8_FFI`
- `Vec<u32>` -> `Vec_u32_FFI`
- `Vec<Vec<u32>>` -> `Vec_Vec_u32_FFI`
- `BTreeMap<crate::HashID, Vec<u32>>` -> `Map_keys_crate_HashID_values_Vec_u32_FFI`
- `BTreeMap<crate::HashID, Vec<u32>>` -> `Map_keys_u32_values_Vec_u32_FFI`
- `BTreeMap<crate::HashID, BTreeMap<crate::HashID, Vec<u32>>>` -> `Map_keys_crate_HashID_values_Map_keys_crate_HashID_values_Vec_u32_FFI`
- etc
Then macro implements the necessary conversions for these structures. Example for such an expansion:
```rust
#[repr(C)] #[derive(Clone)] 
pub struct Map_keys_self_HashID_values_self_HashID_FFI {
    pub count: usize, 
    pub keys: *mut *mut self::HashIDFFI, 
    pub values: * mut * mut self::HashIDFFI,
} 
impl ferment_interfaces::FFIConversion<BTreeMap<self::HashID, self::HashID>> for Map_keys_self_HashID_values_self_HashID_FFI {
    unsafe fn ffi_from_const(ffi: *const Map_keys_self_HashID_values_self_HashID_FFI) -> BTreeMap<self::HashID, self::HashID> {
        let ffi_ref = &*ffi;
        (0..ffi_ref.count).fold(BTreeMap<self::HashID, self::HashID>::new(), |mut acc, i| {
            let key = *ffi_ref.keys.add(i); 
            let value = *ffi_ref.values.add(i); 
            acc.insert(key, value); 
            acc
        })
    } 
    unsafe fn ffi_to_const(obj: BTreeMap<self::HashID, self::HashID>) -> *const Map_keys_self_HashID_values_self_HashID_FFI {
        ferment_interfaces::boxed(Self { 
            count: obj.len(), 
            keys: ferment_interfaces::boxed_vec(obj.keys().map(|o| <self::HashID as ferment_interfaces::FFIConversion>::ffi_from_const(o)).collect()), 
            values: ferment_interfaces::boxed_vec(obj.values().map(|o| <self::HashID as ferment_interfaces::FFIConversion>::ffi_from_const(o)).collect())
        })
    } 
    unsafe fn destroy(ffi: *mut Map_keys_self_HashID_values_self_HashID_FFI) {
        ferment_interfaces::unbox_any(ffi); 
    }
} 
impl Drop for Map_keys_self_HashID_values_self_HashID_FFI {
    fn drop(&mut self) {
        unsafe {
            ferment_interfaces::unbox_vec_ptr(self.keys, self.count);
            ferment_interfaces::unbox_vec_ptr(self.values, self.count);
        }
    }
}
```