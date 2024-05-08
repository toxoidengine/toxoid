pub mod player;
use player::*;
use toxoid_api::*;

pub fn init() {
    #[cfg(feature = "client")] {
        add_network_event("LocalPlayerJoin", local_player_join);
        add_network_event("PlayerJoin", player_join);
        add_network_event("PlayerMove", player_move);
    }
}
