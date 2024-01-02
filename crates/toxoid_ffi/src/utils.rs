
// This is a workaround for WASM32 not supporting 64 bit integers
// Emscripten has JS helper functions for this but those are
// not available in no_std dynamically link side modules like
// is used in this engine
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