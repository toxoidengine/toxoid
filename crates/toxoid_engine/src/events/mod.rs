pub mod player;
use player::*;

pub fn init() {
    #[cfg(feature = "client")]
    toxoid_ffi::ecs::toxoid_add_network_event("LocalPlayerJoin", local_player_join);
}