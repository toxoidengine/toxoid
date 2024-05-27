use toxoid_api::*;

#[cfg(target_os = "emscripten")]
use toxoid_ffi::emscripten::{EmBool, EmscriptenKeyboardEvent, toxoid_set_keydown_callback, toxoid_set_keyup_callback};

#[cfg(target_os = "emscripten")]
unsafe extern "C" fn keydown_cb(
    _event_type:  core::ffi::c_int, 
    key_event: *const EmscriptenKeyboardEvent, 
    _user_data: *mut core::ffi::c_void
) -> EmBool {
    let key = unsafe { (*key_event).keyCode };
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();

    if key == KeyCode::Up as u32 {
        keyboard_input.set_up(true);
    }
    if key == KeyCode::Down as u32 {
        keyboard_input.set_down(true);
    }
    if key == KeyCode::Left as u32 {
        keyboard_input.set_left(true);
    }
    if key == KeyCode::Right as u32 {
        keyboard_input.set_right(true);
    }
    return 0;
}

#[cfg(target_os = "emscripten")]
unsafe extern "C" fn keyup_cb(
    _event_type:  core::ffi::c_int, 
    key_event: *const EmscriptenKeyboardEvent, 
    _user_data: *mut core::ffi::c_void
) -> EmBool {
    let key = unsafe { (*key_event).keyCode };
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();

    if key == KeyCode::Up as u32 {
        keyboard_input.set_up(false);
    }
    if key == KeyCode::Down as u32 {
        keyboard_input.set_down(false);
    }
    if key == KeyCode::Left as u32 {
       
        keyboard_input.set_left(false);
    }
    if key == KeyCode::Right as u32 {
        keyboard_input.set_right(false);
    }
    return 0;
}

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