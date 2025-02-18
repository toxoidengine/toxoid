use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::*;

pub fn create_render_target(width: u32, height: u32, z_depth: u32) -> Entity {
    // Create entity
    let mut entity = Entity::new(None);
    entity.add::<RenderTarget>();
    entity.add::<Position>();
    entity.add::<Size>();
    entity.add::<BlendMode>();
    // Set render target object pointer
    let rt = SokolRenderer2D::create_render_target(width, height);
    let rt_component = entity.get::<RenderTarget>();
    rt_component
        .set_render_target(
            Box::leak(rt) as *const _ as *const std::ffi::c_void as u64
        );
    rt_component.set_z_depth(z_depth);
    // Set size
    let size = entity.get::<Size>();
    size.set_width(width);
    size.set_height(height);
    entity
}

pub fn create_sprite_from_data(data: Vec<u8>) -> Entity {
    // Create entity from entity ID passed to user data
    let mut sprite_entity = Entity::new(None);
    sprite_entity.add::<Sprite>();
    sprite_entity.add::<Position>();
    sprite_entity.add::<Size>();
    let size = data.len();
    let data_box = data.clone().into_boxed_slice();
    let data_ptr = Box::into_raw(data_box);
    // Create sokol sprite
    let sokol_sprite = SokolRenderer2D::create_sprite(data_ptr as *const u8, size);
    let sprite_width = sokol_sprite.width();
    let sprite_height = sokol_sprite.height();
    // Set size
    let size = sprite_entity.get::<Size>();
    size.set_width(sprite_width);
    size.set_height(sprite_height);
    // Set sprite
    let sprite = sprite_entity.get::<Sprite>();
    sprite.set_sprite(Box::into_raw(sokol_sprite) as *mut () as u64);
    sprite_entity.add::<Blittable>();
    // Create render target entity
    let mut rt_entity = create_render_target(sprite_width, sprite_height, ZDepth::AbovePlayer as u32);
    sprite_entity.add_relationship_id(Relationship::Custom(RenderTargetRelationship::get_id()), rt_entity.get_id());
    // // Create renderable entity
    rt_entity.add::<Renderable>();
    sprite_entity.add::<Loaded>();
    sprite_entity
}