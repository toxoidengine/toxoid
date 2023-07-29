extern crate proc_macro;
use core::ffi::c_void;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    FieldsNamed, Ident, Type,
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
    Array,
    U32Array,
    F32Array,
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

extern "C" {
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_component_set_member_u32(component_ptr: *mut c_void, offset: u32, value: u32);
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

            let getters_and_setters =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        let getter_name = Ident::new(
                            &format!("get_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        let setter_name = Ident::new(
                            &format!("set_{}", field_name.as_ref().unwrap()),
                            field_name.span(),
                        );
                        quote! {
                            pub fn #getter_name(&self) -> &#field_type {
                                &self.#field_name
                            }

                            pub fn #setter_name(&mut self, value: #field_type) {
                                self.#field_name = value;
                                // Your toxoid_component_set_member_u32 logic can be here
                                // Depending on your requirements
                            }
                        }
                    });

            let struct_fields =
                field_names
                    .clone()
                    .zip(field_types.clone())
                    .map(|(field_name, field_type)| {
                        quote! {
                            #field_name: #field_type,
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
                            #(#default_body)*
                        }
                    }
                }
            };

            // Create the register component tokens.
            let struct_name_str = name.to_string();
            let field_names_str = field_names.clone().map(|f| f.clone().unwrap().to_string());
            let field_types_code = field_types.clone().map(|f| get_type_code(f));
            let register_component_tokens = quote! {
                let component_id = register_component_ecs(
                    #struct_name_str,
                    &[#(#field_names_str),*],
                    &[#(#field_types_code),*],
                );
                cache_component_ecs(core::any::TypeId::of::<#name>(), component_id);
                component_id
            };
            // Create the register implementation.
            let register_fn = quote! {
                fn register() -> i32 {
                    #register_component_tokens
                    //0
                }
            };

            quote! {
                pub struct #name {
                    ptr: *mut core::ffi::c_void,
                    #(#struct_fields)*
                }

                #default_impl

                impl #name {
                    #(#getters_and_setters)*
                    // pub fn set_ptr(&mut self, ptr: *mut ::std::os::raw::c_void) {
                    //     self.ptr = ptr;
                    // }
                    // pub fn get_ptr(&self) -> *mut ::std::os::raw::c_void {
                    //     self.ptr
                    // }
                }

                impl IsComponent for #name {
                    // Add implementation details here.
                    #register_fn
                    fn set_ptr(&mut self, ptr: *mut core::ffi::c_void) {
                        self.ptr = ptr;
                    }
                    fn get_ptr(&self) -> *mut core::ffi::c_void {
                        self.ptr
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
        // Type::Array(tp) if tp.elem.is_ident("u8") => FieldType::Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u8>") => FieldType::Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<u32>") => FieldType::U32Array as u8,
        Type::Path(tp) if tp.path.is_ident("Vec<f32>") => FieldType::F32Array as u8,
        _ => {
            // print_string("Unsupported field type");
            FieldType::Bool as u8
        }
    }
}
