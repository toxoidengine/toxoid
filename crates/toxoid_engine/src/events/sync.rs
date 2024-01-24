use toxoid_api::*;
use toxoid_net::NetworkMessageEntity;

pub extern "C" fn create_entity(message: &NetworkMessageEntity) {}

pub extern "C" fn sync_components(message: &NetworkMessageEntity) {}