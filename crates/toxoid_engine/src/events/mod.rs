pub mod player;
use player::*;

pub fn init() {
    #[cfg(feature = "client")] {
        unsafe { toxoid_net::toxoid_add_network_event("LocalPlayerJoin".to_string(), local_player_join) };
        unsafe { toxoid_net::toxoid_add_network_event("PlayerJoin".to_string(), player_join) };
        unsafe { toxoid_net::toxoid_add_network_event("PlayerMove".to_string(), player_move) };
    }
}
