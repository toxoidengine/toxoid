use toxoid_api::*;

use toxoid_api::*;
#[cfg(feature = "render")]
use toxoid_render::Renderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolRenderer2D;
#[cfg(feature = "render")]
use toxoid_sokol::SokolSprite;

pub fn init() {
    // let mut entity = Entity::new();
    // entity.add::<Position>();
    // entity.add::<Size>();
    // entity.add::<Sprite>();
    // let components = unsafe { flecs_serialize_entity(entity.get_id()) };
    // println!("Components {:?}", components);

    // Initialize tilemap
    // tilemap::init();

    // #[cfg(feature = "render")]
    // use toxoid_render::Renderer2D;
    // #[cfg(feature = "render")]
    // use toxoid_sokol::SokolRenderer2D;
    
    // crate::utils::load::load_sprite("assets/character.png", |entity: &mut Entity| {
    //     entity.add::<Player>();
    //     let mut position = entity.get::<Position>();
    //     position.set_x(100);
    //     position.set_y(100);
        
    //     let rt = SokolRenderer2D::create_render_target(500, 500);
    //     let sprite_component = entity.get::<Sprite>();
    //     let sprite_ptr = sprite_component.get_sprite().ptr as *mut SokolSprite;
    //     let sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(sprite_ptr) };

    //     // Blit two sprites on render target at different locations
    //     SokolRenderer2D::begin_rt(&rt, 500., 500.);
    //     SokolRenderer2D::blit_sprite(&sprite, 0., 0., 100., 100., &rt, 0., 0.);
    //     SokolRenderer2D::blit_sprite(&sprite, 0., 0., 100., 100., &rt, 50., 50.);
    //     SokolRenderer2D::end_rt();

    //     // Create render target entity
    //     {
    //         let mut entity = Entity::new();
    //         entity.add::<Position>();
    //         entity.add::<Size>();
    //         entity.add::<RenderTarget>();

    //         let mut position = entity.get::<Position>();
    //         position.set_x(100);
    //         position.set_y(100);
    //         let mut size = entity.get::<Size>();
    //         size.set_width(500);
    //         size.set_height(500);
    //         let mut render_target = entity.get::<RenderTarget>();
    //         render_target.set_render_target(Pointer{ ptr: Box::into_raw(rt) as *mut c_void });

    //         entity.add::<Renderable>();
    //     }
        
    //     // Flag sprite as renderable for draw_sprite_system
    //     // entity.add::<Renderable>();

    //     // std::mem::forget(rt);
    //     std::mem::forget(sprite);

    //     crate::utils::load::load_sprite("assets/map.png", |map_entity: &mut Entity| {
    //         map_entity.add::<Map>();
    //         map_entity.add::<Renderable>();
    //         let mut filter = Filter::new();
    //         filter
    //             .with::<(Sprite,Player)>()
    //             .build();
    //         let mut iter = Iter::from(filter.iter());
    //         // println!("Next {:?}", iter.filter_next());
    //         if (iter.filter_next()) {
    //             let entities = iter.entities();
    //             println!("Entities {:?}", entities.len())
    //         }
    //     });
    // });
    
   // Parent entity
   let mut player_entity = Entity::new();
   player_entity.set_name("player");
   player_entity.add::<Local>();
   player_entity.add::<Player>();
   player_entity.add::<Networked>();
   
   let mut child_entity = Entity::new();
   child_entity.add::<Position>();
   child_entity.get::<Position>().set_x(100);
   child_entity.get::<Position>().set_y(777);
   child_entity.child_of_by_id(player_entity.get_id());

   let _json = unsafe { toxoid_ffi::ecs::toxoid_entity_to_json(player_entity.get_id()) }; 
   
   // let mut local_player = World::get_singleton::<Networked>();
   // local_player.set_entity_id(player_entity.get_id());
   // TODO: Make animation a child entity of player later
   #[cfg(feature = "render")] {
      let player_animation_entity = crate::utils::load::load_animation("assets/player.atlas", "assets/player.json", |entity| {});
      unsafe {
         let mut position = (*player_animation_entity).get::<Position>();
         position.set_x(100);
         position.set_y(100);
         (*player_animation_entity).add::<Local>();
      }
   }

   #[cfg(feature = "render")]
   crate::utils::load::load_worldmap("assets/world_1.world", |world_entity: &mut Entity| {});
}

      //  let world_entity_id: usize = world_entity.get_id() as usize;
      //  crate::utils::load::load_sprite("assets/default_tileset.png", move |tileset_entity: &mut Entity| {
      //      let image_width = 4800;
      //      let image_height = 720;
      //      let tile_width = 16;
      //      let tile_height = 16;
           
      //      // Get world entity
      //      let world_entity = Entity::from_id(world_entity_id.try_into().unwrap());
      //      let world_ptr = world_entity.get::<TiledWorldComponent>().get_world().ptr as *mut toxoid_tiled::TiledWorld;
      //      let world: Box<toxoid_tiled::TiledWorld> = unsafe { Box::from_raw(world_ptr) };
      //      let tileset_entity_id: usize = tileset_entity.get_id() as usize;
      //      // Iterate over world data to load cells
      //      world
      //          .maps
      //          .unwrap()
      //          .iter()
      //          .for_each(|map| {
      //              // println!("Loading map: {}", map.file_name);
      //              // if map.file_name != "cell_3.json" {
      //              //     return;
      //              // }
      //              // Load cell
      //              crate::utils::load::load_cell(format!("assets/{}", map.file_name).as_str(), move |cell_entity: &mut Entity| {
      //                  let cell_ptr = cell_entity.get::<TiledCellComponent>().get_cell().ptr as *mut toxoid_tiled::TiledCell;
      //                  let cell: Box<toxoid_tiled::TiledCell> = unsafe { Box::from_raw(cell_ptr) };
      //                  let tileset_entity = Entity::from_id(tileset_entity_id.try_into().unwrap());
      //                  let tileset_ptr = tileset_entity.get::<Sprite>().get_sprite().ptr as *mut SokolSprite;
      //                  let tileset_sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(tileset_ptr) };
      //                  let pixel_width = cell.width * cell.tilewidth;
      //                  let pixel_height = cell.height * cell.tileheight;
      //                  let rt = SokolRenderer2D::create_render_target(pixel_width, pixel_height);
      //                  SokolRenderer2D::begin_rt(&rt, pixel_width as f32, pixel_height as f32);
      //                  cell
      //                      .layers
      //                      .iter()
      //                      .for_each(|layer| {
      //                          if layer.layer_type == "group" {
      //                              layer
      //                                  .layers
      //                                  .as_ref()
      //                                  .unwrap()
      //                                  .iter()
      //                                  .for_each(|layer| {
      //                                      if layer.layer_type == "tilelayer" {
      //                                          // Iterate over the tiles in the map
      //                                          for y in 0..cell.height {
      //                                              for x in 0..cell.width {
      //                                                  // Calculate the position to blit each tile on the render target
      //                                                  let dest_x = x as f32 * tile_width as f32;
      //                                                  let dest_y = y as f32 * tile_height as f32;
      //                                                  let i = x as usize + (y as usize * cell.width as usize);
      //                                                  let tile_id = layer.data.as_ref().unwrap()[i];
      //                                                  // Tiled 1-indexes the tile ids
      //                                                  // and 0 is a special value for an empty tile
      //                                                  if tile_id == 0 {
      //                                                      continue;
      //                                                  }
      //                                                  // Calculate x and y position of the tile in the tileset
      //                                                  // Based on the tile id which is the index of the tile in the tileset
      //                                                  // Assuming the tileset is a single row of tiles
      //                                                  // You may need to adjust this based on the tileset layout
      //                                                  // Calculate the source x and y position of the tile in the tileset
      //                                                  let tileset_width = image_width / tile_width;
      //                                                  let tileset_x = (tile_id - 1) % tileset_width;
      //                                                  let tileset_y = (tile_id - 1) / tileset_width;
      //                                                  let src_x = tileset_x as f32 * tile_width as f32;
      //                                                  let src_y = tileset_y as f32 * tile_height as f32;

      //                                                  // Blit tile from the tileset to the map's render target
      //                                                  // Assuming you have a way to determine the source tile's position in the tileset, adjust src_x and src_y accordingly
      //                                                  SokolRenderer2D::blit_sprite(&tileset_sprite, src_x, src_y, tile_width as f32, tile_height as f32, &rt, dest_x, dest_y);
                                                       
      //                                                  // Create an entity for each tile
      //                                                  let mut tile_entity = Entity::new();
      //                                                  tile_entity.add::<Position>();
      //                                                  let mut position = tile_entity.get::<Position>();
      //                                                  position.set_x(dest_x as u32);
      //                                                  position.set_y(dest_y as u32);
      //                                                  tile_entity.add::<Size>();
      //                                                  let mut size = tile_entity.get::<Size>();
      //                                                  size.set_width(tile_width);
      //                                                  size.set_height(tile_height);
      //                                                  // Add other components as needed, e.g., for collision checks
      //                                              }
      //                                          }
      //                                      }
      //                                  });
      //                              }
      //                      });
      //                      SokolRenderer2D::end_rt();

      //                      // Create render target entity
      //                      {
      //                          let mut entity = Entity::new();
      //                          entity.add::<Position>();
      //                          entity.add::<Size>();
      //                          entity.add::<RenderTarget>();

      //                          let mut position = entity.get::<Position>();
      //                          position.set_x(0);
      //                          position.set_y(0);
      //                          let mut size = entity.get::<Size>();
      //                          size.set_width(pixel_width);
      //                          size.set_height(pixel_height);
      //                          let mut render_target = entity.get::<RenderTarget>();
      //                          render_target.set_render_target(Pointer{ ptr: Box::leak(rt) as *mut _ as *mut c_void });

      //                          entity.add::<Renderable>();
      //                      }
      //              });
      //          });
      //  });
