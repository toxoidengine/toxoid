use crate::bindings::*;

pub fn print_i32(v: i32) {
    unsafe {
        toxoid_print_i32(v);
    }
}

// pub fn print_u64(v: u64) {
//     unsafe {
//         toxoid_print_u64(v);
//     }
// }

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}
