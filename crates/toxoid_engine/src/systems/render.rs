use toxoid_api::*;
use toxoid_render::Renderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolSprite;

#[cfg(feature = "render")]
// Rect, Renderable, Color, Size, Position
pub fn render_rect_system(iter: &mut Iter) {
    let positions = iter.field::<Position>(5);
    let sizes = iter.field::<Size>(4);
    let colors = iter.field::<Color>(3);

    for i in 0..iter.count() {
        let pos = positions.get(i).unwrap();
        let size = sizes.get(i).unwrap();
        let color = colors.get(i).unwrap();
        
        // Draw Rect
        // #[cfg(feature = "sokol")]
        toxoid_sokol::SokolRenderer2D::draw_filled_rect(pos, size, color);
    }
}

#[cfg(feature = "render")]
// Sprite, Renderable, Size, Position
pub fn render_sprite_system(iter: &mut Iter) {
    let positions = iter.field::<Position>(4);
    let sizes = iter.field::<Size>(3);
    let sprites = iter.field::<Sprite>(1);

    for i in 0..iter.count() {
        let pos = positions.get(i).unwrap();
        let size = sizes.get(i).unwrap();
        let sprite = sprites.get(i).unwrap();
        let sprite_ptr = sprite.get_sprite();
        let sprite_box = unsafe { Box::from_raw(sprite_ptr.ptr as *mut SokolSprite) };
        let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
        // Draw Sprite
        // #[cfg(feature = "sokol")]
        toxoid_sokol::SokolRenderer2D::draw_sprite(sprite_trait_object, pos.get_x() as f32, pos.get_y() as f32);
    }
}

#[cfg(feature = "render")]
// RenderTarget, Renderable, Size, Position
pub fn render_rt_system(iter: &mut Iter) {
    let positions = iter.field::<Position>(5);
    let sizes = iter.field::<Size>(4);
    let rts = iter.field::<RenderTarget>(1);

    for i in 0..iter.count() {
        let pos = positions.get(i).unwrap();
        let size = sizes.get(i).unwrap();
        let rt = rts.get(i).unwrap();
        let rt_ptr = rt.get_render_target();
        let rt_box = unsafe { Box::from_raw(rt_ptr.ptr as *mut toxoid_sokol::SokolRenderTarget) };
        let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_box as Box<dyn toxoid_render::RenderTarget>));
        // Draw Render Target
        // #[cfg(feature = "sokol")]
        toxoid_sokol::SokolRenderer2D::draw_render_target(rt_trait_object, pos.get_x() as f32, pos.get_y() as f32, size.get_width() as f32, size.get_height() as f32);
    }
}

// SpineInstance, Position, BoneAnimation
#[cfg(all(feature = "render", feature = "spine"))]
pub fn render_bone_animation(iter: &mut Iter) {
    let spine_instances = iter.field::<SpineInstance>(1);
    let positions = iter.field::<Position>(2);
    for i in 0..iter.count() {
        use toxoid_sokol::bindings::*;
        // Get the spine instance 
        let spine_instance: &SpineInstance = spine_instances.get(i).unwrap();
        let instantiated = spine_instance.get_instantiated();
        if instantiated {
            unsafe {
                let instance = spine_instance.get_instance().ptr as *mut sspine_instance;

                // Advance the instance animation and draw the instance.
                // Important to note here is that no actual sokol-gfx rendering happens yet,
                // instead sokol-spine will only record vertices, indices and draw commands.
                // Also, all sokol-spine functions can be called with invalid or 'incomplete'
                // handles, that way we don't need to care about whether the spine objects
                // have actually been created yet (because their data might still be loading)
                let delta_time = sapp_frame_duration();
                sspine_update_instance(*instance, delta_time as f32);
                sspine_draw_instance_in_layer(*instance, 0);

                // Set position
                let pos: &Position = positions.get(i).unwrap();
                sspine_set_position(*instance, sspine_vec2 { x: pos.get_x() as f32, y: pos.get_y() as f32 });
            }
        }
    }
}