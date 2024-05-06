use crate::*;

pub fn send_components(entity: &mut Entity, components: &[&dyn Component], event: &str) {
    unsafe { toxoid_net_send_components(split_u64(entity.get_id()), components, event); }
}