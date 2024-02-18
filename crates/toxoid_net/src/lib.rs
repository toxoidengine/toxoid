pub use flexbuffers;
use serde::{Deserialize, Serialize};
use core::result::Result;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use toxoid_api::*;

// Assuming `flexbuffers` and `NetworkMessages` are compatible with `no_std`
// You might need a custom error type since `String` and other std types are not available
#[derive(Debug)]
pub enum DeserializeError {
    FlexbufferError,
    // Other error types as needed
}

impl From<flexbuffers::SerializationError> for DeserializeError {
    fn from(_: flexbuffers::SerializationError) -> Self {
        DeserializeError::FlexbufferError
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessageComponent {
    pub name: String,
    pub object: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkMessageEntity {
    pub id: u64,
    pub event: String,
    pub components: Vec<NetworkMessageComponent>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkMessages {
    pub messages: Vec<NetworkMessageEntity>
}

pub static NETWORK_EVENT_CACHE: Lazy<Mutex<HashMap<String, extern "C" fn(message: &NetworkMessageEntity)>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn serialize(network_messages: NetworkMessages) -> Result<Vec<u8>, DeserializeError> {
    let mut s = flexbuffers::FlexbufferSerializer::new();
    network_messages.serialize(&mut s)?;
    Ok(s.view().to_vec())
}


pub fn deserialize(data: &[u8]) -> Result<NetworkMessages, DeserializeError> {
    let d = flexbuffers::Reader::get_root(data).map_err(|_| DeserializeError::FlexbufferError)?;
    NetworkMessages::deserialize(d).map_err(|_| DeserializeError::FlexbufferError)
}

#[no_mangle]
pub fn toxoid_network_send(network_messages: NetworkMessages) {
    let data = serialize(network_messages).unwrap();
    let websocket = World::get_singleton::<WebSocket>();
    #[cfg(target_os = "emscripten")]
    unsafe {
        toxoid_ffi::emscripten::emscripten_websocket_send_binary(
            websocket.get_socket().ptr, 
            data.as_ptr() as *const core::ffi::c_void, 
            data.len() as i32
        )
    };
    #[cfg(not(target_os = "emscripten"))]
    unimplemented!();
}

#[no_mangle]
pub fn toxoid_network_recv(data: Vec<u8>) {
    let network_messages = deserialize(&data).unwrap();
    network_messages
        .messages
        .iter()
        .for_each(|message| {
            unsafe {
                toxoid_run_network_event(message.event.clone(), &message);
            }
        });
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_add_network_event(
    event_name: String,
    callback: extern "C" fn(message: &NetworkMessageEntity)
) {
    let mut cache = NETWORK_EVENT_CACHE.lock().unwrap();
    cache.insert(event_name, callback);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_run_network_event(
    event_name: String,
    message: &NetworkMessageEntity
) {
    let cache = NETWORK_EVENT_CACHE.lock().unwrap();
    if let Some(event_cb) = cache.get(&event_name[..]) {
        event_cb(message);
    } else {
        eprintln!("Event not found: {:?}", event_name);
    }
}
