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
                let tileset_sprite: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
                let tileset_size = tileset_entity.get::<Size>();
                let width = tileset_size.get_width();
                let height = tileset_size.get_height();

                let rt_entity = render_target_entities.get_mut(0).unwrap();
                let render_target = rt_entity.get::<RenderTarget>();
                let render_target_ptr = render_target.get_render_target();
                let render_target_box = unsafe { Box::from_raw(render_target_ptr as *mut SokolRenderTarget) };
                let render_target_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(render_target_box as Box<dyn toxoid_render::RenderTarget>));
                let rt_width = 800.;
                let rt_height = 600.;

                let cell = cell_entity.get::<TiledCell>();
                let cell = cell.get_cell() as *mut toxoid_tiled::TiledCell;
                let pixel_width = unsafe { (*cell).width * (*cell).tilewidth };
                let pixel_height = unsafe { (*cell).height * (*cell).tileheight };
                let tile_width = unsafe { (*cell).tilewidth };
                let tile_height = unsafe { (*cell).tileheight };
                let image_width = 4800;
                let image_height = 720;
                let rt = SokolRenderer2D::create_render_target(pixel_width, pixel_height);
                SokolRenderer2D::begin_rt(&rt, pixel_width as f32, pixel_height as f32);
                unsafe {
                    // Set proper blend mode before drawing tiles
                    (*cell)
                        .layers
                        .iter()
                        .for_each(|layer| {
                            if layer.layer_type == "group" {
                                layer
                                    .layers
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .for_each(|layer| {
                            if layer.layer_type == "tilelayer" {
                                let height = (*cell).height;
                                let width = (*cell).width;
                                // Iterate over the tiles in the map
                                for y in 0..height {
                                    for x in 0..width {
                                        // Calculate the position to blit each tile on the render target
                                        let dest_x = x as f32 * tile_width as f32;
                                        let dest_y = y as f32 * tile_height as f32;
                                        let i = x as usize + (y as usize * width as usize);
                                        let tile_id = layer.data.as_ref().unwrap()[i];
                                        // Tiled 1-indexes the tile ids
                                        // and 0 is a special value for an empty tile
                                        if tile_id == 0 {
                                            continue;
                                        }
                                        // Calculate x and y position of the tile in the tileset
                                        // Based on the tile id which is the index of the tile in the tileset
                                        // Assuming the tileset is a single row of tiles
                                        // You may need to adjust this based on the tileset layout
                                        // Calculate the source x and y position of the tile in the tileset
                                        let tileset_width = image_width / tile_width;
                                        let tileset_x = (tile_id - 1) % tileset_width;
                                        let tileset_y = (tile_id - 1) / tileset_width;
                                        let src_x = tileset_x as f32 * tile_width as f32;
                                        let src_y = tileset_y as f32 * tile_height as f32;
                                        
                                        // Blit tile from the tileset to the map's render target
                                        // Assuming you have a way to determine the source tile's position in the tileset, adjust src_x and src_y accordingly
                                        SokolRenderer2D::blit_sprite(&tileset_sprite, src_x, src_y, tile_width as f32, tile_height as f32, &rt, dest_x, dest_y);
                                        
                                        // Create an entity for each tile
                                        let mut tile_entity = Entity::new(None);
                                        tile_entity.add::<Position>();
                                        let mut position = tile_entity.get::<Position>();
                                        position.set_x(dest_x as u32);
                                        position.set_y(dest_y as u32);
                                        tile_entity.add::<Size>();
                                        let mut size = tile_entity.get::<Size>();
                                        size.set_width(tile_width);
                                        size.set_height(tile_height);
                                        // Add other components as needed, e.g., for collision checks
                                    }
                                }
                            } else if layer.layer_type == "objectgroup" {
                                layer
                                    .objects
                                    .as_ref()
                                    .unwrap()
                                    .iter()
                                    .for_each(|object| {
                                        object
                                        .properties
                                        .as_ref()
                                        .unwrap()
                                        .iter()
                                        .for_each(|property| {
                                            if property.name == "entity" {
                                                // println!("Entity: {}", property.value.as_str());
                                                // toxoid_json_to_entity(
                                                //     make_c_string(
                                                //         property.value.as_str()
                                                //     )
                                                // );
                                            }
                                            // if property.name == "filename" {
                                                
                                            // }
                                            });
                                        });
                                    }
                            });
                        }
                    });
                }
                SokolRenderer2D::end_rt();

                // Create render target entity
                let mut entity = Entity::new(None);
                entity.add::<RenderTarget>();
                entity.add::<Position>();
                entity.add::<Size>();
                entity.add::<BlendMode>();
                
                let mut position = entity.get::<Position>();
                // Assuming grid dimensions are known
                let grid_width = 2; // Number of cells horizontally
                let index = cell_entity.get::<TiledCell>().get_index();
                // Calculate grid positions based on the index
                let grid_x = index % grid_width; // Calculate x position in the grid
                let grid_y = index / grid_width; // Calculate y position in the grid
                let position_x = grid_x as u32 * pixel_width; // Position x for the cell
                let position_y = grid_y as u32 * pixel_height; // Position y for the cell
                position.set_x(position_x);
                position.set_y(position_y);
                // println!("i: {}, pixel_width: {}, pixel_height: {}", index, pixel_width, pixel_height);
                // println!("Position: {}, {}", position_x, position_y);
                
                // Set the position of the cell entity
                let mut position = cell_entity.get::<Position>();
                position.set_x(position_x);
                position.set_y(position_y);
                let mut size = entity.get::<Size>();
                size.set_width(pixel_width);
                size.set_height(pixel_height);
                let mut render_target = entity.get::<RenderTarget>();
                render_target.set_render_target(Box::leak(rt) as *const _ as *const std::ffi::c_void as u64);
                
                // Add renderable component
                entity.add::<Renderable>();
                
                // Remove the blittable component
                cell_entity.remove::<Blittable>();
            }
        }
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
