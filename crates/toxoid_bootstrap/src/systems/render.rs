use toxoid_api::*;
use toxoid_sokol::{bindings::*, SokolRenderTarget, SokolRenderer2D, SokolSprite};
use toxoid_render::Renderer2D;
use crate::prefabs::*;

// // Rect Renderer
// #[components(_, Position, Size, Color, _)]
// pub fn rect_render_system(iter: &Iter) {
//     for (pos, size, color) in components {
//         SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
//     }
// }

// // Sprite Renderer
// #[components(Sprite, Size)]
// pub fn sprite_render_system(iter: &Iter) {
//     for (sprite, _size) in components {
//         let sprite_box = unsafe { Box::from_raw(sprite.get_sprite() as *mut SokolSprite) };
//         let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
//         SokolRenderer2D::draw_sprite(sprite_trait_object, 0., 0.);
//     }
// }

// // SpineInstance, Position, BoneAnimation
// #[components(SpineInstance, Position, _)]
// pub fn render_bone_animation(iter: &Iter) {
//     for (spine_instance, position) in components {
//         if spine_instance.get_instantiated() {
//             unsafe {
//                 // Get the spine instance
//                 let instance = spine_instance.get_instance() as *mut sspine_instance;
//                 // Advance the instance animation and draw the instance.
//                 // Important to note here is that no actual sokol-gfx rendering happens yet,
//                 // instead sokol-spine will only record vertices, indices and draw commands.
//                 // Also, all sokol-spine functions can be called with invalid or 'incomplete'
//                 // handles, that way we don't need to care about whether the spine objects
//                 // have actually been created yet (because their data might still be loading)
//                 let mut delta_time = sapp_frame_duration();
//                 if delta_time < 0.016 {
//                     delta_time = delta_time / 8.
//                 }
//                 // Update animation and record draw commands
//                 sspine_update_instance(*instance, delta_time as f32);
//                 sspine_set_position(*instance, sspine_vec2 { 
//                     x: position.get_x() as f32, 
//                     y: position.get_y() as f32 
//                 });
//                 // Record draw commands (but don't render yet)
//                 sspine_draw_instance_in_layer(*instance, 0);
//             }
//         }
//     }
// }

// Blit sprite to render target
#[components(Sprite, _, Size, _, RenderTarget, Size)]
pub fn blit_sprite_system(iter: &Iter) {
    let mut entities = iter.entities();
    for (i, (sprite, size, render_target, rt_size)) in components.into_iter().enumerate() {
        // Get render target pointer / object / box / trait object
        let rt_ptr = render_target.get_render_target();
        let rt_ptr_box = unsafe { Box::from_raw(rt_ptr as *mut SokolRenderTarget) };
        let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_ptr_box as Box<dyn toxoid_render::RenderTarget>));
        // Get sprite size
        let width = size.get_width();
        let height = size.get_height();
        // Get render target size
        let rt_width = rt_size.get_width();
        let rt_height = rt_size.get_height();
        // Get sprite pointer / object / box / trait object
        let sprite_ptr = sprite.get_sprite();
        let sprite_box = unsafe { Box::from_raw(sprite_ptr as *mut SokolSprite) };
        let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
        // Begin render target
        SokolRenderer2D::begin_rt(&rt_trait_object, rt_width as f32, rt_height as f32);
        // Blit sprite to render target
        SokolRenderer2D::blit_sprite(sprite_trait_object, 0., 0., width as f32, height as f32, rt_trait_object, 0., 0.);
        // End render target
        SokolRenderer2D::end_rt();
        entities[i].remove::<Blittable>();
    }
}

// Blit cell to render target
#[components(TiledCell, _, Size)]
pub fn blit_cell_system(iter: &Iter) {
    let mut entities = iter.entities();
    for (i, (cell, size)) in components.into_iter().enumerate() {
        let cell_entity = entities.get_mut(i).unwrap();
        let mut children = cell_entity.children();
        if children.len() > 0 {
            // TODO: Make this into Flecs Query with custom relationships
            let mut tileset_entities = children.iter_mut().filter(|child| child.has::<Tileset>() && child.has::<Blittable>()).collect::<Vec<_>>();
            let mut children = cell_entity.children();
            let mut render_target_entities = children.iter_mut().filter(|child| child.has::<RenderTarget>()).collect::<Vec<_>>();

            if tileset_entities.len() > 0 && render_target_entities.len() > 0 {
                // Get tileset 
                let tileset_entity = tileset_entities.get_mut(0).unwrap();
                let sprite = tileset_entity.get::<Sprite>();
                
                let sprite_ptr = sprite.get_sprite();
                let sprite_box = unsafe { Box::from_raw(sprite_ptr as *mut SokolSprite) };
                let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
                let tileset_size = tileset_entity.get::<Size>();
                let width = tileset_size.get_width();
                let height = tileset_size.get_height();

                let rt_entity = render_target_entities.get_mut(0).unwrap();
                let render_target = rt_entity.get::<RenderTarget>();
                let render_target_ptr = render_target.get_render_target();
                let render_target_box = unsafe { Box::from_raw(render_target_ptr as *mut SokolRenderTarget) };
                let render_target_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(render_target_box as Box<dyn toxoid_render::RenderTarget>));
                
                SokolRenderer2D::begin_rt(&render_target_trait_object, width as f32, height as f32);
                SokolRenderer2D::blit_sprite(sprite_trait_object, 0., 0., width as f32, height as f32, render_target_trait_object, 0., 0.);
                SokolRenderer2D::end_rt();
                rt_entity.add::<Renderable>();
            }

            // let render_target_entities = children.iter_mut().filter(|child| child.has::<RenderTarget>()).collect::<Vec<_>>();
            // children.iter_mut().for_each(|child| {
            //     if child.has::<Tileset>() && child.has::<Blittable>() {
            //         println!("Tileset: {:?}", child.get_id());

            //         // let mut sprite = child.get::<Sprite>();
            //         // let sprite_ptr = sprite.get_sprite();
            //         // let sprite_box = unsafe { Box::from_raw(sprite_ptr as *mut SokolSprite) };
            //         // let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
            //         // println!("Sprite: {:?}", sprite_ptr);

            //         // SokolRenderer2D::begin_rt(&rt_trait_object, rt_width as f32, rt_height as f32);
            //         // SokolRenderer2D::blit_sprite(sprite_trait_object, 0., 0., width as f32, height as f32, rt_trait_object, 0., 0.);
            //         // SokolRenderer2D::end_rt();
            //     }

            //     if child.has::<RenderTarget>() {
            //         println!("Render Target: {:?}", child.get_id());
            //         // let render_target = child.get::<RenderTarget>();
            //         // let render_target_ptr = render_target.get_render_target();
            //         // let render_target_box = unsafe { Box::from_raw(render_target_ptr as *mut SokolRenderTarget) };
            //         // let render_target_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(render_target_box as Box<dyn toxoid_render::RenderTarget>));
            //         // println!("Child: {}", child.get_id());
            //         // entity.remove::<Blittable>();
            //     }
            // });
        }
        // Get render target pointer / object / box / trait object
        // let rt_ptr = render_target.get_render_target();
        // let rt_ptr_box = unsafe { Box::from_raw(rt_ptr as *mut SokolRenderTarget) };
        // let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_ptr_box as Box<dyn toxoid_render::RenderTarget>));
    }
}

// Sort Render Targets by Z-Index
// TODO: Use query trampoline instead of C functions directly and use callback resource to make this work on WASM.
pub extern "C" fn draw_render_target_sort(_e1: ecs_entity_t, v1: *const std::ffi::c_void, _e2: ecs_entity_t, v2: *const std::ffi::c_void) -> i32 {
    let mut rt1 = RenderTarget::default();
    let mut rt2 = RenderTarget::default();
    let rt1_component = ToxoidComponent::from_ptr_host(v1 as u64);
    let rt2_component = ToxoidComponent::from_ptr_host(v2 as u64);
    rt1.set_component(rt1_component);
    rt2.set_component(rt2_component);
    let z1 = rt1.get_z_index();
    let z2 = rt2.get_z_index();
    z1.cmp(&z2) as i32
}

// Draw Render Targets to screen as final output
#[components(RenderTarget, _, Size, Position, BlendMode)]
pub fn draw_render_targets_system(iter: &Iter) {
    for (rt, size, position, blend_mode) in components {
        // Get render target object / pointer
        let rt_ptr = rt.get_render_target();
        let rt_ptr_box = unsafe { Box::from_raw(rt_ptr as *mut SokolRenderTarget) };
        let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_ptr_box as Box<dyn toxoid_render::RenderTarget>));
        // Get blend mode
        let blend_mode = blend_mode.get_blend_mode();
        // Get size
        let width = size.get_width();
        let height = size.get_height();
        // Get position
        let x = position.get_x();
        let y = position.get_y();
        // Draw render target
        SokolRenderer2D::draw_render_target(rt_trait_object, 0., 0., width as f32, height as f32, x as f32, y as f32, width as f32, height as f32, blend_mode);
    }
}

// Systems that draw render targets to the screen as a final output
pub fn draw_systems(render_systems_entity: &mut Entity) {
    // // Rect Renderer
    // let mut system = System::dsl("Rect, Position, Size, Color, Renderable", None, rect_render_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // // Sprite Renderer
    // let mut system = System::dsl("Sprite, Size", None, sprite_render_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // // Bone Animation Renderer
    // let mut system = System::dsl("SpineInstance, Position, BoneAnimation", None, render_bone_animation);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // Draw Render Targets
    let mut system = System::dsl("RenderTarget, Renderable, Size, Position, BlendMode", None, draw_render_targets_system);
    system.order_by(RenderTarget::get_id(), draw_render_target_sort);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());
}

pub fn test_system(iter: &Iter) {
    let mut entities = iter.entities();
    println!("Entities: {:?}", entities.len());
}

// Systems that blit render targets
pub fn blit_systems(render_systems_entity: &mut Entity) {
    // let mut system = System::dsl("Sprite, Blittable, Position, Size", None, blit_sprite_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());

    // Blit sprite to render target
    let mut system = System::dsl("Sprite, Blittable, Size, (ChildOf, $Parent), RenderTarget($Parent), Size($Parent)", None, blit_sprite_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Blit cell to render target
    let mut system = System::dsl("TiledCell, Blittable, Size", None, blit_cell_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());
}

// Rendering Systems
pub fn init() {
    // Render systems group
    let mut render_systems_entity = Entity::named("Render Systems Group");
    
    // Systems
    // Blit systems
    blit_systems(&mut render_systems_entity);
    // Draw systems
    draw_systems(&mut render_systems_entity);

    // Disable render systems
    render_systems_entity.disable();

    // Set render systems group
    let mut render_systems_singleton = World::get_singleton::<RenderSystems>();
    render_systems_singleton.set_entity(render_systems_entity.get_id());
}
