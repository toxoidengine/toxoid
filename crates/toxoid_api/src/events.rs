use crate::*;

// pub extern "C" fn sync_components(message: &MessageEntity) {
//     let entity_id = network_entity_cache_get(message.id);
//     if entity_id != 0 {
//         deserialize_entity_sync(entity_id, message.components) 
//     }
// }

pub fn init() {
    // add_network_event("SyncComponents", sync_components);
}