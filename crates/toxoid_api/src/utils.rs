use crate::*;

const FNV_PRIME: u64 = 1099511628211;
const OFFSET_BASIS: u64 = 14695981039346656037;

pub fn fnv1a_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(OFFSET_BASIS, |hash, &byte| {
        (hash ^ byte as u64).wrapping_mul(FNV_PRIME)
    })
}

pub fn get_timestamp() -> f64 {
    unsafe { combine_f32(toxoid_get_timestamp()) }
}

pub fn make_c_string(string: &str) -> *mut i8 {
    unsafe { toxoid_make_c_string(string) }
}