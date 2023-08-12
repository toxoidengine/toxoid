extern "C" {
    pub fn emscripten_set_main_loop_arg(
        f: unsafe extern "C" fn(*mut std::ffi::c_void),
        arg: *mut std::ffi::c_void,
        fps: i32,
        sim_infinite_loop: i32,
    );
    pub fn emscripten_cancel_main_loop();
}

unsafe extern "C" fn packaged_main_loop(_parg: *mut std::ffi::c_void) {
    if let Err(_) = crate::main_loop() {
       emscripten_cancel_main_loop();
    }
}

pub fn start_loop() {
    unsafe {
        emscripten_set_main_loop_arg(
            packaged_main_loop,
            std::ptr::null_mut(),
            -1,
            0,
        );
    }
}