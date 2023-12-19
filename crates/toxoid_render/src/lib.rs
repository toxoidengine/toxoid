use std::any::Any;

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait Sprite: Any {
    // Define methods that all sprites should have
    fn as_any(&self) -> &dyn Any;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_width(&mut self, width: u32);
    fn set_height(&mut self, height: u32);
}

pub trait Renderer2D {
    // Constructor
    fn new() -> Self;
    // Create sprite
    fn create_sprite(filename: &str) -> Box<dyn Sprite>;
    // Blit sprite (draw sprite on another base sprite)
    fn blit_sprite(sprite: &Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32);
    // Resize sprite
    fn resize_sprite(sprite: &Box<dyn Sprite>, width: u32, height: u32);
    // Render sprite
    fn draw_sprite(sprite: &Box<dyn Sprite>, x: f32, y: f32, scale_factor: f32);
    // Draw a rect which is just the outline
    fn draw_rect(rect: Rect, color: Color);
    // Draw a filled rect
    fn draw_filled_rect(rect: Rect, color: Color);
    // Draw a line
    fn draw_line(ax: f32, ay: f32, bx: f32, by: f32);
    // Clear sprite
    fn clear_sprite(sprite: &Box<dyn Sprite>, x: i32, y: i32, width: i32, height: i32);
    // Clear entire canvas
    fn clear_canvas(x: i32, y: i32, width: i32, height: i32);
}

pub trait Renderer3D {}
