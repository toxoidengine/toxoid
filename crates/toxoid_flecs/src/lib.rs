#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

#[cfg(target_arch = "wasm32")]
pub mod bindings {
    include!("./bindings_x86.rs");
}
#[cfg(not(target_arch = "wasm32"))]
pub mod bindings {
    include!("./bindings.rs");
}

pub use bindings::*;
