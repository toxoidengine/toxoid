use toxoid_api::*;

pub fn init() {
    // Create Player entity and set singleton
    let mut player_entity = Entity::new(None);
    player_entity.add::<Player>();
    player_entity.add::<Position>();

    // Set player entity in singleton
    let player_singleton = World::get_singleton::<Player>();
    player_singleton.set_entity(player_entity.get_id());
}
