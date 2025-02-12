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
    let rt_component = entity.get::<RenderTarget>();
    rt_component
        .set_render_target(
            Box::leak(rt) as *const _ as *const std::ffi::c_void as u64
        );
    // Set size
    let size = entity.get::<Size>();
    size.set_width(width);
    size.set_height(height);
    entity
}