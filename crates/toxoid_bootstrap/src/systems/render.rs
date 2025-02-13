use toxoid_api::*;
use toxoid_sokol::{bindings::*, SokolRenderTarget, SokolRenderer2D, SokolSprite, sapp};
use toxoid_render::Renderer2D;

// SpineInstance, Position, BoneAnimation
#[components(SpineInstance, Position, _)]
pub fn blit_bone_animation_system(iter: &Iter) {
    let mut entities = iter.entities();
    for (i, (spine_instance, position)) in components.into_iter().enumerate() {
        let entity = entities.get_mut(i).unwrap();
        if spine_instance.get_instantiated() {
            unsafe {
                // Get render target and its spine context
                let mut rt_entity = entity.parent();
                let rt = rt_entity.get::<RenderTarget>();
                let rt_ptr = rt.get_render_target();
                let rt_ptr_box = Box::from_raw(rt_ptr as *mut SokolRenderTarget);
                let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_ptr_box as Box<dyn toxoid_render::RenderTarget>));
                let instance = spine_instance.get_instance() as *mut sspine_instance;
                
                // Update spine instance but don't set position
                sspine_update_instance(*instance, sapp_frame_duration() as f32);
                
                // Draw spine instance
                sspine_draw_instance_in_layer(*instance, 0);

                let (window_width, window_height) = (sapp::width(), sapp::height());
                // Set up render target pass
                let layer_transform = sspine_layer_transform {
                    size: sspine_vec2 { 
                        x: window_width as f32, 
                        y: window_height as f32
                    },
                    origin: sspine_vec2 { 
                        x: 30., 
                        y: 70.
                    }
                };
                SokolRenderer2D::begin_rt(&rt_trait_object, window_width as f32, window_height as f32);
                sspine_draw_layer(0, &layer_transform);
                SokolRenderer2D::end_rt();

                rt.set_flip_y(true);
                rt.set_z_depth(ZDepth::SameAsPlayer as u32);
                rt_entity.add::<Renderable>();
            }
        }
    }
}

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
        render_target.set_z_depth(ZDepth::AbovePlayer as u32);
        entities[i].remove::<Blittable>();
    }
}

// Blit cell to render target
#[components(TiledCell, _, Size)]
pub fn blit_cell_system(iter: &Iter) {
    let mut entities = iter.entities();
    for (i, (_cell, _size)) in components.into_iter().enumerate() {
        let cell_entity = entities.get_mut(i).unwrap();
        let mut children = cell_entity.children();
        if children.len() > 0 {
            // TODO: Make this into Flecs Query with custom relationships
            let mut tileset_entities = children.iter_mut().filter(|child| child.has::<Tileset>() && child.has::<Blittable>()).collect::<Vec<_>>();
            let mut children = cell_entity.children();
            let render_target_entities = children.iter_mut().filter(|child| child.has::<RenderTarget>()).collect::<Vec<_>>();

            if tileset_entities.len() > 0 && render_target_entities.len() > 0 {
                // Get tileset 
                let tileset_entity = tileset_entities.get_mut(0).unwrap();
                let sprite = tileset_entity.get::<Sprite>();
                
                let sprite_ptr = sprite.get_sprite();
                let sprite_box = unsafe { Box::from_raw(sprite_ptr as *mut SokolSprite) };
                let tileset_sprite: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
                // let tileset_size = tileset_entity.get::<Size>();
                // let width = tileset_size.get_width();
                // let height = tileset_size.get_height();

                // let rt_entity = render_target_entities.get_mut(0).unwrap();
                // let render_target = rt_entity.get::<RenderTarget>();
                // let render_target_ptr = render_target.get_render_target();
                // let render_target_box = unsafe { Box::from_raw(render_target_ptr as *mut SokolRenderTarget) };
                // let render_target_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(render_target_box as Box<dyn toxoid_render::RenderTarget>));
                // let rt_width = 800.;
                // let rt_height = 600.;

                let cell = cell_entity.get::<TiledCell>();
                let cell = cell.get_cell() as *mut toxoid_tiled::TiledCell;
                let pixel_width = unsafe { (*cell).width * (*cell).tilewidth };
                let pixel_height = unsafe { (*cell).height * (*cell).tileheight };
                let tile_width = unsafe { (*cell).tilewidth };
                let tile_height = unsafe { (*cell).tileheight };
                let image_width = 4800;
                // let image_height = 720;
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
                                        let position = tile_entity.get::<Position>();
                                        position.set_x(dest_x as i32);
                                        position.set_y(dest_y as i32);
                                        tile_entity.add::<Size>();
                                        let size = tile_entity.get::<Size>();
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
                
                let position = entity.get::<Position>();
                // Assuming grid dimensions are known
                let grid_width = 2; // Number of cells horizontally
                let index = cell_entity.get::<TiledCell>().get_index();
                // Calculate grid positions based on the index
                let grid_x = index % grid_width; // Calculate x position in the grid
                let grid_y = index / grid_width; // Calculate y position in the grid
                let position_x = grid_x as i32 * pixel_width as i32; // Position x for the cell
                let position_y = grid_y as i32 * pixel_height as i32; // Position y for the cell
                position.set_x(position_x);
                position.set_y(position_y);
                // println!("i: {}, pixel_width: {}, pixel_height: {}", index, pixel_width, pixel_height);
                // println!("Position: {}, {}", position_x, position_y);
                
                // Set the position of the cell entity
                let position = cell_entity.get::<Position>();
                position.set_x(position_x);
                position.set_y(position_y);
                let size = entity.get::<Size>();
                size.set_width(pixel_width);
                size.set_height(pixel_height);
                let render_target = entity.get::<RenderTarget>();
                render_target.set_render_target(Box::leak(rt) as *const _ as *const std::ffi::c_void as u64);
                render_target.set_z_depth(ZDepth::BottomLayer as u32);
                
                // Add renderable component
                entity.add::<Renderable>();
                
                // Remove the blittable component
                cell_entity.remove::<Blittable>();
            }
        }
    }
}

// Rect Renderer
// #[components(_, Position, Size, Color, _)]
// pub fn blit_rect_system(iter: &Iter) {
//     for (pos, size, color) in components {
//         SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
//     }
// }

// Sort Render Targets by Z-Index
// TODO: Use query trampoline instead of C functions directly and use callback resource to make this work on WASM.
pub extern "C" fn draw_render_target_sort(_e1: ecs_entity_t, v1: *const std::ffi::c_void, _e2: ecs_entity_t, v2: *const std::ffi::c_void) -> i32 {
    let mut rt1 = RenderTarget::default();
    let mut rt2 = RenderTarget::default();
    let rt1_component = ToxoidComponent::from_ptr_host(v1 as u64);
    let rt2_component = ToxoidComponent::from_ptr_host(v2 as u64);
    rt1.set_component(rt1_component);
    rt2.set_component(rt2_component);
    let z1 = rt1.get_z_depth();
    let z2 = rt2.get_z_depth();
    z1.cmp(&z2) as i32
}

// Draw Render Targets to screen as final output
#[components(RenderTarget, _, Size, Position, BlendMode)]
pub fn draw_render_targets_system(iter: &Iter) {
    // Get camera position
    let main_camera = World::get_singleton::<MainCamera>();
    let mut camera_entity = Entity::from_id(main_camera.get_entity());
    let camera_pos = camera_entity.get::<Position>();
    let camera = camera_entity.get::<Camera>();
    let camera_x = camera_pos.get_x() as f32;
    let camera_y = camera_pos.get_y() as f32;
    let zoom = camera.get_zoom();

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

        // Apply camera transform to world position
        let world_x = position.get_x() as f32 - camera_x;
        let world_y = position.get_y() as f32 - camera_y;
        
        // Scale positions and sizes by zoom
        let scaled_x = world_x * zoom;
        let scaled_y = world_y * zoom;
        let scaled_width = width as f32 * zoom;
        let scaled_height = height as f32 * zoom;

        // Flip Y for Spine
        // TODO: Figure out some other way to do this
        #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
        let source_height = if rt.get_flip_y() { -(height as f32) } else { height as f32 };
        #[cfg(not(all(target_arch="wasm32", target_os="emscripten")))]
        let source_height = height as f32;
        // Draw render target with camera transform applied
        SokolRenderer2D::draw_render_target(
            rt_trait_object,
            0., 0.,
            width as f32, source_height,
            scaled_x, scaled_y,
            scaled_width, scaled_height,
            blend_mode
        );
    }
}

// Systems that draw render targets to the screen as a final output
pub fn draw_systems(render_systems_entity: &mut Entity) {
    // Draw Render Targets
    let mut system = System::dsl("RenderTarget, Renderable, Size, Position, BlendMode", None, draw_render_targets_system);
    system.order_by(RenderTarget::get_id(), draw_render_target_sort);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());
}

// Systems that blit render targets
pub fn blit_systems(render_systems_entity: &mut Entity) {
    // Blit cell to render target
    let mut system = System::dsl("TiledCell, Blittable, Size", None, blit_cell_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Bone Animation Renderer
    let mut system = System::dsl("SpineInstance, Position, Blittable", None, blit_bone_animation_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Blit sprite to render target
    let mut system = System::dsl("Sprite, Blittable, Size, (ChildOf, $Parent), RenderTarget($Parent), Size($Parent)", None, blit_sprite_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // // Rect Renderer
    // let mut system = System::dsl("Rect, Position, Size, Color, Blittable", None, blit_rect_system);
    // system.build();
    // render_systems_entity.parent_of_id(system.get_id());
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
    let render_systems_singleton = World::get_singleton::<RenderSystems>();
    render_systems_singleton.set_entity(render_systems_entity.get_id());
}
