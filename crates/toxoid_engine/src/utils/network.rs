// TODO: Make this file more crossplatform generic and less dependent on Emscripten
use toxoid_api::*;
use core::ffi::c_void;
use std::collections::HashMap;

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
    let network_messages = toxoid_serialize::deserialize(data).unwrap();
    network_messages.messages.iter().for_each(|message| {
        unsafe { toxoid_net::toxoid_net_run_event(message.event.clone(), message) };
    });
}

pub fn serialize_entity(entity_id: ecs_entity_t) -> Vec<toxoid_serialize::NetworkMessageComponent> {
    unsafe { toxoid_ffi::ecs::toxoid_serialize_entity(entity_id) }
}

pub fn serialize_component(entity_id: ecs_entity_t, component_id: ecs_entity_t) -> toxoid_serialize::NetworkMessageComponent {
    unsafe { toxoid_ffi::ecs::toxoid_serialize_component(entity_id, component_id) }
}

pub fn deserialize_entity(components_serialized: &[toxoid_api::MessageComponent]) -> HashMap<std::string::String, HashMap<std::string::String, toxoid_ffi::ecs::DynamicType>> {
    unsafe { toxoid_ffi::ecs::toxoid_deserialize_entity(components_serialized) }
}

pub fn deserialize_entity_sync(entity_id: ecs_entity_t, components_serialized: &[MessageComponent]) {
    unsafe { toxoid_ffi::ecs::toxoid_deserialize_entity_sync(entity_id, components_serialized) }
}