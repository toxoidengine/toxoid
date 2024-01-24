use toxoid_api::{*, split_u64};
use toxoid_net::NetworkMessageEntity;

pub extern "C" fn local_player_join(message: &NetworkMessageEntity) {
    println!("Local player ID received: {:?}", message.id);
    // Set local player ID
    let mut local_player = World::get_singleton::<Networked>();
    local_player.set_id(message.id);

    // Create entity
    let render_entity = crate::utils::load::load_image("assets/character.png");
    render_entity.add::<Local>();

    // Add to network entity cache
    unsafe { toxoid_network_entity_cache_insert(split_u64(message.id), split_u64(render_entity.get_id())) };
}