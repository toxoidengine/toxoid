use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::*;

pub fn create_render_target(width: u32, height: u32) -> Entity {
    // Create entity
    let mut entity = Entity::new(None);
    entity.add::<RenderTarget>();
    entity.add::<Position>();
    entity.add::<Size>();
    entity.add::<BlendMode>();
    // Set render target object pointer
    let rt = SokolRenderer2D::create_render_target(width, height);
    let mut rt_component = entity.get::<RenderTarget>();
    rt_component
        .set_render_target(
            Box::leak(rt) as *const _ as *const std::ffi::c_void as u64
        );
    // Set size
    let mut size = entity.get::<Size>();
    size.set_width(width);
    size.set_height(height);
    entity
}

pub fn create_sprite(path: &str) -> Entity {
    // Create render target entity
    let game_config = World::get_singleton::<GameConfig>();
    let game_width = game_config.get_width();
    let game_height = game_config.get_height();
    let mut rt_entity = create_render_target(game_width, game_height);
    // Create renderable entity
    rt_entity.add::<Renderable>();
    // Create sprite entity
    let mut sprite_entity = load_sprite(path);
    sprite_entity.child_of(rt_entity);
    sprite_entity
}