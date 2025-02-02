use toxoid_api::*;
use toxoid_sokol::{SokolRenderer2D, SokolSprite};
use toxoid_render::Renderer2D;

// Rect Renderer
#[components(_, Position, Size, Color, _)]
pub fn rect_render_system(iter: &Iter) {
    for (pos, size, color) in components {
        SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
    }
}

// Sprite Renderer
#[components(Sprite, Size)]
pub fn sprite_render_system(iter: &Iter) {
    for (sprite, _size) in components {
        let sprite_box = unsafe { Box::from_raw(sprite.get_sprite() as *mut SokolSprite) };
        let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
        SokolRenderer2D::draw_sprite(sprite_trait_object, 0., 0.);
    }
}

// Rendering Systems
pub fn init() {
    // Render systems group
    let mut render_systems_entity = Entity::named("Render Systems Group");
    
    // Rect Renderer
    let mut system = System::dsl("Rect, Position, Size, Color, Renderable", None, rect_render_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Sprite Renderer
    let mut system = System::dsl("Sprite, Size", None, sprite_render_system);
    system.build();
    render_systems_entity.parent_of_id(system.get_id());

    // Disable render systems
    render_systems_entity.disable();

    // Set render systems group
    let mut render_systems_singleton = World::get_singleton::<RenderSystems>();
    render_systems_singleton.set_entity(render_systems_entity.get_id());
}
