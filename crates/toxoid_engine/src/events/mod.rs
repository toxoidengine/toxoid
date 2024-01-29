pub mod player;
use player::*;

pub fn init() {
    unsafe {
        #[cfg(target_os = "emscripten")]
        toxoid_ffi::ecs::toxoid_add_network_event("LocalPlayerJoin", local_player_join);
    }
}