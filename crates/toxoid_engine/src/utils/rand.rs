use rand::Rng;

#[no_mangle]
pub extern "C" fn gen_rng_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[no_mangle]
pub extern "C" fn gen_rng_grid_pos() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let window_width = 1250;
    let window_height = 700;
    let entity_width = 50;
    let entity_height = 50;
    let range_max_x = window_width / entity_width;
    let range_max_y = window_height / entity_height;
    let random_x = rng.gen_range(0..range_max_x) * entity_width; // divided by entity height 
    let random_y = rng.gen_range(0..range_max_y) * entity_height; 
    (random_x, random_y)
}

