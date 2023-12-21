
// This is a workaround for WASM32 not supporting 64 bit integers
// Emscripten has JS helper functions for this but those are
// not available in no_std dynamically link side modules like
// is used in this engine
#[repr(C)]
pub struct SplitU64 {
    high: u32,
    low: u32,
}

pub fn split_u64(value: u64) -> SplitU64 {
    SplitU64 {
        high: (value >> 32) as u32,
        low: (value & 0xFFFFFFFF) as u32,
    }
}

