use core::result::Result;
use once_cell::sync::Lazy;
use toxoid_ffi::flecs_core::EcsWorld;
use std::sync::Mutex;
use std::collections::HashMap;
use toxoid_api::*;
use toxoid_serialize::*;

pub static NETWORK_EVENT_CACHE: Lazy<Mutex<HashMap<String, extern "C" fn(message: &MessageEntity)>>> = Lazy::new(|| Mutex::new(HashMap::new()));

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

pub fn send_components(entity_id: ecs_entity_t, components: &[&dyn Component], event: String) {
    // Server does not need to know about network ids as the server
    // is authoritative
    // let network_entity_cache = toxoid_ffi::ecs::NETWORK_ENTITY_CACHE.lock().unwrap();
    // let network_id = network_entity_cache.get(&entity_id).map_or_else(|| {
    //     eprintln!("Network ID not found for entity: {:?}", entity_id);
    //     // Provide a default value or handle the error appropriately
    //     0 // Assuming 0 is a safe default
    // }, |id| *id);
    let components_vec: Vec<NetworkMessageComponent> = components
        .iter()
        .map(|component| {
            let component_id = component.get_id();
            if component.get_singleton() {
                unsafe { toxoid_ffi::ecs::toxoid_serialize_component(EcsWorld, component_id) }
            } else {
                unsafe { toxoid_ffi::ecs::toxoid_serialize_component(entity_id, component_id) }
            }
        })
        .collect();

    let network_message_entity = NetworkMessageEntity {
        // Server does not need to know about network ids as the server
        // is authoritative
        // id: network_id,
        id: 0,
        event: event.clone(),
        components: components_vec,
    };

    let network_messages = NetworkMessages {
        messages: vec![network_message_entity]
    };

    send(network_messages);
}

pub fn send_entities(entity_ids: &[ecs_entity_t], event: String) {
    let mut network_messages = NetworkMessages {
        messages: vec![]
    };
    entity_ids
        .iter()
        .for_each(|entity_id| {
            let components = unsafe { toxoid_ffi::ecs::toxoid_serialize_entity(*entity_id) };
            let network_entity_cache = toxoid_ffi::ecs::NETWORK_ENTITY_CACHE.lock().unwrap();
            let network_id = network_entity_cache.get(&entity_id).unwrap();
            network_messages.messages.push(NetworkMessageEntity {
                id: *network_id,
                event: event.clone(),
                components
            });
        });
    send(network_messages);
}

pub fn send_entity(entity_id: ecs_entity_t, event: String) {
    // TODO: Make this the network ID, not the entity ID using 
    let network_entity_cache = toxoid_ffi::ecs::NETWORK_ENTITY_CACHE.lock().unwrap();
    let network_id = network_entity_cache.get(&entity_id).unwrap();
    let components = unsafe { toxoid_ffi::ecs::toxoid_serialize_entity(entity_id) };
    let network_messages = NetworkMessages {
        messages: vec![
            NetworkMessageEntity {
                id: *network_id,
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
    println!("Connected: {:?}", websocket.get_connected());
    // Check if the WebSocket is in a ready state
    if !websocket.get_connected() {
        return;
    }
    #[cfg(target_os = "emscripten")]
    unsafe {
        let result = toxoid_ffi::emscripten::emscripten_websocket_send_binary(
            websocket.get_socket().ptr, 
            data.as_ptr() as *const core::ffi::c_void, 
            data.len() as i32
        );
    };

    #[cfg(not(target_os = "emscripten"))]
    unimplemented!();
}

// pub fn receive_entity(entity: NetworkMessageEntity) {
//     // TODO: Make this the network ID, not the entity ID using hashmap
//     let id = entity.id;
//     let components = entity.components;
//     unsafe { toxoid_ffi::flecs_core::flecs_deserialize_entity(components) };
// }

pub fn receive(data: Vec<u8>) {
    let network_messages = deserialize(&data)
        .unwrap();
    network_messages
        .messages
        .iter()
        .for_each(|message| {
            unsafe {
                toxoid_net_run_event(message.event.clone(), &message);
            }
        });
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_net_add_event(
    event_name: &'static str,
    callback: extern "C" fn(message: &MessageEntity)
) {
    let mut cache = NETWORK_EVENT_CACHE.lock().unwrap();
    cache.insert(event_name.to_string(), callback);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_net_run_event(
    event_name: String,
    message: &NetworkMessageEntity
) {
    let cache = NETWORK_EVENT_CACHE.lock().unwrap();
    if let Some(event_cb) = cache.get(&event_name[..]) {
        let event_name = Box::leak(event_name.into_boxed_str());
        let components: Vec<MessageComponent> = message
            .components
            .iter()
            .map(|component| {
                let name = Box::leak(component.name.clone().into_boxed_str());
                let data = Box::leak(component.data.to_vec().into_boxed_slice());

                MessageComponent {
                    name: name,
                    data: data
                }
            })
            .collect();
        let message = MessageEntity {
            id: message.id,
            event: event_name,
            components: Box::leak(components.into_boxed_slice())
        };

        event_cb(&message);

        // // Convert the `&'static MessageEntity` back into a `Box<MessageEntity>`
        // let message_boxed = unsafe { Box::from_raw(&message as *const MessageEntity as *mut MessageEntity) };

        // // Dropping the box will also drop `components` and `event` inside `MessageEntity`
        // drop(message_boxed)
    } else {
        eprintln!("Event not found: {:?}", event_name)
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn toxoid_net_send_components(
    entity_id: SplitU64, 
    components: &[&dyn Component], 
    event: &str
) {
    send_components(combine_u32(entity_id), components, event.to_string());
}

