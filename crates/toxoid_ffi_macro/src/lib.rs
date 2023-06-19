extern crate proc_macro;

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
    fn toxoid_print_string(v: *const i8, v_len: usize);
}

fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}

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
                            _ if std::any::TypeId::of::<#ty>() == std::any::TypeId::of::<u32>() => {
                                print_string("Setting a u32 value");
                                ()
                            }
                            _ if std::any::TypeId::of::<#ty>() == std::any::TypeId::of::<f32>() => {
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
        pub fn register() -> Self {
            #register_component_tokens;
            Self::default()
        }
    };

    // Create the final token stream.
    let expanded = quote! {
        impl #struct_name {
            #(#getter_tokens)*
            #(#setter_tokens)*
            #register_impl
            #default_impl
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}

fn get_type_code(ty: &Type) -> u8 {
    match ty {
        Type::Path(tp) if tp.path.is_ident("u32") => FieldType::U32 as u8,
        Type::Path(tp) if tp.path.is_ident("f32") => FieldType::F32 as u8,
        // add more type matches as needed
        _ => {
            print_string("Unsupported field type");
            FieldType::Bool as u8
        },
    }
}

// #[macro_export]
// macro_rules! component {
//     ($name:ident, $($field:ident: $ftype:ty),* $(,)?) => {
//         {
//             let name = stringify!($name);
//             let fields = &[ $( stringify!($field), )* ];
//             let types = &[ $( match stringify!($ftype) {
//                 "u8" => Type::U8 as u8,
//                 "u16" => Type::U16 as u8,
//                 "u32" => Type::U32 as u8,
//                 "u64" => Type::U64 as u8,
//                 "i8" => Type::I8 as u8,
//                 "i16" => Type::I16 as u8,
//                 "i32" => Type::I32 as u8,
//                 "i64" => Type::I64 as u8,
//                 "f32" => Type::F32 as u8,
//                 "f64" => Type::F64 as u8,
//                 "bool" => Type::Bool as u8,
//                 "String" => Type::String as u8,
//                 "Vec<u32>" => Type::U32Array as u8,
//                 "Vec<f32>" => Type::F32Array as u8,
//                 _ => {
//                     print_string("Error: unknown type for component member");
//                     0
//                 },
//             }, )* ];
//             register_component(name, fields, types);
//             // register_component("Position", &["x", "y"], &[Type::U32 as u8, Type::U32 as u8]);
//         }
//     };
// }