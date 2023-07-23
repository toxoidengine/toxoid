extern crate proc_macro;

use core::ffi::c_void;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

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
    Array,
    U32Array,
    F32Array,
}

extern "C" {
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_component_set_member_u32(component_ptr: *mut c_void, offset: u32, value: u32);
}

// fn print_string(v: &str) {
//     unsafe {
//         toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
//     }
// }

#[proc_macro_derive(Components)]
pub fn components_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store tokens for each struct field.
    let mut getter_tokens = Vec::new();
    let mut setter_tokens = Vec::new();

    // Used to store the name and type of each struct field.
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    // Used to store the name and default value of each struct field.
    let mut field_defaults = Vec::new();

    // Check if our struct has named fields.
    if let Data::Struct(data) = input.data {
        // Check if our struct has named fields.
        if let Fields::Named(fields) = data.fields {
            // Iterate over each field in the struct.
            for field in fields.named {
                // Get the name and type of each field.
                let name = field.ident.unwrap();
                let ty = field.ty;

                // Getter and setter names.
                let get_name = Ident::new(&format!("get_{}", name), name.span());
                let set_name = Ident::new(&format!("set_{}", name), name.span());

                // Getter token.
                let getter_token = quote! {
                    pub fn #get_name(&self) -> &#ty {
                        &self.#name
                    }
                };

                // Setter token with type check.
                let setter_token = quote! {
                    pub fn #set_name(&mut self, value: #ty) {
                        self.#name = value;
                        match () {
                            _ if core::any::TypeId::of::<#ty>() == core::any::TypeId::of::<u32>() => {
                                print_string("Setting a u32 value");
                                unsafe {
                                    toxoid_component_set_member_u32(core::ptr::null_mut(), 0, value as u32);
                                }
                                ()
                            }
                            _ if core::any::TypeId::of::<#ty>() == core::any::TypeId::of::<f32>() => {
                                print_string("Setting a f32 value");
                                ()
                            }
                            _ => ()
                        }
                    }
                };

                // Store the tokens for each field.
                getter_tokens.push(getter_token);
                setter_tokens.push(setter_token);

                // Store the name and type of each field.
                field_names.push(name.to_string());
                field_types.push(get_type_code(&ty));

                // Assume that all types implement Default and use it for field initialization.
                let default_value = quote! { #ty::default() };
                field_defaults.push((name, default_value));
            }
        }
    }

    // Get the name of the struct.
    let struct_name = &input.ident;

    // Create the default implementation.
    let default_body = field_defaults.iter().map(|(name, default)| quote! { #name: #default });
    let default_impl = quote! {
        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#default_body),*
                }
            }
        }
    };

    // Create the register component tokens.
    let struct_name_str = struct_name.to_string();
    let register_component_tokens = quote! {
        register_component(
            #struct_name_str,
            &[#(#field_names),*],
            &[#(#field_types),*],
        )
    };

    // Create the register implementation.
    let register_impl = quote! {
        pub fn register() -> i32 {
            #register_component_tokens
        }
    };

    // Create the final token stream.
    let expanded = quote! {
        #default_impl
        impl #struct_name {
            #(#getter_tokens)*
            #(#setter_tokens)*
            #register_impl
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
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
        // Type::Array(tp) if tp.elem.is_ident("u8") => FieldType::Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u8>") => FieldType::Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u32>") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<f32>") => FieldType::F32Array as u8,
        _ => {
            // print_string("Unsupported field type");
            FieldType::Bool as u8
        },
    }
}