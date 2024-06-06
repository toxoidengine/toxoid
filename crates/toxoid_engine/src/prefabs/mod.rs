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

pub fn create_sprite(filename: &str, callback: impl FnMut(&mut Entity) + 'static) -> Entity {
   // Create sprite entity
   let mut sprite_entity = Entity::new();
   sprite_entity.set_name("sprite");
   sprite_entity.add::<Sprite>();
   sprite_entity.add::<Position>();
   sprite_entity.add::<Size>();
   sprite_entity.add::<Loadable>();
   sprite_entity.add::<Callback>();
   
   // Set sprite filename
   sprite_entity.get::<Sprite>()
      .set_filename(StringPtr::new(filename));

   // Set callback
   let callback = Box::new(callback);
   let callback = Box::into_raw(Box::new(CallbackFn { callback })) as *mut c_void;
   sprite_entity.get::<Callback>()
      .set_callback(
         Pointer::new(
            callback
         )
      );

   // Return sprite entity
   sprite_entity
}

pub fn init() {
   // Player entity
   let mut player_entity = Entity::new();
   player_entity.set_name("player");
   player_entity.add::<Local>();
   player_entity.add::<Player>();
   player_entity.add::<Networked>();

   let _sprite_entity = create_sprite("assets/character.png", |entity: &mut Entity| {
      entity.add::<Renderable>();
   });

   #[cfg(feature = "render")] {
      let animation_entity = crate::utils::load::load_animation("assets/player.atlas", "assets/player.json", |entity| {
      });
      unsafe {
         let mut position = (*animation_entity).get::<Position>();
         position.set_x(100);
         position.set_y(100);
         (*animation_entity).add::<Local>();
      }
   }

   #[cfg(feature = "render")]
   crate::utils::load::load_worldmap("assets/world_1.world", |world_entity: &mut Entity| {});
}