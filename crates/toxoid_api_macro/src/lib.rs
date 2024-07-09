extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser}, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, FieldsNamed, Ident, ItemFn, Type
};

#[repr(u8)]
enum FieldType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    U32Array,
    F32Array,
    Pointer
}

// Constants for FNV-1a hashing
const FNV_PRIME: u64 = 1099511628211;
const OFFSET_BASIS: u64 = 14695981039346656037;

// Function to compute FNV-1a hash of a string
fn fnv1a_hash_str(s: &str) -> u64 {
    s.bytes().fold(OFFSET_BASIS, |hash, byte| {
        (hash ^ (byte as u64)).wrapping_mul(FNV_PRIME)
    })
}

// The input to the macro will be a list of field names and types.
struct ComponentStruct {
    name: Ident,
    fields: FieldsNamed,
}

// Implement the parsing functionality.
impl Parse for ComponentStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let fields = input.parse()?;
        Ok(ComponentStruct { name, fields })
    }
}
 
#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    let components = Punctuated::<ComponentStruct, Comma>::parse_terminated
        .parse(input)
        .unwrap();
    let expanded = components
        .into_iter()
        .map(|component| {
            let ComponentStruct { name, fields } = component;
            let fields: Vec<_> = fields.named.iter().collect();

            let field_names = fields.iter().map(|f| &f.ident);
            let field_types = fields.iter().map(|f| &f.ty);

            fn align_offset(offset: u32, align: u32) -> u32 {
                (offset + align - 1) & !(align - 1)
            }
            let mut current_offset = 0;
            let fields_offsets = fields.iter().map(
                |field| {
                    let field_type = &field.ty;
                    let size = get_type_size(field_type);
                    let align = get_type_alignment(field_type);
                    current_offset = align_offset(current_offset, align);
                    let offset = current_offset;
                    current_offset += size;
                    offset
                }
            );

            let getters_and_setters =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .zip(fields_offsets)
                    .map(|((field_name, field_type), field_offset)| {
                        let getter_name = Ident::new(
                            &format!("get_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let setter_name = Ident::new(
                            &format!("set_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );

                        let field_type_str = format!("{}", quote!(#field_type));
                        match () {
                            // Compare field_type against string type such as "u8"
                            _ if field_type_str == "u8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_u8(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u8) {
                                        unsafe {
                                            toxoid_component_set_member_u8(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_u16(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u16) {
                                        unsafe {
                                            toxoid_component_set_member_u16(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_u32(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u32) {
                                        unsafe {
                                            toxoid_component_set_member_u32(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "u64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            combine_u32(toxoid_component_get_member_u64(self.ptr, #field_offset))
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: u64) {
                                        unsafe {
                                            toxoid_component_set_member_u64(self.ptr, #field_offset, split_u64(value));
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i8" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_i8(self.ptr, #field_offset);
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i8) {
                                        unsafe {
                                            toxoid_component_set_member_i8(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i16" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_i16(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i16) {
                                        unsafe {
                                            toxoid_component_set_member_i16(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_i32(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i32) {
                                        unsafe {
                                            toxoid_component_set_member_i32(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "i64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_i64(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: i64) {
                                        unsafe {
                                            toxoid_component_set_member_i64(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_f32(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: f32) {
                                        unsafe {
                                            toxoid_component_set_member_f32(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "f64" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_f64(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: f64) {
                                        unsafe {
                                            toxoid_component_set_member_f64(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "bool" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_bool(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: bool) {
                                        unsafe {
                                            toxoid_component_set_member_bool(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "*mut c_char" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_string(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: *mut c_char) {
                                        unsafe {
                                            toxoid_component_set_member_string(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "* mut u32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_u32array(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: *mut u32) {
                                        unsafe {
                                            toxoid_component_set_member_u32array(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "* mut f32" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_f32array(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: *mut f32) {
                                        unsafe {
                                            toxoid_component_set_member_f32array(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "* mut c_void" => {
                                quote! {
                                    pub fn #getter_name(&self) -> #field_type {
                                        unsafe {
                                            toxoid_component_get_member_ptr(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: *mut c_void) {
                                        unsafe {
                                            toxoid_component_set_member_ptr(self.ptr, #field_offset, value);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "U32Array" => {
                                quote! {
                                    pub fn #getter_name(&self) -> *mut u32 {
                                        unsafe {
                                            toxoid_component_get_member_u32array(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: U32Array) {
                                        unsafe {
                                            toxoid_component_set_member_u32array(self.ptr, #field_offset, value.ptr);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "F32Array" => {
                                quote! {
                                    pub fn #getter_name(&self) -> *mut f32 {
                                        unsafe {
                                            toxoid_component_get_member_f32array(self.ptr, #field_offset)
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: F32Array) {
                                        unsafe {
                                            toxoid_component_set_member_f32array(self.ptr, #field_offset, value.ptr);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "Pointer" => {
                                quote! {
                                    pub fn #getter_name(&self) -> Pointer {
                                        unsafe {
                                            Pointer { ptr: toxoid_component_get_member_ptr(self.ptr, #field_offset) }
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: Pointer) {
                                        unsafe {
                                            toxoid_component_set_member_ptr(self.ptr, #field_offset, value.ptr);
                                        }
                                    }
                                }
                            },
                            _ if field_type_str == "StringPtr" => {
                                quote! {
                                    pub fn #getter_name(&self) -> &str {
                                        unsafe {
                                            let member_ptr = toxoid_component_get_member_ptr(self.ptr, #field_offset);
                                            let c_str: &core::ffi::CStr = unsafe { core::ffi::CStr::from_ptr(member_ptr as *const i8) };
                                            let string_ptr: &str = c_str.to_str().unwrap();
                                            string_ptr
                                        }
                                    }
                                    pub fn #setter_name(&mut self, value: StringPtr) {
                                        unsafe {
                                            toxoid_component_set_member_ptr(self.ptr, #field_offset, value.ptr as *mut core::ffi::c_void);
                                        }
                                    }
                                }
                            },
                            _ => {
                                println!("Unsupported field type: {}", quote!(#field_type));
                                panic!("Unsupported field type for getter/setter");
                                // quote! {
                                //     pub fn #getter_name(&self) -> #field_type {
                                //         unsafe {
                                //             toxoid_component_get_member_u8(self.ptr, #field_offset)
                                //         }
                                //     }
                                //     pub fn #setter_name(&mut self, value: u8) {
                                //         unsafe {
                                //             toxoid_component_set_member_string(self.ptr, #field_offset, value);
                                //         }
                                //     }
                                // }
                            }
                        }
                    });

            let struct_fields =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        let field_type_str = format!("{}", quote!(#field_type));
                        match field_type_str.as_str() {
                            "Pointer" => {
                                quote! {
                                    #[serde(skip)]
                                    pub #field_name: #field_type,
                                }
                            },
                            "* mut c_void" => {
                                quote! {
                                    #[serde(skip)]
                                    pub #field_name: #field_type,
                                }
                            },
                            "U32Array" => {
                                quote! {
                                    #[serde(skip)]
                                    pub #field_name: #field_type,
                                }
                            },
                            "F32Array" => {
                                quote! {
                                    #[serde(skip)]
                                    pub #field_name: #field_type,
                                }
                            },
                            _ => {
                                quote! {
                                    pub #field_name: #field_type,
                                }
                            }
                        }
                    });

            let default_body =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        quote! {
                            #field_name: #field_type::default(),
                        }
                    });

            let default_impl = quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self {
                            ptr: ::std::ptr::null_mut(),
                            singleton: false,
                            id: 0,
                            #(#default_body)*
                        }
                    }
                }
            };

            let struct_name_str = name.to_string();
            let type_hash = fnv1a_hash_str(&struct_name_str);
            let type_hash_fn = quote! {
                fn get_hash() -> u64 {
                    #type_hash
                }
            };

            // Create the register component tokens.
            let field_names_str = field_names.clone().map(|f| f.clone().unwrap().to_string());
            let field_types_code = field_types.clone().map(|f| get_type_code(f));
            let register_component_tokens = quote! {
                let component_id_split = unsafe {
                    toxoid_register_component_ecs(
                        #struct_name_str,
                        &[#(#field_names_str),*],
                        &[#(#field_types_code),*],
                    )
                };
                let component_id = combine_u32(component_id_split);
                let type_hash = split_u64(#type_hash);
                cache_component_ecs(type_hash, split_u64(component_id));
                component_id
            };
            
            // Create the register implementation.
            let register_fn = quote! {
                fn register() -> u64 {
                    #register_component_tokens
                }
            };
            
            let type_name = struct_name_str.as_str();
            let type_name_fn = quote! {
                fn get_name() -> &'static str {
                    #type_name
                }
            };

            quote! {
                #[derive(Clone, PartialEq, Serialize, Deserialize)]
                #[repr(C)]
                pub struct #name {
                    #[serde(skip, default = "default_ptr")]
                    ptr: *mut core::ffi::c_void,
                    singleton: bool,
                    id: ecs_entity_t,
                    #(#struct_fields)*
                }

                #default_impl

                impl #name {
                    #(#getters_and_setters)*
                }

                impl ComponentType for #name {
                    // Static methods
                    #register_fn
                    #type_hash_fn
                    #type_name_fn
                }

                impl Component for #name {
                    // Object compatible trait methods
                    fn get_id(&self) -> u64 {
                        combine_u32(unsafe { toxoid_component_lookup(toxoid_make_c_string(#type_name)) }) 
                    }

                    fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
                        self.ptr = ptr;
                    }

                    fn get_ptr(&self) -> *mut core::ffi::c_void {
                        self.ptr
                    }

                    fn set_singleton(&mut self, singleton: bool) {
                        self.singleton = singleton;
                    }

                    fn get_singleton(&self) -> bool {
                        self.singleton
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    TokenStream::from(quote! {
        #(#expanded)*
    })
}

fn get_type_code(ty: &Type) -> u8 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u8") => FieldType::U8 as u8,
        Type::Path(tp) if tp.path.is_ident("u16") => FieldType::U16 as u8,
        Type::Path(tp) if tp.path.is_ident("u32") => FieldType::U32 as u8,
        Type::Path(tp) if tp.path.is_ident("u64") => FieldType::U64 as u8,
        Type::Path(tp) if tp.path.is_ident("i8") => FieldType::I8 as u8,
        Type::Path(tp) if tp.path.is_ident("i16") => FieldType::I16 as u8,
        Type::Path(tp) if tp.path.is_ident("i32") => FieldType::I32 as u8,
        Type::Path(tp) if tp.path.is_ident("i64") => FieldType::I64 as u8,
        Type::Path(tp) if tp.path.is_ident("f32") => FieldType::F32 as u8,
        Type::Path(tp) if tp.path.is_ident("f64") => FieldType::F64 as u8,
        Type::Path(tp) if tp.path.is_ident("bool") => FieldType::Bool as u8,
        Type::Path(tp) if tp.path.is_ident("String") => FieldType::String as u8,
        Type::Path(tp) if tp.path.is_ident("U32Array") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("F32Array") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("Pointer") => FieldType::Pointer as u8,
        Type::Path(tp) if tp.path.is_ident("StringPtr") => FieldType::String as u8,
        Type::Ptr(ptr) => {
            match *ptr.elem {
                Type::Path(ref tp) if tp.path.is_ident("u32") => {
                    FieldType::U32Array as u8
                },
                Type::Path(ref tp) if tp.path.is_ident("f32") => {
                    FieldType::F32Array as u8
                },
                Type::Path(ref tp) if tp.path.is_ident("c_void") => {
                    FieldType::Pointer as u8
                },
                _ => {
                    println!("Unsupported pointer type code: {}", quote!(#ptr));
                    panic!("Unsupported type code")
                }
            }
        }
        _ => {
            println!("Unsupported type: {}", quote!(#ty));
            panic!("Unsupported type code")
        }
    }
}

fn get_type_size(ty: &Type) -> u32 {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        match ty {
            Type::Path(tp) if tp.path.is_ident("u8") => 1,
            Type::Path(tp) if tp.path.is_ident("u16") => 2,
            Type::Path(tp) if tp.path.is_ident("u32") => 4,
            Type::Path(tp) if tp.path.is_ident("u64") => 8,
            Type::Path(tp) if tp.path.is_ident("i8") => 1,
            Type::Path(tp) if tp.path.is_ident("i16") => 2,
            Type::Path(tp) if tp.path.is_ident("i32") => 4,
            Type::Path(tp) if tp.path.is_ident("i64") => 8,
            Type::Path(tp) if tp.path.is_ident("f32") => 4,
            Type::Path(tp) if tp.path.is_ident("f64") => 8,
            Type::Path(tp) if tp.path.is_ident("bool") => 1,
            Type::Path(tp) if tp.path.is_ident("String") => 4,
            Type::Path(tp) if tp.path.is_ident("U32Array") => 4,
            Type::Path(tp) if tp.path.is_ident("F32Array") => 4,
            Type::Path(tp) if tp.path.is_ident("Pointer") => 4,
            Type::Path(tp) if tp.path.is_ident("StringPtr") => 4,
            Type::Ptr(ptr) => {
                match *ptr.elem {
                    Type::Path(ref tp) if tp.path.is_ident("u32") => {
                        4
                    },
                    Type::Path(ref tp) if tp.path.is_ident("f32") => {
                        4
                    },
                    Type::Path(ref tp) if tp.path.is_ident("c_void") => {
                        4
                    },
                    _ => {
                        println!("Unsupported pointer field type: {}", quote!(#ptr));
                        panic!("Unsupported field type")
                    }
                }
            }
            _ => {
                println!("Unsupported field type: {}", quote!(#ty));
                panic!("Unsupported field type")
            }
        }
    } else {
        match ty {
            Type::Path(tp) if tp.path.is_ident("u8") => 1,
            Type::Path(tp) if tp.path.is_ident("u16") => 2,
            Type::Path(tp) if tp.path.is_ident("u32") => 4,
            Type::Path(tp) if tp.path.is_ident("u64") => 8,
            Type::Path(tp) if tp.path.is_ident("i8") => 1,
            Type::Path(tp) if tp.path.is_ident("i16") => 2,
            Type::Path(tp) if tp.path.is_ident("i32") => 4,
            Type::Path(tp) if tp.path.is_ident("i64") => 8,
            Type::Path(tp) if tp.path.is_ident("f32") => 4,
            Type::Path(tp) if tp.path.is_ident("f64") => 8,
            Type::Path(tp) if tp.path.is_ident("bool") => 1,
            Type::Path(tp) if tp.path.is_ident("String") => 8,
            Type::Path(tp) if tp.path.is_ident("U32Array") => 8,
            Type::Path(tp) if tp.path.is_ident("F32Array") => 8,
            Type::Path(tp) if tp.path.is_ident("Pointer") => 8,
            Type::Path(tp) if tp.path.is_ident("StringPtr") => 8,
            Type::Ptr(ptr) => {
                match *ptr.elem {
                    Type::Path(ref tp) if tp.path.is_ident("u32") => {
                        8
                    },
                    Type::Path(ref tp) if tp.path.is_ident("f32") => {
                        8
                    },
                    Type::Path(ref tp) if tp.path.is_ident("c_void") => {
                        8
                    },
                    _ => {
                        println!("Unsupported pointer field type: {}", quote!(#ptr));
                        panic!("Unsupported field type")
                    }
                }
            }
            _ => {
                println!("Unsupported field type: {}", quote!(#ty));
                panic!("Unsupported field type")
            }
        }
    }    
}

fn get_type_alignment(ty: &Type) -> u32 {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("wasm32") {
        match ty {
            Type::Path(tp) if tp.path.is_ident("u8") => 1,
            Type::Path(tp) if tp.path.is_ident("u16") => 2,
            Type::Path(tp) if tp.path.is_ident("u32") => 4,
            Type::Path(tp) if tp.path.is_ident("u64") => 8,
            Type::Path(tp) if tp.path.is_ident("i8") => 1,
            Type::Path(tp) if tp.path.is_ident("i16") => 2,
            Type::Path(tp) if tp.path.is_ident("i32") => 4,
            Type::Path(tp) if tp.path.is_ident("i64") => 8,
            Type::Path(tp) if tp.path.is_ident("f32") => 4,
            Type::Path(tp) if tp.path.is_ident("f64") => 8,
            Type::Path(tp) if tp.path.is_ident("bool") => 1,
            Type::Path(tp) if tp.path.is_ident("String") => 4, // Assuming String is a pointer
            Type::Path(tp) if tp.path.is_ident("U32Array") => 4, // Assuming U32Array is a pointer
            Type::Path(tp) if tp.path.is_ident("F32Array") => 4, // Assuming F32Array is a pointer
            Type::Path(tp) if tp.path.is_ident("Pointer") => 4, // Assuming Pointer is a pointer
            Type::Path(tp) if tp.path.is_ident("StringPtr") => 4, // Assuming StringPtr is a pointer
            Type::Ptr(_) => 4, // Pointers are 4 bytes in a 32-bit context
            _ => {
                println!("Unsupported field type: {}", quote!(#ty));
                panic!("Unsupported field type")
            }
        }
    } else {
        match ty {
            Type::Path(tp) if tp.path.is_ident("u8") => 1,
            Type::Path(tp) if tp.path.is_ident("u16") => 2,
            Type::Path(tp) if tp.path.is_ident("u32") => 4,
            Type::Path(tp) if tp.path.is_ident("u64") => 8,
            Type::Path(tp) if tp.path.is_ident("i8") => 1,
            Type::Path(tp) if tp.path.is_ident("i16") => 2,
            Type::Path(tp) if tp.path.is_ident("i32") => 4,
            Type::Path(tp) if tp.path.is_ident("i64") => 8,
            Type::Path(tp) if tp.path.is_ident("f32") => 4,
            Type::Path(tp) if tp.path.is_ident("f64") => 8,
            Type::Path(tp) if tp.path.is_ident("bool") => 1,
            Type::Path(tp) if tp.path.is_ident("String") => 8, // Assuming String is a pointer
            Type::Path(tp) if tp.path.is_ident("U32Array") => 8, // Assuming U32Array is a pointer
            Type::Path(tp) if tp.path.is_ident("F32Array") => 8, // Assuming F32Array is a pointer
            Type::Path(tp) if tp.path.is_ident("Pointer") => 8, // Assuming Pointer is a pointer
            Type::Path(tp) if tp.path.is_ident("StringPtr") => 8, // Assuming StringPtr is a pointer
            Type::Ptr(_) => 8, // Pointers are 8 bytes in a 64-bit context
            _ => {
                println!("Unsupported field type: {}", quote!(#ty));
                panic!("Unsupported field type")
            }
        }
    }
}


#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(attr as syn::parse::Nothing); // ignore attributes for now
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract relevant information from the input function
    let fn_name = input_fn.sig.ident.clone();
    let extern_fn_name = syn::Ident::new(&format!("{}_extern_c", fn_name), fn_name.span());
    let inputs = input_fn.sig.inputs.clone();
    let block = input_fn.block.clone();

    // Generate the extern "C" wrapper function
    let expanded = quote! {
        #input_fn
    
        #[no_mangle]
        pub extern "C" fn #extern_fn_name(#inputs) {
            #block
        }
    };

    TokenStream::from(expanded)
}

use syn::{Token, Result};
use quote::format_ident;
use syn::Stmt;

struct ComponentTuple(Punctuated<Option<Type>, Token![,]>);

impl Parse for ComponentTuple {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut components = Punctuated::new();
        while !input.is_empty() {
            if input.peek(Token![_]) {
                input.parse::<Token![_]>()?;
                components.push_value(None);
            } else {
                let typ: Type = input.parse()?;
                components.push_value(Some(typ));
            }
            if !input.is_empty() {
                components.push_punct(input.parse()?);
            }
        }
        Ok(ComponentTuple(components))
    }
}

#[proc_macro_attribute]
pub fn components(args: TokenStream, input: TokenStream) -> TokenStream {
    let ComponentTuple(types) = parse_macro_input!(args as ComponentTuple);
    let mut func = parse_macro_input!(input as ItemFn);

    let component_vars: Vec<_> = types.iter().enumerate().filter_map(|(index, typ)| {
        typ.as_ref().map(|t| {
            let ident = make_variable_name(t);
            let term_index = index as i32 + 1; // 1-based indexing in FLecs
            let stmt: Stmt = syn::parse_quote! {
                let #ident = iter.field_mut::<#t>(#term_index);
            };
            (stmt, ident)
        })
    }).collect();

    let iter_name = format_ident!("components");
    let zip_expr = component_vars.iter()
        .map(|(_, ident)| quote! { #ident.iter_mut() })
        .reduce(|acc, next| quote! { #acc.zip(#next) })
        .unwrap_or_else(|| quote! { std::iter::empty() });

    let tuple_idents: Vec<_> = component_vars.iter().map(|(_, ident)| ident).collect();

    let map_expr = match tuple_idents.len() {
        1 => quote! { |#(#tuple_idents),*| (#(#tuple_idents),*) },
        2 => quote! { |((a, b))| (a, b) },
        3 => quote! { |(((a, b), c))| (a, b, c) },
        4 => quote! { |((((a, b), c), d))| (a, b, c, d) },
        5 => quote! { |(((((a, b), c), d), e))| (a, b, c, d, e) },
        6 => quote! { |((((((a, b), c), d), e), f))| (a, b, c, d, e, f) },
        7 => quote! { |(((((((a, b), c), d), e), f), g))| (a, b, c, d, e, f, g) },
        8 => quote! { |((((((((a, b), c), d), e), f), g), h))| (a, b, c, d, e, f, g, h) },
        9 => quote! { |(((((((((a, b), c), d), e), f), g), h), i))| (a, b, c, d, e, f, g, h, i) },
        _ => panic!("Unsupported number of components"),
    };

    let iter_statement = syn::parse_quote! {
        let #iter_name = #zip_expr.map(#map_expr);
    };

    // Inject component retrieval and iterator creation at the start of the function's block
    let mut stmts = std::mem::take(&mut func.block.stmts);
    for (stmt, _) in component_vars.clone().into_iter().rev() {
        stmts.insert(0, stmt);
    }
    stmts.insert(component_vars.len(), iter_statement);
    func.block.stmts = stmts;

    let output = quote! {
        #func
    };

    TokenStream::from(output)
}

fn make_variable_name(t: &Type) -> Ident {
    let type_str = match t {
        Type::Path(type_path) if type_path.qself.is_none() => {
            // Extract the last segment as the type name
            type_path.path.segments.last().unwrap().ident.to_string()
        },
        _ => panic!("Unsupported type in `each` macro")
    };

    // Convert CamelCase to snake_case
    let mut snake_case = String::new();
    for (i, ch) in type_str.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            snake_case.push('_');
        }
        snake_case.push(ch.to_lowercase().next().unwrap());
    }

    format_ident!("{}", snake_case)
}