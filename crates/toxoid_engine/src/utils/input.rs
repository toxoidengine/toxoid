pub fn init() {
    #[cfg(target_os = "emscripten")]
    {
        let canvas_id = std::ffi::CString::new("canvas").unwrap();
        unsafe {
            let result = toxoid_ffi::emscripten::toxoid_set_keydown_callback(
                canvas_id.as_ptr() as *const core::ffi::c_char, 
                std::ptr::null_mut(), 
                1, 
                keydown_cb
            );
            if result != 0 {
                panic!("Error setting keydown callback");
            }
            let result = toxoid_ffi::emscripten::toxoid_set_keyup_callback(
                canvas_id.as_ptr() as *const core::ffi::c_char, 
                std::ptr::null_mut(), 
                1, 
                keyup_cb
            );
            if result != 0 {
                panic!("Error setting keyup callback");
            }
        }
    }  
}