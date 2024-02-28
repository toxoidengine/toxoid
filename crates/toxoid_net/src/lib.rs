use core::result::Result;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use toxoid_api::*;
use toxoid_serialize::*;

pub static NETWORK_EVENT_CACHE: Lazy<Mutex<HashMap<String, extern "C" fn(message: &NetworkMessageEntity)>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn init() {}

#[no_mangle]
pub extern "C" fn toxoid_network_send(name: *const u8, name_len: usize, data: *const u8, data_len: usize) {
    // TODO: THis is gonna require reflection
    // let name = unsafe { std::slice::from_raw_parts(name, name_len) };
    // let name = std::str::from_utf8(name).unwrap();
    // let data = unsafe { std::slice::from_raw_parts(data, data_len) };
    // let network_messages = NetworkMessages {
    //     messages: vec![
    //         NetworkMessageEntity {
    //             id: 0,
    //             event: "LocalPlayerJoin".to_string(),
    //             components: vec![
    //                 NetworkMessageComponent {
    //                     name: "Position".to_string(),
    //                     object: data.to_vec()
    //                 }
    //             ]
    //         }
    //     ]
    // };
    // send(network_messages);
}

#[no_mangle]
pub extern "C" fn toxoid_network_receive(data: *const u8, len: usize) {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    receive(data.to_vec());
}

pub fn send_components(components: &[Component], event: String) {
    let mut network_message_entity = NetworkMessageEntity {
        id: id as u64,
        event: event.clone(),
        components: vec![]
    };
    let mut network_messages = NetworkMessages {
        messages: vec![network_message_entity]
    };
    components
        .iter()
        .for_each(|component| {
            // TODO: Implement get_id() for Component macro
            let id = component.get_id();
            let network_message_component = unsafe { toxoid_ffi::flecs_core::flecs_serialize_component(id) };
            network_message_entity.components.push(network_message_component);
        });
    send(network_messages);
}

pub fn send_entities(entities: &[Entity], event: String) {
    let mut network_messages = NetworkMessages {
        messages: vec![]
    };
    entities
        .iter()
        .for_each(|entity| {
            let id = entity.get_id();
            let components = unsafe { toxoid_ffi::flecs_core::flecs_serialize_entity(id) };
            network_messages.messages.push(NetworkMessageEntity {
                id: id as u64,
                event: event.clone(),
                components
            });
        });
    send(network_messages);
}

pub fn send_entity(entity: Entity, event: String) {
    // TODO: Make this the network ID, not the entity ID using hashmap
    let id = entity.get_id();
    let components = unsafe { toxoid_ffi::flecs_core::flecs_serialize_entity(id) };
    let network_messages = NetworkMessages {
        messages: vec![
            NetworkMessageEntity {
                id: id as u64,
                event,
                components
            }
        ]
    };
    send(network_messages);
}

pub fn send(network_messages: NetworkMessages) {
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

pub fn receive_entity(entity: NetworkMessageEntity) {
    // TODO: Make this the network ID, not the entity ID using hashmap
    let id = entity.id;
    let components = entity.components;
    unsafe { toxoid_ffi::flecs_core::flecs_deserialize_entity(id as ecs_entity_t, components) };
}

pub fn receive(data: Vec<u8>) {
    let network_messages = deserialize(&data)
        .unwrap();
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
