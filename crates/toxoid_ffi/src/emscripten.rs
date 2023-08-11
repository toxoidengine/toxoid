extern "C" {
    pub fn emscripten_set_main_loop_arg(
        f: unsafe extern "C" fn(*mut std::ffi::c_void),
        arg: *mut std::ffi::c_void,
        fps: i32,
        sim_infinite_loop: i32,
    );
    pub fn emscripten_cancel_main_loop();
}

unsafe extern "C" fn packaged_main_loop(parg: *mut std::ffi::c_void) {
    // let arg = &mut *(parg as *mut GameLoopArg);
    // if let Err(_) = main_loop(&*arg.sdl_context, &mut *arg.canvas, &mut *arg.state) {
    //    //  emscripten_cancel_main_loop();
    // }
}