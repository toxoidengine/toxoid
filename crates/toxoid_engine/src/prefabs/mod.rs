use toxoid_api::*;

pub fn init() {
   // Parent entity
   let mut player_entity = Entity::new();
   player_entity.add::<Local>();
   player_entity.add::<Player>();
   player_entity.add::<Networked>();

   // let mut local_player = World::get_singleton::<Networked>();
   // local_player.set_entity_id(player_entity.get_id());
   // TODO: Make animation a child entity of player later
   #[cfg(feature = "render")] {
      let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json", |entity| {});
      unsafe {
         let mut position = (*player_animation_entity).get::<Position>();
         (*player_animation_entity).add::<Local>();
         println!("Player Animation Entity {:?}", (*player_animation_entity).get_id());
      }
   }
}
