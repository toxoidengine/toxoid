use crate::bindings::*;
use sokol::app as sapp;
use sokol::gfx as sg;
use toxoid_render::{Renderer2D, Sprite, Rect, Color};
use std::any::Any;
use std::ptr;
use std::os::raw::c_void;

pub struct SokolRenderer2D {}

pub struct SokolSprite {
    pub width: u32,
    pub height: u32,
    pub image: sg_image,
}

impl Sprite for SokolSprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}

impl Renderer2D for SokolRenderer2D {
    fn new() -> Self {
        Self {}
    }

    fn create_sprite(filename: &str) -> Box<dyn Sprite> {
        let image = load_image(filename);

        let desc = unsafe { sg_query_image_desc(image) };
        let (width, height) = (desc.width, desc.height);

        Box::new(SokolSprite {
            width: width as u32,
            height: height as u32,
            image,
        })
    }

    fn blit_sprite(&self, sprite: Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32) {
        unsafe {
            let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sokol_sprite.image);
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn resize_sprite(&self, sprite: Box<dyn Sprite>, width: u32, height: u32) {
        unimplemented!();
    }

    fn draw_sprite(&self, sprite: Box<dyn Sprite>, x: i32, y: i32) {
        unsafe {
            let dest_rect = sgp_rect { x: x as f32, y: y as f32, w: sprite.width() as f32, h: sprite.height() as f32 };
            let src_rect = sgp_rect { x: 0., y: 0., w: sprite.width() as f32, h: sprite.height() as f32 };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sokol_sprite.image);
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn draw_rect(&self, rect: Rect, color: Color) {
        unsafe {
            sgp_set_color(color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a as f32 / 255.);
            sgp_draw_filled_rect(rect.x as f32, rect.y as f32, rect.width as f32, rect.height as f32);
        }
    }

    fn draw_filled_rect(&self, rect: Rect, color: Color) {
        unsafe {
            sgp_set_color(color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a as f32 / 255.);
            sgp_draw_filled_rect(rect.x as f32, rect.y as f32, rect.width as f32, rect.height as f32);
        }
    }

    fn draw_line(&self, ax: f32, ay: f32, bx: f32, by: f32) {
        unsafe {
            sgp_draw_line(ax, ay, bx, by);
        }
    }

    fn clear(&self, sprite: Box<dyn Sprite>, x: i32, y: i32, width: i32, height: i32) {
        unimplemented!();
    }

    fn clear_canvas(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            sgp_scissor(x, y, width, height);
            sgp_clear();
        }
    }
}

pub fn load_image(filename: &str) -> sg_image {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut channels: i32 = 0;

    let data = unsafe { stbi_load(filename.as_ptr() as *const i8, &mut width, &mut height, &mut channels, 4) };
    if data.is_null() {
        return sg_image { id: SG_INVALID_ID as u32 };
    }

    let size = (width * height * 4) as usize;

    let mut image_desc = sg_image_desc {
        _start_canary: 0,
        type_: sg_image_type_SG_IMAGETYPE_2D,
        render_target: false,
        width,
        height,
        num_slices: 1,
        num_mipmaps: 1,
        usage: sg_usage_SG_USAGE_IMMUTABLE,
        pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
        sample_count: 1,
        data: sg_image_data {
            subimage: [[sg_range { ptr: data as *const c_void, size }; 16]; 6],
        },
        label: ptr::null(),
        gl_textures: [0; 2usize],
        gl_texture_target: 0,
        mtl_textures: [ptr::null(); 2usize],
        d3d11_texture: ptr::null(),
        d3d11_shader_resource_view: ptr::null(),
        wgpu_texture: ptr::null(),
        wgpu_texture_view: ptr::null(),
        _end_canary: 0,
    };

    let image = unsafe { sg_make_image(&mut image_desc) };
    unsafe { stbi_image_free(data as *mut c_void) };
    image
}