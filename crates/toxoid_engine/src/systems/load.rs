use toxoid_api::*;
use toxoid_api_macro::*;

#[cfg(feature = "fetch")]
#[components(TiledCellComponent)]
pub fn load_cell_system(iter: &mut Iter) {
    let entities = iter.entities();
    components
        .enumerate()
        .for_each(|(i, cell)| {
            let mut cell_entity = &mut entities[i];
            let cell = cell
                .get_cell()
                .ptr as *mut toxoid_tiled::TiledCell;
            unsafe {
                (*cell)
                    .tilesets
                    .iter()
                    // Filter for duplicate tileset.name
                    .for_each(|tileset| {
                        let mut tileset_entity = Entity::new();
                        tileset_entity.set_name("sprite");
                        tileset_entity.add::<Sprite>();
                        tileset_entity.add::<Position>();
                        tileset_entity.add::<Size>();
                        tileset_entity.add::<Loadable>();
                        tileset_entity.get::<Sprite>()
                           .set_filename(StringPtr::new("assets/default_tileset.png"));

                        tileset_entity.add::<TilesetComponent>();
                        tileset_entity.child_of_by_id(cell_entity.get_id());
                    });
                // TODO: Avoid hashmap lookup
                cell_entity.remove::<Loadable>();
                cell_entity.add::<Blittable>();
            }
        });
}

#[cfg(feature = "fetch")]
#[components(TiledWorldComponent)]
pub fn load_world_system(iter: &mut Iter) {
    let entities = iter.entities();
    components
        .enumerate()
        .for_each(|(i, world)| {
            let mut world_entity = &mut entities[i];
            let world = world
                .get_world()
                .ptr as *mut toxoid_tiled::TiledWorld;
            unsafe {
                (*world)
                    .maps
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|(i, map)| {
                        let world_id = world_entity.get_id();
                        let cell_entity = crate::utils::load::load_cell(format!("assets/{}", map.file_name).as_str(), move |cell_entity: &mut Entity| {
                            cell_entity.child_of_by_id(world_id);
                        });
                        let mut cell_component = (*cell_entity).get::<TiledCellComponent>();
                        cell_component.set_index(i as u32);
                    });
            }
            // TODO: Avoid hashmap lookup
            world_entity.remove::<Loadable>();
        });
}

#[cfg(feature = "fetch")]
#[components(Sprite, _)]
pub fn load_sprite_system(iter: &mut Iter)  {
    let entities = iter.entities();
    components
        .enumerate()
        .for_each(|(i, sprite)| {
            let entity = &entities[i];
            crate::utils::load::load_sprite(sprite.get_filename(), entity);
            entities[i].remove::<Loadable>();
        });
}

pub fn init() {
    #[cfg(feature = "fetch")] {
        System::new(load_world_system)
            .with::<(TiledWorldComponent, Loadable)>()
            .build();
        // System::new(load_cell_system)
        //     .with::<(TiledCellComponent, Loadable)>()
        //     .build();
        System::new(load_sprite_system)
            .with::<(Sprite, Loadable)>()
            .build();
    }
}