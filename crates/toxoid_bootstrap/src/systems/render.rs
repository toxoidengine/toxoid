use toxoid_api::*;
use toxoid_sokol::{SokolRenderer2D, SokolSprite};
use toxoid_render::Renderer2D;

// Rendering Systems
pub fn init() {
    System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();

    System::dsl("Sprite, Size", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let sprite = entity.get::<Sprite>();
            let sprite_box = unsafe { Box::from_raw(sprite.get_sprite() as *mut SokolSprite) };
            let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
            SokolRenderer2D::draw_sprite(sprite_trait_object, 0., 0.);
        });
    })
    .build();
}