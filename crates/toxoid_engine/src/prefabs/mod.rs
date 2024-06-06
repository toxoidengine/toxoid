use toxoid_api::*;

use toxoid_api::*;
#[cfg(feature = "render")]
use toxoid_render::Renderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolRenderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolSprite;

pub struct CallbackFn {
   pub callback: Box<dyn FnMut(&mut Entity)>
}

pub fn init() {
   // Player entity
   let mut player_entity = Entity::new();
   player_entity.set_name("player");
   player_entity.add::<Local>();
   player_entity.add::<Player>();
   player_entity.add::<Networked>();

   // Sprite entity
   let mut sprite_entity = Entity::new();
   sprite_entity.set_name("sprite");
   sprite_entity.add::<Sprite>();
   sprite_entity.add::<Position>();
   sprite_entity.add::<Size>();
   sprite_entity.add::<Loadable>();
   sprite_entity.add::<Callback>();
   sprite_entity.get::<Sprite>()
      .set_filename(StringPtr::new("assets/character.png"));
   let callback = Box::new(|entity: &mut Entity| {
      entity.add::<Renderable>();
   });
   let callback = Box::into_raw(Box::new(CallbackFn { callback })) as *mut c_void;
   sprite_entity.get::<Callback>()
      .set_callback(
         Pointer::new(
            callback
         )
      );
   sprite_entity.child_of_by_id(player_entity.get_id());

   #[cfg(feature = "render")] {
      let animation_entity = crate::utils::load::load_animation("assets/player.atlas", "assets/player.json", |entity| {
      });
      unsafe {
         let mut position = (*animation_entity).get::<Position>();
         position.set_x(100);
         position.set_y(100);
         (*animation_entity).add::<Local>();
         (*animation_entity).child_of_by_id(player_entity.get_id());
      }
   }

   #[cfg(feature = "render")]
   crate::utils::load::load_worldmap("assets/world_1.world", |world_entity: &mut Entity| {});
}