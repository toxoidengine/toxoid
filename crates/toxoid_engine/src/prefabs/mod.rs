
use toxoid_api::*;
use toxoid_ffi::flecs_core::flecs_serialize_entity;
#[cfg(feature = "render")]
use toxoid_sokol::SokolSprite;

pub fn init() {
    // let mut entity = Entity::new();
    // entity.add::<Position>();
    // entity.add::<Size>();
    // entity.add::<Sprite>();
    // let components = unsafe { flecs_serialize_entity(entity.get_id()) };
    // println!("Components {:?}", components);
    
    // Game Config
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    // Keyboard Input
    World::add_singleton::<KeyboardInput>();

    // Local Player
    World::add_singleton::<Networked>();
    
    // Parent entity
    let mut player_entity = Entity::new();
    player_entity.add::<Local>();
    player_entity.add::<Player>();
    player_entity.add::<Networked>();
    // let mut local_player = World::get_singleton::<Networked>();
    // local_player.set_entity_id(player_entity.get_id());
    // TODO: Make animation a child entity of player later
    // #[cfg(feature = "render")]
    // let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    // let mut position = player_animation_entity.get::<Position>();
    // #[cfg(feature = "render")]
    // player_animation_entity.add::<Local>();

    #[cfg(feature = "render")]
    use toxoid_render::Renderer2D;
    #[cfg(feature = "render")]
    use toxoid_sokol::SokolRenderer2D;
    
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

    #[cfg(feature = "render")]
    crate::utils::load::load_sprite("assets/character.png", |entity: &mut Entity| {
        let mut position = entity.get::<Position>();
        position.set_x(0);
        position.set_y(0);
        entity.add::<Renderable>();
        entity.add::<Player>();
        entity.add::<Direction>();

         let mut local_player = World::get_singleton::<Networked>();
         local_player.set_entity_id(entity.get_id());

         crate::utils::network::init();
    });

    #[cfg(feature = "client")]
    crate::utils::load::load_worldmap("assets/world_1.world", |world_entity: &mut Entity| {
        let world_entity: usize = world_entity.get_id() as usize;
        crate::utils::load::load_sprite("assets/default_tileset.png", move |tileset_entity: &mut Entity| {
            // let image_width = 4800;
            // let image_height = 720;
            // let tile_width = 16;
            // let tile_height = 16;

            // let mut position = tileset_entity.get::<Position>();
            // position.set_x(100);
            // position.set_y(100);
            // let rt = SokolRenderer2D::create_render_target(500, 500);
            // let tileset_ptr = tileset_entity.get::<Sprite>().get_sprite().ptr as *mut SokolSprite;
            // let tileset_sprite: Box<dyn toxoid_render::Sprite> = unsafe { Box::from_raw(tileset_ptr) };

            // // Blit two sprites on render target at different locations
            // SokolRenderer2D::begin_rt(&rt, 500., 500.);
            // SokolRenderer2D::blit_sprite(&tileset_sprite, 0., 0., 100., 100., &rt, 0., 0.);
            
            // // Create render target entity
            // {
            //     let mut entity = Entity::new();
            //     entity.add::<Position>();
            //     entity.add::<Size>();
            //     entity.add::<RenderTarget>();

            //     let mut position = entity.get::<Position>();
            //     position.set_x(100);
            //     position.set_y(100);
            //     let mut size = entity.get::<Size>();
            //     size.set_width(500);
            //     size.set_height(500);
            //     let mut render_target = entity.get::<RenderTarget>();
            //     render_target.set_render_target(Pointer{ ptr: Box::into_raw(rt) as *mut c_void });

            //     entity.add::<Renderable>();
            // }

            // Get world entity
            let world_entity = Entity::from_id(world_entity.try_into().unwrap());
            let world_ptr = world_entity.get::<TiledWorldComponent>().get_world().ptr as *mut toxoid_tiled::TiledWorld;
            let world: Box<toxoid_tiled::TiledWorld> = unsafe { Box::from_raw(world_ptr) };
            // Iterate over world data to load cells
            world
                .maps
                .unwrap()
                .iter()
                .for_each(|map| {
                    // Load cell
                    crate::utils::load::load_cell(format!("assets/{}", map.file_name).as_str(), move |cell_entity: &mut Entity| {
                        let cell_ptr = cell_entity.get::<TiledCellComponent>().get_cell().ptr as *mut toxoid_tiled::TiledCell;
                        let cell: Box<toxoid_tiled::TiledCell> = unsafe { Box::from_raw(cell_ptr) };
                        cell
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
                                            println!("Layer type: {:?}", layer.layer_type);
                                            if layer.layer_type == "tilelayer" {
                                                layer
                                                .data
                                                .as_ref()
                                                .unwrap()
                                                .iter()
                                                .for_each(|tile| {
                                                    println!("Tile {:?}", tile);
                                                });
                                            }
                                        });
                                    }
                            //   layer.tiles.iter().for_each(|tile| {
                            //       let mut tile_entity = Entity::new();
                            //       tile_entity.add::<Position>();
                            //       tile_entity.add::<Size>();
                            //       tile_entity.add::<Sprite>();
                            //       let mut position = tile_entity.get::<Position>();
                            //       position.set_x(tile.x);
                            //       position.set_y(tile.y);
                            //       let mut size = tile_entity.get::<Size>();
                            //       size.set_width(tile.width);
                            //       size.set_height(tile.height);
                            //       let mut sprite = tile_entity.get::<Sprite>();
                            //       sprite.set_filename("assets/default_tileset.png");
                            //       sprite.set_sprite(tile.sprite);
                            //       let mut renderable = tile_entity.get::<Renderable>();
                            //       renderable.set_renderable(true);
                            //   });
                            });
                    });
                    // let map_entity = Entity::new();
                    // map_entity.add::<Map>();
                    // map_entity.add::<Renderable>();
                    // map_entity.add::<TiledMap>();
                    // let mut tiled_map = map_entity.get::<TiledMap>();
                    // tiled_map.set_map(map);
                    // let mut position = map_entity.get::<Position>();
                    // position.set_x(map.x);
                    // position.set_y(map.y);
                    // let mut size = map_entity.get::<Size>();
                    // size.set_width(map.width);
                    // size.set_height(map.height);
                    // let mut renderable = map_entity.get::<Renderable>();
                    // renderable.set_renderable(true);
                });
        });
    });
}

pub extern "C" fn load_cell() {

}
