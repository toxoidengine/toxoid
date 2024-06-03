use toxoid_api::*;
use toxoid_api_macro::*;
#[cfg(feature = "render")]
use toxoid_sokol::render_2d::*;
#[cfg(feature = "render")]
use toxoid_render::Renderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::bindings::*;

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
        SokolRenderer2D::draw_filled_rect(pos, size, color);
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
        SokolRenderer2D::draw_sprite(sprite_trait_object, pos.get_x() as f32, pos.get_y() as f32);
    }
}

#[cfg(feature = "render")]
#[components(RenderTarget, _, Size, Position)]
// RenderTarget, Renderable, Size, Position
pub fn render_rt_system(iter: &mut Iter) {
    components
        .for_each(|(rt, size, pos)| {
            let rt_ptr = rt.get_render_target();
            let rt_box = unsafe { Box::from_raw(rt_ptr.ptr as *mut SokolRenderTarget) };
            let rt_trait_object: &Box<dyn toxoid_render::RenderTarget> = Box::leak(Box::new(rt_box as Box<dyn toxoid_render::RenderTarget>));
            // Draw Render Target
            // #[cfg(feature = "sokol")]
            SokolRenderer2D::draw_render_target(rt_trait_object, pos.get_x() as f32, pos.get_y() as f32, size.get_width() as f32, size.get_height() as f32);
        });
}

// SpineInstance, Position, BoneAnimation
#[cfg(all(feature = "render", feature = "spine"))]
pub fn render_bone_animation(iter: &mut Iter) {
    let spine_instances = iter.field::<SpineInstance>(1);
    let positions = iter.field::<Position>(2);
    for i in 0..iter.count() {
        use bindings::*;
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

// #[cfg(feature = "render")]
// pub fn render_tilemap_system() {
//     println!("Loading map: {}", map.file_name);
//     if map.file_name != "cell_3.json" {
//         return;
//     }

//     let cell_ptr = cell_entity.get::<TiledCellComponent>().get_cell().ptr as *mut toxoid_tiled::TiledCell;
//     let cell: Box<toxoid_tiled::TiledCell> = unsafe { Box::from_raw(cell_ptr) };
//     let tileset_entity = Entity::from_id(tileset_entity_id.try_into().unwrap());
//     let tileset_ptr = tileset_entity.get::<Sprite>().get_sprite().ptr as *mut SokolSprite;
//     let tileset_sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(tileset_ptr) };
//     let pixel_width = cell.width * cell.tilewidth;
//     let pixel_height = cell.height * cell.tileheight;
//     let rt = SokolRenderer2D::create_render_target(pixel_width, pixel_height);
//     SokolRenderer2D::begin_rt(&rt, pixel_width as f32, pixel_height as f32);
//     cell
//         .layers
//         .iter()
//         .for_each(|layer| {
//             if layer.layer_type == "group" {
//                 layer
//                     .layers
//                     .as_ref()
//                     .unwrap()
//                     .iter()
//                     .for_each(|layer| {
//                         if layer.layer_type == "tilelayer" {
//                             // Iterate over the tiles in the map
//                             for y in 0..cell.height {
//                                 for x in 0..cell.width {
//                                     // Calculate the position to blit each tile on the render target
//                                     let dest_x = x as f32 * tile_width as f32;
//                                     let dest_y = y as f32 * tile_height as f32;
//                                     let i = x as usize + (y as usize * cell.width as usize);
//                                     let tile_id = layer.data.as_ref().unwrap()[i];
//                                     // Tiled 1-indexes the tile ids
//                                     // and 0 is a special value for an empty tile
//                                     if tile_id == 0 {
//                                         continue;
//                                     }
//                                     // Calculate x and y position of the tile in the tileset
//                                     // Based on the tile id which is the index of the tile in the tileset
//                                     // Assuming the tileset is a single row of tiles
//                                     // You may need to adjust this based on the tileset layout
//                                     // Calculate the source x and y position of the tile in the tileset
//                                     let tileset_width = image_width / tile_width;
//                                     let tileset_x = (tile_id - 1) % tileset_width;
//                                     let tileset_y = (tile_id - 1) / tileset_width;
//                                     let src_x = tileset_x as f32 * tile_width as f32;
//                                     let src_y = tileset_y as f32 * tile_height as f32;

//                                     // Blit tile from the tileset to the map's render target
//                                     // Assuming you have a way to determine the source tile's position in the tileset, adjust src_x and src_y accordingly
//                                     SokolRenderer2D::blit_sprite(&tileset_sprite, src_x, src_y, tile_width as f32, tile_height as f32, &rt, dest_x, dest_y);
                                    
//                                     // Create an entity for each tile
//                                     let mut tile_entity = Entity::new();
//                                     tile_entity.add::<Position>();
//                                     let mut position = tile_entity.get::<Position>();
//                                     position.set_x(dest_x as u32);
//                                     position.set_y(dest_y as u32);
//                                     tile_entity.add::<Size>();
//                                     let mut size = tile_entity.get::<Size>();
//                                     size.set_width(tile_width);
//                                     size.set_height(tile_height);
//                                     // Add other components as needed, e.g., for collision checks
//                                 }
//                             }
//                         }
//                     });
//                 }
//         });
//         SokolRenderer2D::end_rt();
//         // Create render target entity
//         {
//             let mut entity = Entity::new();
//             entity.add::<Position>();
//             entity.add::<Size>();
//             entity.add::<RenderTarget>();

//             let mut position = entity.get::<Position>();
//             position.set_x(0);
//             position.set_y(0);
//             let mut size = entity.get::<Size>();
//             size.set_width(pixel_width);
//             size.set_height(pixel_height);
//             let mut render_target = entity.get::<RenderTarget>();
//             render_target.set_render_target(Pointer{ ptr: Box::leak(rt) as *mut _ as *mut c_void });

//             entity.add::<Renderable>();
//         }
// }

#[cfg(feature = "render")]
#[components(TiledCellComponent)]
pub fn blit_cell_system(iter: &mut Iter) {
    let entities = iter.entities();
    entities.sort_by(|a, b| a.get::<TiledCellComponent>().get_index().cmp(&b.get::<TiledCellComponent>().get_index()));
    entities
        .iter_mut()
        .enumerate()
        .for_each(|(i, cell_entity)| unsafe {
            let cell = cell_entity
                .get::<TiledCellComponent>()
                .get_cell()
                .ptr as *mut toxoid_tiled::TiledCell;
            let children = cell_entity.children();
            // TODO: Optimize this with Filter (must have childof as an option for
            // filter / query builder.
            let tileset_children = children
                .iter()
                .filter(|child| child.has::<Sprite>() && child.has::<TilesetComponent>() && child.has::<Blittable>())
                .collect::<Vec<&Entity>>();
            if tileset_children.len() == 0 {
                return;
            }

            // TODO, check which tileset is being used by the current cell / layers
            let tileset_ptr = tileset_children[0].get::<Sprite>().get_sprite().ptr as *mut SokolSprite;
            let tileset_sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(tileset_ptr) };
            let pixel_width = (*cell).width * (*cell).tilewidth;
            let pixel_height = (*cell).height * (*cell).tileheight;
            let tile_width = (*cell).tilewidth;
            let tile_height = (*cell).tileheight;
            let image_width = 4800;
            let image_height = 720;
            let rt = SokolRenderer2D::create_render_target(pixel_width, pixel_height);
            SokolRenderer2D::begin_rt(&rt, pixel_width as f32, pixel_height as f32);
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
                                    // Iterate over the tiles in the map
                                    for y in 0..(*cell).height {
                                        for x in 0..(*cell).width {
                                            // Calculate the position to blit each tile on the render target
                                            let dest_x = x as f32 * tile_width as f32;
                                            let dest_y = y as f32 * tile_height as f32;
                                            let i = x as usize + (y as usize * (*cell).width as usize);
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
                                            let mut tile_entity = Entity::new();
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
                                }
                            });
                        }
                });
            SokolRenderer2D::end_rt();
            // Create render target entity
            let mut entity = Entity::new();
            entity.add::<Position>();
            entity.add::<Size>();
            entity.add::<RenderTarget>();

            let mut position = entity.get::<Position>();
            // Assuming grid dimensions are known
            let grid_width = 2; // Number of cells horizontally
            let index = cell_entity.get::<TiledCellComponent>().get_index();
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
            render_target.set_render_target(Pointer{ ptr: Box::leak(rt) as *mut _ as *mut c_void });

            entity.add::<Renderable>();

            // Remove the blittable component
            cell_entity.remove::<Blittable>();
        });
}


pub fn init() {
    #[cfg(feature = "render")] {
        // Renderers
        System::new(render_rt_system)
            .with::<(RenderTarget, Renderable, Size, Position)>()
            .build();
        System::new(render_sprite_system)
            .with::<(Sprite, Renderable, Size, Position)>()
            .build();
        System::new(render_bone_animation)
            .with::<(SpineInstance, Position, BoneAnimation)>()
            .build();
        System::new(render_rect_system)
            .with::<(Rect, Renderable, Color, Size, Position)>()
            .build();
        
        // Blitting
        System::new(blit_cell_system)
            .with::<(TiledCellComponent, Blittable)>()
            .build();
    }
    
    // #[cfg(feature = "render")]
    // System::new(render_cell_system)
    //     .with::<(TiledCellComponent, Renderable)>()
    //     .build();
}