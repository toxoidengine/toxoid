use crate::*;

pub struct MessageComponent {
    pub name: &'static str,
    pub data: &'static [u8]
}

pub struct MessageEntity {
    pub id: u64,
    pub event: &'static str,
    pub components: &'static [MessageComponent]
}
pub struct Messages {
    pub messages: &'static [MessageEntity]
}

pub fn send_components(entity: &mut Entity, components: &[&dyn Component], event: &str) {
    unsafe { toxoid_net_send_components(split_u64(entity.get_id()), components, event); }
}

#[cfg(all(target_arch="wasm32"))]
pub fn add_network_event(event: &str, callback: extern "C" fn(message: &MessageEntity)) {
    unsafe { toxoid_net_add_event(event, callback); }
}

// #[cfg(not(target_arch="wasm32"))]
// pub fn add_network_event(event: &str, callback: Box<dyn Fn(&crate::net::MessageEntity) + 'static>) {
//     unsafe { toxoid_net_add_event(event, callback); }
// }

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
pub fn network_entity_cache_insert(local_id: u64, network_id: u64) {
    unsafe { toxoid_network_entity_cache_insert(split_u64(local_id), split_u64(network_id)); }
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub fn network_entity_cache_insert(local_id: u64, network_id: u64) {
    unsafe { toxoid_network_entity_cache_insert(local_id, network_id) }
}

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
pub fn network_entity_cache_get(local_id: u64) -> u64 {
    unsafe { combine_u32(toxoid_network_entity_cache_get(split_u64(local_id))) }
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub fn network_entity_cache_get(local_id: u64) -> u64 {
    unsafe { toxoid_network_entity_cache_get(local_id) }
}

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
pub fn network_entity_cache_remove(local_id: u64) {
    unsafe { toxoid_network_entity_cache_remove(split_u64(local_id)) }
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub fn network_entity_cache_remove(local_id: u64) {
    unsafe { toxoid_network_entity_cache_remove(local_id) }
}

pub fn deserialize_entity_sync(entity_id: ecs_entity_t, components_serialized: &[crate::net::MessageComponent]) {
    unsafe { toxoid_deserialize_entity_sync(entity_id, components_serialized) }
}
