use toxoid_api::{*, split_u64};

#[cfg(feature = "client")]
pub extern "C" fn local_player_join(message: &MessageEntity) {
    // Set local player ID
    let mut local_player = World::get_singleton::<Networked>();
    local_player.set_network_id(message.id);
    
    // Create entity
    // let render_entity = crate::utils::load::load_image("assets/character.png");
    // render_entity.add::<Local>();

    // Add to network entity cache
    unsafe { toxoid_network_entity_cache_insert(split_u64(local_player.get_entity_id()), split_u64(message.id)) };
}


#[cfg(feature = "client")]
pub extern "C" fn player_join(message: &MessageEntity) {
    #[cfg(feature = "render")] {
        let entity = crate::utils::load::load_sprite("assets/character.png", |entity: &mut Entity| {
            let mut position = entity.get::<Position>();
            position.set_x(0);
            position.set_y(0);
            entity.add::<Renderable>();
            entity.add::<Player>();
            entity.add::<Direction>();
        });
        unsafe { toxoid_network_entity_cache_insert(split_u64(message.id), split_u64((*entity).get_id())) };
    }
}

#[cfg(feature = "client")]
pub extern "C" fn player_leave(message: &MessageEntity) {
    println!("Player ID {:?} disconnected from server.", message.id);
}

#[cfg(feature = "client")]
pub extern "C" fn player_move(message: &MessageEntity) {
    let entity_id = combine_u32(unsafe { toxoid_network_entity_cache_get(split_u64(message.id)) });
    if entity_id != 0 {
        unsafe { toxoid_ffi::flecs_core::flecs_deserialize_entity_sync(entity_id, message.components.clone()) };
    }
}

