// use std::time::{SystemTime, UNIX_EPOCH};

// // This is a workaround for WASM32 not supporting 64 bit integers
// // Emscripten has JS helper functions for this but those are
// // not available in no_std dynamically link side modules like
// // is used in this engine
#[derive(Debug)]
#[repr(C)]
pub struct SplitU64 {
    pub high: u32,
    pub low: u32,
}

pub fn split_u64(value: u64) -> SplitU64 {
    SplitU64 {
        high: (value >> 32) as u32,
        low: (value & 0xFFFFFFFF) as u32,
    }
}

pub fn combine_u32(split_u64: SplitU64) -> u64 {
    ((split_u64.high as u64) << 32) | (split_u64.low as u64)
}

#[derive(Debug)]
#[repr(C)]
pub struct SplitF64 {
    pub high: f32,
    pub low: f32,
}

pub fn split_f64(value: f64) -> SplitF64 {
    let bits = value.to_bits();           // Get the binary representation of f64 as u64
    let high = ((bits >> 32) & 0xFFFFFFFF) as u32; // Extract the high 32 bits
    let low = (bits & 0xFFFFFFFF) as u32;         // Extract the low 32 bits
    SplitF64 {
        high: f32::from_bits(high), // Convert the high bits back to f32
        low: f32::from_bits(low),   // Convert the low bits back to f32
    }
}

pub fn combine_f32(split_f64: SplitF64) -> f64 {
    let bits1 = split_f64.high.to_bits() as u64;  // Convert the first f32 to u32 and then to u64
    let bits2 = split_f64.high.to_bits() as u64;  // Convert the second f32 to u32 and then to u64
    let combined = (bits1 << 32) | bits2; // Shift the first 32 bits left and combine with the second 32 bits
    f64::from_bits(combined)              // Convert the u64 to f64
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn toxoid_make_c_string(string: &str) -> *mut i8 {
    let c_string = std::ffi::CString::new(string).unwrap();
    let c_string_ptr = c_string.into_raw();
    c_string_ptr
}

extern "C" {
    #[cfg(target_arch = "wasm32")]
    fn emscripten_performance_now() -> f64;
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn toxoid_get_timestamp() -> SplitF64 {
    split_f64(unsafe { emscripten_performance_now() })
    // split_f64(
    //     SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .unwrap()
    //         .as_millis() as f64
    // )
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn toxoid_get_timestamp() -> u64 {
    unimplemented!()
}