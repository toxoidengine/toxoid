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

    fn blit_sprite(sprite: Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, dx: f32, dy: f32) {
        unsafe {
            let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sokol_sprite.image);
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn resize_sprite(sprite: Box<dyn Sprite>, width: u32, height: u32) {
        let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
        let old_image = sokol_sprite.image;

        // Create a new image with the desired dimensions
        let new_image_desc = sg_image_desc {
            width: width as i32,
            height: height as i32,
            // Copy other parameters from the old image
            ..unsafe { sg_query_image_desc(old_image) }
        };
        let _new_image = unsafe { sg_make_image(&new_image_desc) };

        // TODO: Copy old image data into the new image

        // Replace the old image with the new one
        // sokol_sprite.image = new_image;

        // Destroy the old image
        unsafe { sg_destroy_image(old_image) };
    }

    fn draw_sprite(sprite: Box<dyn Sprite>, x: i32, y: i32) {
        unsafe {
            let dest_rect = sgp_rect { x: x as f32, y: y as f32, w: sprite.width() as f32, h: sprite.height() as f32 };
            let src_rect = sgp_rect { x: 0., y: 0., w: sprite.width() as f32, h: sprite.height() as f32 };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sokol_sprite.image);
            sgp_draw_textured_rect(0, dest_rect, src_rect);
        }
    }

    fn draw_rect(rect: Rect, color: Color) {
        unsafe {
            sgp_set_color(color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a as f32 / 255.);
            sgp_draw_filled_rect(rect.x as f32, rect.y as f32, rect.width as f32, rect.height as f32);
        }
    }

    fn draw_filled_rect(rect: Rect, color: Color) {
        unsafe {
            sgp_set_color(color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a as f32 / 255.);
            sgp_draw_filled_rect(rect.x as f32, rect.y as f32, rect.width as f32, rect.height as f32);
        }
    }

    fn draw_line(ax: f32, ay: f32, bx: f32, by: f32) {
        unsafe {
            sgp_draw_line(ax, ay, bx, by);
        }
    }

    fn clear_sprite(_sprite: Box<dyn Sprite>, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            // Set a scissor rectangle to the desired area
            sgp_scissor(x, y, width, height);

            // Clear the scissor rectangle
            sgp_clear();

            // Reset the scissor rectangle to default
            sgp_reset_scissor();
        }
    }

    fn clear_canvas(x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            sgp_scissor(x, y, width, height);
            sgp_clear();
        }
    }
}

use core::ffi::{c_char, c_int, c_ulong};
use std::ffi::{CStr, CString};
pub const EMSCRIPTEN_FETCH_LOAD_TO_MEMORY: u32 = 1;
pub const EMSCRIPTEN_FETCH_STREAM_DATA: u32 = 2;
pub const EMSCRIPTEN_FETCH_PERSIST_FILE: u32 = 4;
pub const EMSCRIPTEN_FETCH_APPEND: u32 = 8;
pub const EMSCRIPTEN_FETCH_REPLACE: u32 = 16;
pub const EMSCRIPTEN_FETCH_NO_DOWNLOAD: u32 = 32;
pub const EMSCRIPTEN_FETCH_SYNCHRONOUS: u32 = 64;
pub const EMSCRIPTEN_FETCH_WAITABLE: u32 = 128;
pub type EM_BOOL = c_int;

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_fetch_attr_init(attr: *mut emscripten_fetch_attr_t);
    fn emscripten_fetch(attr: *const emscripten_fetch_attr_t, url: *const c_char) -> *mut emscripten_fetch_t;
}

#[cfg(target_os = "emscripten")]
#[repr(C)]
pub struct emscripten_fetch_attr_t {
    requestMethod: [c_char; 32],
    userData: *mut c_void,
    onsuccess: Option<extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    onerror: Option<extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    onprogress: Option<extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    attributes: u32,
    timeoutMSecs: c_ulong,
    withCredentials: EM_BOOL,
    destinationPath: *const c_char,
    userName: *const c_char,
    password: *const c_char,
    requestHeaders: *const *const c_char,
    overriddenMimeType: *const c_char,
    requestData: *const c_char,
    requestDataSize: usize,
}

#[cfg(target_os = "emscripten")]
#[repr(C)]
pub struct emscripten_fetch_t {
    id: u32,
    userData: *mut c_void,
    url: *const c_char,
    data: *const c_char,
    numBytes: u64,
    dataOffset: u64,
    totalBytes: u64,
    readyState: u16,
    status: u16,
    statusText: [c_char; 64],
    __proxyState: u32,
    __attributes: emscripten_fetch_attr_t,
}

#[cfg(target_os = "emscripten")]
pub fn load_image(filename: &str) -> sg_image {
    unsafe {
        println!("Loading image: {}", filename);
        let mut attr: emscripten_fetch_attr_t = std::mem::zeroed();
        emscripten_fetch_attr_init(&mut attr);
        attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;
        attr.onsuccess = Some(download_succeeded);
        attr.onerror = Some(download_failed);
        let c_string = CString::new(filename).unwrap();
        let result = emscripten_fetch(&attr, c_string.as_ptr());

        unimplemented!()
    }
}

#[cfg(not(target_os = "emscripten"))]
pub fn load_image(filename: &str) -> sg_image {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut channels: i32 = 0;

    let data = unsafe { stbi_load(filename.as_ptr() as *const i8, &mut width, &mut height, &mut channels, 4) };
    if data.is_null() {
        eprintln!("Failed to load image: {}", filename);
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

extern "C" fn download_succeeded(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("Success!");
        // let fetch_struct = *fetch;
        // let data = fetch_struct.data as *mut u8;
        // let size = fetch_struct.numBytes as usize;

        // // Create sg_image from data
        // let mut image_desc = sg_image_desc {
        //     _start_canary: 0,
        //     type_: sg_image_type_SG_IMAGETYPE_2D,
        //     render_target: false,
        //     width: 0, // You need to provide the width
        //     height: 0, // You need to provide the height
        //     num_slices: 1,
        //     num_mipmaps: 1,
        //     usage: sg_usage_SG_USAGE_IMMUTABLE,
        //     pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
        //     sample_count: 1,
        //     data: sg_image_data {
        //         subimage: [[sg_range { ptr: data as *const c_void, size }; 16]; 6],
        //     },
        //     label: ptr::null(),
        //     gl_textures: [0; 2usize],
        //     gl_texture_target: 0,
        //     mtl_textures: [ptr::null(); 2usize],
        //     d3d11_texture: ptr::null(),
        //     d3d11_shader_resource_view: ptr::null(),
        //     wgpu_texture: ptr::null(),
        //     wgpu_texture_view: ptr::null(),
        //     _end_canary: 0,
        // };

        // let image = sg_make_image(&mut image_desc);
        // emscripten_fetch_close(fetch);
        // image
    }
}

extern "C" fn download_failed(fetch: *mut emscripten_fetch_t) {
    unsafe {
        println!("Fail!");
        // eprintln!("Failed to load image: {}", CStr::from_ptr((*fetch).url).to_str().unwrap());
        // emscripten_fetch_close(fetch);
    }
}