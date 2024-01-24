pub mod join;
use join::*;

pub fn init() {
    unsafe {
        toxoid_ffi::ecs::toxoid_add_network_event("LocalPlayerJoin", local_player_join);
    }
}