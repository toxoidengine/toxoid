#![feature(thread_id_value)]
#![allow(improper_ctypes_definitions)]
extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    println!("This runs in the main thread: {}", std::thread::ThreadId::as_u64(&std::thread::current().id()));
    std::thread::spawn(|| {
        println!("This runs in a new thread: {}", std::thread::ThreadId::as_u64(&std::thread::current().id()));
    });

    // Initialize Toxoid ECS initializers + default components + default sy stems.
    toxoid_engine::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
}