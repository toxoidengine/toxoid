extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

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

    // Check if our struct has named fields.
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let name = field.ident.unwrap();
                let ty = field.ty;

                let get_name = Ident::new(&format!("get_{}", name), name.span());
                let set_name = Ident::new(&format!("set_{}", name), name.span());

                let getter_token = quote! {
                    pub fn #get_name(&self) -> &#ty {
                        &self.#name
                    }
                };

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

                getter_tokens.push(getter_token);
                setter_tokens.push(setter_token);
            }
        }
    }

    let struct_name = &input.ident;

    // Create the final token stream.
    let expanded = quote! {
        impl #struct_name {
            #(#getter_tokens)*
            #(#setter_tokens)*
        }
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(expanded)
}