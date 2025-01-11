use toxoid_sokol::SokolRenderer2D;
use toxoid_render::Renderer2D;
use toxoid_api::*;
use crate::entities;

pub fn init() {    
    unsafe {
        toxoid_host::QUERY_TRAMPOLINE = Some(toxoid_runtime::query_trampoline);
    }
    
    // Rendering System
    System::dsl("Rect, Position, Size, Color, Renderable", None, |iter| {
        iter.entities().iter_mut().for_each(|entity| {
            let pos = entity.get::<Position>();
            let size = entity.get::<Size>();
            let color = entity.get::<Color>();
            SokolRenderer2D::draw_filled_rect(&pos, &size, &color);
        });
    })
        .build();
}