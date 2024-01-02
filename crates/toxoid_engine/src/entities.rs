use std::os::raw::c_void;

use toxoid_api::*;
use toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes;

pub extern "C" fn callback(
    event_type: *mut ::std::os::raw::c_void,
    user_data: *mut ::std::os::raw::c_void
) {
    println!("Hello?");
}

pub extern "C" fn onmessage_cb(
    event_type: ::std::os::raw::c_int,
    websocket_event: *const toxoid_ffi::emscripten::EmscriptenWebSocketMessageEvent,
    user_data: *mut ::std::os::raw::c_void,
)  {
    println!("Hello message");
    let data = unsafe{ (*websocket_event).data };
    let data_len = unsafe{ (*websocket_event).numBytes };
    let data = unsafe { std::slice::from_raw_parts(data, data_len as usize) };
    let data = std::str::from_utf8(data).unwrap();
    println!("data: {}", data);
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
        toxoid_ffi::emscripten::emscripten_websocket_set_onopen_callback_on_thread(ws, user_data, callback,  toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD as *mut c_void);
        toxoid_ffi::emscripten::emscripten_websocket_set_onmessage_callback_on_thread(ws, user_data, onmessage_cb, toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD );
    }

    crate::utils::load_image("assets/character.png");
}