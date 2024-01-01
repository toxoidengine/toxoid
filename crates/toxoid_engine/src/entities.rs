use std::os::raw::c_void;

use toxoid_api::*;
use toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes;

pub extern "C" fn callback(
    event_type: *mut ::std::os::raw::c_void,
    user_data: *mut ::std::os::raw::c_void
) {
    println!("Hello?");
}

pub extern "C" fn callback(
    event_type: *mut ::std::os::raw::c_void,
    user_data: *mut ::std::os::raw::c_void
) {
    println!("Hello?");
}


pub fn init() {
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    World::add_singleton::<KeyboardInput>();
    
    let mut attributes = EmscriptenWebSocketCreateAttributes {
        url: "ws://127.0.0.1:9000\0".as_ptr() as *const i8, 
        protocol: std::ptr::null()
    };
    let ws = unsafe { toxoid_ffi::emscripten::emscripten_websocket_new(&mut attributes as *mut EmscriptenWebSocketCreateAttributes) };
    println!("ws: {:?}", ws);
    let user_data = 0 as *mut ::std::os::raw::c_void;
    unsafe {
        toxoid_ffi::emscripten::emscripten_websocket_set_onopen_callback_on_thread(ws, user_data, callback,  toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_MAIN_RUNTIME_THREAD as *mut c_void);
    }

    crate::utils::load_image("assets/character.png");
}