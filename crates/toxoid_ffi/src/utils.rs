use rand::Rng;

pub fn gen_rng_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn gen_rng_grid_pos() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let window_width = 800;
    let window_height = 600;
    let entity_width = 50;
    let entity_height = 50;
    let range_max_x = window_width / entity_width;
    let range_max_y = window_height / entity_height;
    let random_x = rng.gen_range(0..range_max_x) * entity_width; // divided by entity height 
    let random_y = rng.gen_range(0..range_max_y) * entity_height; 
    (random_x, random_y)
}

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

