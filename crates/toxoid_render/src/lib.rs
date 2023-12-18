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

pub trait Image {
    // Define methods that all images should have
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub image: dyn Image
}

pub trait Renderer2D {
    // Constructor
    fn new() -> Self;
    // Create sprite
    fn create_sprite(width: u32, height: u32) -> Box<Sprite>;
    // Blit sprite (draw sprite on another base sprite)
    fn blit_sprite(&self, sprite: Box<Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32);
    // Resize sprite
    fn resize_sprite(&self, sprite: Box<Sprite>, width: u32, height: u32);
    // Render sprite
    fn draw_sprite(&self, sprite: Box<Sprite>, x: i32, y: i32);
    // Draw a rect which is just the outline
    fn draw_rect(&self, rect: Rect, color: Color);
    // Draw a filled rect
    fn draw_filled_rect(&self, rect: Rect, color: Color);
    // Draw a line
    fn draw_line(&self, ax: f32, ay: f32, bx: f32, by: f32);
    // Clear sprite
    fn clear(&self, sprite: Box<Sprite>, x: i32, y: i32, width: i32, height: i32);
    // Clear entire canvas
    fn clear_canvas(&self, x: i32, y: i32, width: i32, height: i32);
}

pub trait Renderer3D {}
