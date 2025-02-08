use toxoid_api::*;
use toxoid_sokol::{SokolRenderer2D, SokolSprite};
use toxoid_render::Renderer2D;

// Rect Renderer
#[components(_, Position, Size, Color, _)]
pub fn rect_render_system(iter: &Iter) {
    for (pos, size, color) in components {
        SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
    }
}

// Sprite Renderer
#[components(Sprite, Size)]
pub fn sprite_render_system(iter: &Iter) {
    for (sprite, _size) in components {
        let sprite_box = unsafe { Box::from_raw(sprite.get_sprite() as *mut SokolSprite) };
        let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
        SokolRenderer2D::draw_sprite(sprite_trait_object, 0., 0.);
    }
}

// SpineInstance, Position, BoneAnimation
use toxoid_sokol::bindings::*; 
use toxoid_sokol::sokol::app as sapp;
#[components(SpineInstance, Position, _)]
pub fn render_bone_animation(iter: &Iter) {
    println!("Render bone animation");
    for (spine_instance, position) in components {
        if spine_instance.get_instantiated() {
            unsafe {
                // Get the spine instance
                let instance = spine_instance.get_instance() as *mut sspine_instance;
                // Advance the instance animation and draw the instance.
                // Important to note here is that no actual sokol-gfx rendering happens yet,
                // instead sokol-spine will only record vertices, indices and draw commands.
                // Also, all sokol-spine functions can be called with invalid or 'incomplete'
                // handles, that way we don't need to care about whether the spine objects
                // have actually been created yet (because their data might still be loading)
                let mut delta_time = sapp_frame_duration();
                if delta_time < 0.016 {
                    delta_time = delta_time / 8.
                }
                // Update animation and record draw commands
                sspine_update_instance(*instance, delta_time as f32);
                sspine_set_position(*instance, sspine_vec2 { 
                    x: position.get_x() as f32, 
                    y: position.get_y() as f32 
                });
                // Record draw commands (but don't render yet)
                sspine_draw_instance_in_layer(*instance, 0);
            }
        }
    }
}

// Rendering Systems
pub fn init() {
    // Render systems group
    let mut render_systems_entity = Entity::named("Render Systems Group");
    
    // // Rect Renderer
    // let mut system = System::dsl("Rect, Position, Size, Color, Renderable", None, rect_render_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // // Sprite Renderer
    // let mut system = System::dsl("Sprite, Size", None, sprite_render_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // Bone Animation Renderer
    let mut system = System::dsl("SpineInstance, Position, BoneAnimation", None, render_bone_animation);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Disable render systems
    render_systems_entity.disable();

    // Set render systems group
    let mut render_systems_singleton = World::get_singleton::<RenderSystems>();
    render_systems_singleton.set_entity(render_systems_entity.get_id());
}
