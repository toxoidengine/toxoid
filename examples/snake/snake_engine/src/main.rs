#![allow(improper_ctypes_definitions)]
extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    println!("Hello world!");
    // Initialize Toxoid ECS initializers + default components + default sy stems.
    toxoid_engine::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
}