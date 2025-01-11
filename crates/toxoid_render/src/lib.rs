use std::any::Any;
use std::ffi::c_void;
use toxoid_api::components::{Position, Size, Color};

pub trait Sprite: Any {
    // Define methods that all sprites should have
    fn as_any(&self) -> &dyn Any;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_width(&mut self, width: u32);
    fn set_height(&mut self, height: u32);
}

pub trait RenderTarget: Any {
    // Define methods that all sprites should have
    fn as_any(&self) -> &dyn Any;
}

pub trait Renderer2D {
    // Constructor
    fn new() -> Self;
    // Begin render pass
    fn begin();
    // End render pass
    fn end();
    // Begin render target pass
    fn begin_rt(destination: &Box<dyn RenderTarget>, dw: f32, dh: f32);
    // End render target pass
    fn end_rt();
    // Get window size
    fn window_size() -> (u32, u32);
    // Create render target that we can blit sprites on (Tilemaps for example)
    fn create_render_target(width: u32, height: u32) -> Box<dyn RenderTarget>;
    // Create sprite
    fn create_sprite(data: *const u8, size: usize) -> Box<dyn Sprite>;
    // Blit sprite (draw sprite on another base sprite)
    fn blit_sprite(source: &Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, destination: &Box<dyn RenderTarget>, dx: f32, dy: f32);
    // Resize sprite
    fn resize_sprite(sprite: &Box<dyn Sprite>, width: u32, height: u32);
    // Draw sprite
    fn draw_sprite(sprite: &Box<dyn Sprite>, x: f32, y: f32);
    // Draw render target
    fn draw_render_target(source: &Box<dyn RenderTarget>, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32, dw: f32, dh: f32, blend_mode: u8);
    // Draw a filled rect
    fn draw_filled_rect(pos: &Position, size: &Size, color: &Color);
    // Draw a line
    fn draw_line(ax: f32, ay: f32, bx: f32, by: f32);
    // Clear sprite
    fn clear_sprite(render_target: &Box<dyn RenderTarget>, x: i32, y: i32, width: i32, height: i32);
    // Clear entire canvas
    fn clear_canvas(x: i32, y: i32, width: i32, height: i32);
}

pub trait Renderer3D {}
