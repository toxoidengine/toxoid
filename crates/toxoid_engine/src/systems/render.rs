use toxoid_api::*;
use toxoid_render::Renderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolSprite;

#[cfg(feature = "render")]
// Rect, Renderable, Color, Size, Position
pub fn render_rect_system(iter: &mut Iter) {
    let entities = iter.entities();
    entities
        .iter()
        .for_each(|entity| {
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            
            // Draw Rect
            // #[cfg(feature = "sokol")]
            toxoid_sokol::SokolRenderer2D::draw_filled_rect(pos, size, color);
        });
}

#[cfg(feature = "render")]
// Sprite, Renderable, Size, Position
pub fn render_sprite_system(iter: &mut Iter) {
    let entities = iter.entities();
    entities
        .iter()
        .for_each(|entity| {
            let sprite = entity.get::<Sprite>();
            let pos = entity.get::<Position>();
            let sprite_ptr = sprite.get_sprite();
            let sprite_box = unsafe { Box::from_raw(sprite_ptr.ptr as *mut SokolSprite) };
            let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
            // Draw Sprite
            // #[cfg(feature = "sokol")]
            toxoid_sokol::SokolRenderer2D::draw_sprite(sprite_trait_object, pos.get_x() as f32, pos.get_y() as f32);
        });

}
#[cfg(feature = "render")]
// RenderTarget, Renderable, Size, Position
pub fn render_rt_system(iter: &mut Iter) {
    println!("Rendering Render Target!");
    let entities = iter.entities();
    entities
        .iter()
        .for_each(|entity| {
            let rt = entity.get::<RenderTarget>();
            let pos = entity.get::<Position>();
            // let rt_ptr = unsafe { &*(rt.get_render_target().ptr as *mut toxoid_sokol::SokolRenderTarget) };
            let rt_boxed: Box<dyn toxoid_render::RenderTarget> = unsafe { Box::from_raw(rt.get_render_target().ptr as *mut toxoid_sokol::SokolRenderTarget) };
            // Draw Sprite
            // #[cfg(feature = "sokol")]
            // toxoid_sokol::SokolRenderer2D::draw_sprite(sprite_trait_object, pos.get_x() as f32, pos.get_y() as f32);
            toxoid_sokol::SokolRenderer2D::draw_render_target(&rt_boxed, pos.get_x() as f32, pos.get_y() as f32, 500., 500.);
            std::mem::forget(rt_boxed);
        });
}

#[cfg(all(feature = "render", feature = "spine"))]
pub fn draw_bone_animation(iter: &mut Iter) {
    let entities = iter.entities();
    entities
        .iter_mut()
        .for_each(|entity| {
            use toxoid_sokol::bindings::*;
            let spine_instance = entity.get::<SpineInstance>();
            let pos = entity.get::<Position>();
            let instance = spine_instance.get_instance().ptr as *mut sspine_instance;
            unsafe { 
                let delta_time = sapp_frame_duration();
                sspine_set_position(*instance, sspine_vec2 { x: pos.get_x() as f32, y: pos.get_y() as f32 });
                // Advance the instance animation and draw the instance.
                // Important to note here is that no actual sokol-gfx rendering happens yet,
                // instead sokol-spine will only record vertices, indices and draw commands.
                // Also, all sokol-spine functions can be called with invalid or 'incomplete'
                // handles, that way we don't need to care about whether the spine objects
                // have actually been created yet (because their data might still be loading)
                sspine_update_instance(*instance, delta_time as f32);
                sspine_draw_instance_in_layer(*instance, 0); 
            };
        });
}