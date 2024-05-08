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

pub extern "C" fn event_sync_components(message: &MessageEntity) {
    println!("SyncComponents");
}

pub fn init() {
    add_network_event("SyncComponents", event_sync_components);
}

pub fn send_components(entity: &mut Entity, components: &[&dyn Component], event: &str) {
    unsafe { toxoid_net_send_components(split_u64(entity.get_id()), components, event); }
}

pub fn add_network_event(event: &str, callback: extern "C" fn(message: &MessageEntity)) {
    unsafe { toxoid_net_add_event(event, callback); }
}