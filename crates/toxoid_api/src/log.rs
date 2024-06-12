use crate::bindings::*;

pub fn print_i32(v: i32) {
    unsafe {
        toxoid_print_i32(v);
    }
}

pub fn print_u64(v: u64) {
    unsafe {
        #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
        toxoid_print_u64(split_u64(v));
        #[cfg(not(all(target_arch="wasm32", target_os="emscripten")))]
        toxoid_print_u64(v);
    }
}

pub fn print_f32(v: f32) {
    unsafe {
        toxoid_print_f32(v);
    }
}

pub fn print_f64(v: f64) {
    unsafe {
        #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
        toxoid_print_f64(split_f64(v));
        #[cfg(not(all(target_arch="wasm32", target_os="emscripten")))]
        toxoid_print_f64(v);
    }
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}
