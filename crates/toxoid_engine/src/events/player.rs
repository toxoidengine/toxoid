use toxoid_api::{*, split_u64};
#[cfg(feature = "client")]
use toxoid_serialize::NetworkMessageEntity;

#[cfg(feature = "client")]
pub extern "C" fn local_player_join(message: &NetworkMessageEntity) {
    // println!("Local player ID received: {:?}", message.id);
    // // Set local player ID
    // let mut local_player = World::get_singleton::<Networked>();
    // local_player.set_network_id(message.id);

    // // Create entity
    // let render_entity = crate::utils::load::load_image("assets/character.png");
    // render_entity.add::<Local>();

    // // Add to network entity cache
    // unsafe { toxoid_network_entity_cache_insert(split_u64(message.id), split_u64(render_entity.get_id())) };
}


#[cfg(feature = "client")]
pub extern "C" fn player_join(message: &NetworkMessageEntity) {
    // println!("Player ID received: {:?}", message.id);
    // // Create entity
    // let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    // // let mut position = player_animation_entity.get::<Position>();
    // // position.set_x(100);
    // // position.set_y(100);
    // // player_animation_entity.add::<Remote>();
    
    // // Update position
    // // let deserialized_component = message.components[0].clone();
    // // let mut position = player_animation_entity.get::<Position>();
    // // position.set_x(deserialized_component.x);
    // // position.set_y(deserialized_component.y);
    
    // // Add to network entity cache
    // unsafe { toxoid_network_entity_cache_insert(split_u64(message.id), split_u64(player_animation_entity.get_id())) };
}


#[cfg(feature = "client")]
pub extern "C" fn player_leave(message: &NetworkMessageEntity) {
    println!("Player ID {:?} disconnected from server.", message.id);
}