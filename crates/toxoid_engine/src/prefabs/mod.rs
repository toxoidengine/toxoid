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
   sprite_entity.add::<BlendMode>();
   
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

   let lamp_post_sprite = create_sprite("assets/street_lamp.png", |entity| {
      (*entity).add::<Blittable>();
   });
   lamp_post_sprite.get::<Position>()
      .set_x(200);
   lamp_post_sprite.get::<Position>()
      .set_y(200);

   let mut light_sprite = create_sprite("assets/light_yellow_2.png", |entity| {
         (*entity).add::<Blittable>();
   });
   light_sprite.add::<Light>();
   light_sprite.get::<BlendMode>()
      .set_mode(BlendModes::Add as u8);
   light_sprite.get::<Position>()
      .set_x(60);
   light_sprite.get::<Position>()
      .set_y(55);

   let lamp_post_sprite = create_sprite("assets/street_lamp.png", |entity| {
      (*entity).add::<Blittable>();
   });
   lamp_post_sprite.get::<Position>()
      .set_x(400);
   lamp_post_sprite.get::<Position>()
      .set_y(200);

   let mut light_sprite = create_sprite("assets/light_yellow.png", |entity| {
         (*entity).add::<Blittable>();
   });
   light_sprite.add::<Light>();
   light_sprite.get::<BlendMode>()
      .set_mode(BlendModes::Add as u8);
   light_sprite.get::<Position>()
      .set_x(260);
   light_sprite.get::<Position>()
      .set_y(55);

   let mut rect_entity = Entity::new();
   rect_entity.set_name("rect");
   // Rect, Renderable, Color, Size, Position
   rect_entity.add::<Rect>();
   rect_entity.add::<Blittable>();
   rect_entity.add::<Color>();
   rect_entity.add::<Size>();
   rect_entity.add::<Position>();

   let mut size = rect_entity.get::<Size>();
   size.set_width(2000);
   size.set_height(2000);
   
   let mut color = rect_entity.get::<Color>();
   // Transparent black
   color.set_r(0.);
   color.set_g(0.);
   color.set_b(0.);
   color.set_a(0.8);

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