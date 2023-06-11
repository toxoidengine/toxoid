extern "C" {
    pub fn app_main();
}

#[no_mangle]
pub unsafe extern "C" fn sephiroth(a: i32, b: i32) -> i32 {
    println!("Printing from Toxoid Engine! {:?}", a + b);
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    app_main();
}

fn main() {
    println!("Hello world!");
    flecs_core::init();
    println!("Flecs Initialized!");
    unsafe {
        println!("World: {:?}", flecs_core::WORLD);
    }
}
