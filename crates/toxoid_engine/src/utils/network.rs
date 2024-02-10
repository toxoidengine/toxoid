// TODO: Make this file more crossplatform generic and less dependent on Emscripten
use toxoid_api::*;
use core::ffi::c_void;

#[cfg(target_os = "emscripten")]
pub fn init() {
    // WebSocket Networked Multiplayer Test
    let mut attributes = toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes {
        url: "ws://127.0.0.1:8080\0".as_ptr() as *const i8, 
        protocol: std::ptr::null()
    };
    let ws = unsafe { toxoid_ffi::emscripten::emscripten_websocket_new(&mut attributes as *mut toxoid_ffi::emscripten::EmscriptenWebSocketCreateAttributes) };
    let user_data = ws as *mut ::core::ffi::c_void;
    unsafe {
        toxoid_ffi::emscripten::emscripten_websocket_set_onopen_callback_on_thread(ws, user_data, onopen_cb,  toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD as *mut c_void);
        toxoid_ffi::emscripten::emscripten_websocket_set_onmessage_callback_on_thread(ws, user_data, onmessage_cb, toxoid_ffi::emscripten::EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD);
    }

    World::add_singleton::<WebSocket>();
    let mut websocket = World::get_singleton::<WebSocket>();
    websocket.set_socket(Pointer{ ptr: ws });
}

#[cfg(not(target_os = "emscripten"))]
pub fn init() {
}

#[cfg(target_os = "emscripten")]
pub extern "C" fn onopen_cb(
    _event_type: *mut ::std::os::raw::c_void,
    _user_data: *mut ::std::os::raw::c_void
) {
    println!("Connection opened.");
}

#[cfg(target_os = "emscripten")]
pub extern "C" fn onmessage_cb(
    _event_type: ::std::os::raw::c_int,
    websocket_event: *const toxoid_ffi::emscripten::EmscriptenWebSocketMessageEvent,
    _user_data: *mut ::std::os::raw::c_void,
)  {
    let data = unsafe{ (*websocket_event).data };
    let data_len = unsafe{ (*websocket_event).numBytes };
    let data = unsafe { std::slice::from_raw_parts(data, data_len as usize) };

    let local_player_data = World::get_singleton::<Networked>();
    let local_player_id = local_player_data.get_entity_id();
    let mut player_network_entity = Entity::from_id(local_player_id);
    let mut net = player_network_entity.get::<Networked>();

    let network_messages = toxoid_net::deserialize(data).unwrap();
    network_messages.messages.iter().for_each(|message| {
        net.set_message(Pointer{ ptr: Box::into_raw(Box::new(message)) as *mut c_void });
    });
    player_network_entity.add::<Updated>();
}
