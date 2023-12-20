use crate::bindings::*;
use sokol::gfx::{self as sg};
use toxoid_render::{Renderer2D, RenderTarget, Sprite, Rect, Color};
use std::any::Any;
use std::ptr;
use std::os::raw::c_void;

pub struct SokolRenderer2D {}

pub struct SokolSprite {
    pub width: u32,
    pub height: u32,
    pub image: sg::Image,
}

pub struct SokolRenderTarget {
    pub sprite: SokolSprite,
    pub depth_image: sg::Sampler,
    pub sampler: sg::Sampler,
    pub pass: sg::Pass,
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
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

impl RenderTarget for SokolRenderTarget {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Renderer2D for SokolRenderer2D {
    fn new() -> Self {
        Self {}
    }

    fn create_render_target(width: u32, height: u32) -> Box<dyn RenderTarget> {
        // let image_desc = sg::ImageDesc {
        //     width: width as i32,
        //     height: height as i32,
        //     _type: sg::ImageType::Dim2,
        //     usage: sg::Usage::Immutable,
        //     // pixel_format: sg::PixelFormat::Rgba8,
        //     pixel_format: sg::PixelFormat::Depth,
        //     render_target: true,
        //     ..Default::default()
        // };
        // let image = sg::make_image(&image_desc);
        // Box::new(SokolSprite {
        //     width,
        //     height,
        //     image,
        // })

        // Create framebuffer image
        let fb_image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            ..Default::default()
        };
        let fb_image = sg::make_image(&fb_image_desc);

        // Create framebuffer depth stencil
        let fb_depth_image_desc = sg::ImageDesc {
            render_target: true,
            width: width as i32,
            height: height as i32,
            pixel_format: sg::PixelFormat::Depth, // DepthStencil maybe?
            ..Default::default()
        };
        let fb_depth_image = sg::make_image(&fb_depth_image_desc);

        // Create linear sampler
        let linear_sampler_desc = sg::SamplerDesc {
            min_filter: sg::Filter::Linear,
            mag_filter: sg::Filter::Linear,
            wrap_u: sg::Wrap::ClampToEdge,
            wrap_v: sg::Wrap::ClampToEdge,
            ..Default::default()
        };
        let linear_sampler = sg::make_sampler(&linear_sampler_desc);

        // Create framebuffer pass
        let pass_desc = sg::PassDesc {
            color_attachments: [sg::PassAttachmentDesc {
                image: fb_image,
                ..Default::default()
            }; 4],
            depth_stencil_attachment: sg::PassAttachmentDesc {
                image: fb_depth_image,
                ..Default::default()
            },
            ..Default::default()
        };
        let fb_pass = sg::make_pass(&pass_desc);

        Box::new(SokolRenderTarget {
            sprite: SokolSprite {
                width,
                height,
                image: sg::Image { id: fb_image.id },
            },
            depth_image: sg::Sampler { id: fb_depth_image.id },
            sampler: sg::Sampler { id: linear_sampler.id },
            pass: sg::Pass { id: fb_pass.id },
        })
    }

    fn create_sprite(filename: &str) -> Box<dyn Sprite> {
        let image = load_image(filename);

        let desc = unsafe { sg_query_image_desc(image) };
        let (width, height) = (desc.width, desc.height);

        Box::new(SokolSprite {
            width: width as u32,
            height: height as u32,
            image: sg::Image { id: image.id },
        })
    }

    fn blit_sprite(source: &Box<dyn Sprite>, sx: f32, sy: f32, sw: f32, sh: f32, destination: &Box<dyn RenderTarget>, dx: f32, dy: f32) {
        unsafe {      // Get the source and destination Sokol sprites
            let sokol_source = source.as_any().downcast_ref::<SokolSprite>().unwrap();
            let sokol_destination = destination.as_any().downcast_ref::<SokolRenderTarget>().unwrap();
    
            // Set the source image
            sgp_set_image(0, sg_image { id: sokol_source.image.id });

            // Set the framebuffer as the current render target
            sg::begin_pass(sokol_destination.pass, &sg::PassAction::default());
    
            // Draw the source sprite onto the destination sprite
            let src_rect = sgp_rect { x: sx, y: sy, w: sw, h: sh };
            let dest_rect = sgp_rect { x: dx, y: dy, w: sw, h: sh };
            sgp_draw_textured_rect(0, dest_rect, src_rect);
    
            // End the pass to apply the drawing commands to the framebuffer
            sg::end_pass();
    
            // Destroy the framebuffer after use
            // sg::destroy_pass(sokol_destination.pass);
        }
    }

    fn resize_sprite(sprite: &Box<dyn Sprite>, width: u32, height: u32) {
        // let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
        // let old_image = sokol_sprite.image;

        // // Create a new image with the desired dimensions
        // let new_image_desc = sg_image_desc {
        //     width: width as i32,
        //     height: height as i32,
        //     // Copy other parameters from the old image
        //     ..unsafe { sg_query_image_desc(old_image) }
        // };
        // let _new_image = unsafe { sg_make_image(&new_image_desc) };

        // // TODO: Copy old image data into the new image

        // // Replace the old image with the new one
        // // sokol_sprite.image = new_image;

        // // Destroy the old image
        // unsafe { sg_destroy_image(old_image) };
        unimplemented!()
    }

    fn draw_sprite(sprite: &Box<dyn Sprite>, x: f32, y: f32, scale_factor: f32) {
        unsafe {
            let dest_rect = sgp_rect { 
                x: (x * scale_factor).round(), 
                y: (y * scale_factor).round(), 
                w: (sprite.width() as f32 * scale_factor).round(), 
                h: (sprite.height() as f32 * scale_factor).round()
            };
            let src_rect = sgp_rect { 
                x: 0., 
                y: 0., 
                w: sprite.width() as f32, 
                h: sprite.height() as f32 
            };
            let sokol_sprite = sprite.as_any().downcast_ref::<SokolSprite>().unwrap();
            sgp_set_image(0, sg_image { id: sokol_sprite.image.id });
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

    fn clear_sprite(_sprite: &Box<dyn Sprite>, x: i32, y: i32, width: i32, height: i32) {
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

use core::ffi::{c_char, c_int};
use std::ffi::CString;

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
#[allow(non_snake_case)]
#[repr(C)]
pub struct emscripten_fetch_attr_t {
    pub requestMethod: [::std::os::raw::c_char; 32usize],
    pub userData: *mut ::std::os::raw::c_void,
    pub onsuccess: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onerror: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onprogress: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onreadystatechange:
        ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub attributes: u32,
    pub timeoutMSecs: u32,
    pub withCredentials: ::std::os::raw::c_int,
    pub destinationPath: *const ::std::os::raw::c_char,
    pub userName: *const ::std::os::raw::c_char,
    pub password: *const ::std::os::raw::c_char,
    pub requestHeaders: *const *const ::std::os::raw::c_char,
    pub overriddenMimeType: *const ::std::os::raw::c_char,
    pub requestData: *const ::std::os::raw::c_char,
    pub requestDataSize: usize,
}

#[cfg(target_os = "emscripten")]
#[allow(non_snake_case)]
#[repr(C)]
pub struct emscripten_fetch_t {
    pub id: u32,
    pub userData: *mut ::std::os::raw::c_void,
    pub url: *const ::std::os::raw::c_char,
    pub data: *const ::std::os::raw::c_char,
    pub numBytes: u64,
    pub dataOffset: u64,
    pub totalBytes: u64,
    pub readyState: ::std::os::raw::c_ushort,
    pub status: ::std::os::raw::c_ushort,
    pub statusText: [::std::os::raw::c_char; 64usize],
    pub __proxyState: u32,
    pub __attributes: emscripten_fetch_attr_t,
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
        
        let filename_cstring = CString::new(filename).unwrap();
        emscripten_fetch(&attr, filename_cstring.as_ptr());

        let data: &[u8; 2042] = &[137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,100,0,0,0,100,8,6,0,0,0,112,226,149,84,0,0,0,1,115,82,71,66,0,174,206,28,233,0,0,7,180,73,68,65,84,120,156,237,155,95,108,83,85,28,199,191,43,108,149,22,11,140,165,101,122,47,25,99,33,125,25,144,78,147,117,26,171,201,162,9,2,46,70,129,128,188,248,160,15,227,129,73,67,252,19,141,98,8,132,12,225,65,18,49,132,151,133,133,248,96,96,26,18,12,6,49,102,163,14,22,180,211,52,99,20,93,27,106,107,237,160,180,37,101,75,175,15,119,231,236,254,233,6,140,177,158,234,239,243,116,239,57,231,254,238,185,231,219,243,251,253,206,185,183,0,65,16,4,65,16,4,65,16,4,65,16,4,65,16,4,65,16,4,65,16,4,65,16,4,65,16,4,65,16,196,92,81,81,234,14,220,15,126,159,87,121,144,246,157,23,250,202,226,185,138,49,191,212,29,152,14,38,196,241,69,109,186,114,119,238,26,70,6,206,0,0,150,123,214,33,100,91,105,184,14,10,80,158,194,8,219,97,191,207,171,28,95,212,6,119,238,26,146,177,48,114,177,144,174,62,157,85,39,141,195,174,127,4,91,173,27,53,181,245,8,217,86,226,205,91,167,202,78,20,33,59,235,247,121,149,222,202,70,157,16,5,171,19,139,157,18,110,38,162,200,164,226,92,136,116,86,193,194,106,23,175,179,228,19,0,38,133,105,25,11,150,149,40,194,117,212,40,134,67,246,152,218,48,81,0,112,49,140,164,35,3,101,41,138,165,212,29,40,134,81,140,101,114,29,150,201,117,188,126,177,83,130,228,110,130,228,110,210,137,161,109,227,144,61,200,197,66,104,25,11,162,39,105,127,224,196,160,84,8,37,136,223,231,85,122,146,118,147,24,0,240,87,228,15,222,78,59,240,218,99,227,185,67,246,224,100,48,130,141,53,89,110,255,145,117,126,150,16,98,26,75,213,146,110,160,180,110,106,153,92,199,197,184,153,136,242,25,49,213,113,49,1,211,145,1,126,188,165,81,22,218,125,9,209,49,169,90,82,28,178,71,55,152,218,65,6,166,23,35,147,138,67,114,55,153,236,178,118,90,187,233,200,0,162,169,168,16,207,93,12,33,214,33,209,84,180,66,2,148,155,137,40,47,203,164,226,250,204,201,234,4,48,41,0,19,132,101,85,108,240,211,145,1,158,145,177,192,207,236,90,242,137,137,25,18,133,168,8,17,67,252,62,175,98,171,117,195,146,79,32,147,138,235,126,241,108,80,25,76,0,64,117,69,221,187,54,152,214,34,236,26,201,221,196,237,89,242,9,216,106,221,194,7,120,33,4,1,192,3,249,194,106,151,174,156,13,54,251,245,27,5,168,95,177,0,128,42,194,205,68,20,14,217,163,171,103,246,210,89,5,185,88,200,180,192,20,141,146,11,194,126,173,5,171,211,20,15,180,25,19,115,59,76,128,104,232,50,10,86,39,154,119,124,197,69,98,162,104,175,101,174,109,97,181,11,14,217,131,45,141,242,28,61,217,204,152,87,202,155,251,125,94,101,247,145,14,60,243,178,23,233,64,63,126,187,17,67,229,227,46,60,102,119,0,0,66,63,127,15,107,85,5,28,178,7,119,83,195,72,103,21,116,157,251,29,221,187,54,224,219,75,67,184,157,206,32,63,6,124,123,105,72,205,204,238,252,133,219,233,12,172,85,21,248,59,241,15,183,83,40,20,144,73,197,177,165,161,18,236,126,202,111,209,143,123,255,140,126,82,202,231,47,70,201,103,8,99,247,145,14,221,175,119,153,92,7,135,93,21,227,139,77,79,234,92,209,214,131,223,224,210,137,247,213,122,182,133,98,112,103,198,216,243,86,243,114,236,62,210,49,71,79,51,115,74,158,254,249,125,94,197,255,238,118,140,143,14,163,106,117,51,60,207,189,131,130,213,9,119,211,179,24,234,253,26,23,63,223,132,249,75,26,0,0,227,163,195,252,216,34,215,0,0,10,145,164,169,110,124,116,24,205,59,190,194,170,150,87,17,186,252,19,44,249,4,6,126,252,12,119,127,189,136,249,75,26,208,185,191,75,216,181,72,201,211,222,137,129,81,118,31,233,64,232,244,121,94,206,22,118,135,143,246,1,232,227,229,59,223,6,170,86,55,243,115,139,92,131,187,191,94,52,181,51,18,58,125,30,238,87,94,192,129,246,67,194,138,1,8,224,178,164,106,73,57,25,140,224,64,251,33,184,95,121,193,84,191,190,179,11,189,149,141,56,190,168,13,189,149,141,56,124,180,143,207,10,64,157,33,135,143,246,233,218,172,239,236,2,160,95,173,51,49,78,6,35,166,157,1,145,40,185,32,128,186,85,210,61,148,199,129,246,67,166,186,53,13,75,1,0,79,124,185,3,0,248,96,107,97,101,172,13,187,70,139,231,185,119,208,61,148,47,186,123,44,18,37,119,89,108,149,190,88,246,224,203,139,151,77,139,188,189,109,235,208,2,0,155,91,1,168,131,157,79,234,109,172,105,88,138,150,177,32,111,179,183,109,157,233,62,233,172,2,73,150,132,223,58,17,166,99,82,181,164,164,179,10,28,246,10,254,210,201,146,79,160,123,215,6,172,242,52,242,32,14,168,110,74,27,212,141,117,67,3,65,108,61,248,13,10,86,39,127,153,197,108,139,44,6,32,136,203,2,212,153,194,102,7,91,228,1,234,66,80,59,224,140,208,233,243,186,36,128,97,145,107,116,171,119,173,77,209,197,0,4,112,89,90,162,169,104,5,11,184,108,32,89,42,171,197,34,215,160,126,116,1,63,54,194,174,209,186,191,114,16,3,16,76,16,64,47,202,116,104,83,223,251,177,249,80,157,154,67,132,237,40,19,101,75,163,12,255,187,219,139,198,9,0,166,248,242,250,107,251,167,181,123,33,52,32,236,51,3,2,11,162,253,12,168,101,44,104,18,133,97,20,224,196,71,207,79,107,119,219,158,31,116,231,162,9,36,156,203,2,38,197,96,244,36,237,192,254,46,157,40,76,8,163,0,225,235,119,208,243,221,21,180,174,117,193,230,172,53,29,27,219,111,219,163,126,84,39,138,48,194,9,162,21,195,157,187,166,171,235,156,16,101,115,199,49,62,176,131,129,176,105,224,167,131,181,207,37,98,56,119,37,206,237,108,219,3,69,4,81,132,73,123,129,226,98,36,99,97,108,172,201,162,39,105,71,79,210,206,197,24,12,132,39,246,175,30,142,240,245,59,24,12,132,113,226,163,231,225,115,123,74,190,165,34,148,32,12,173,24,183,151,220,229,229,53,181,245,165,234,210,156,33,140,32,218,32,14,76,138,177,109,222,24,122,146,246,255,133,24,128,96,49,228,205,91,167,0,168,65,92,59,51,140,132,175,223,153,245,123,63,10,155,51,65,40,65,62,56,117,6,191,12,255,3,248,183,243,50,237,236,8,217,86,162,17,151,74,213,189,57,65,24,151,5,168,187,180,107,26,150,98,125,103,23,92,111,124,0,0,58,49,24,108,175,106,54,121,20,54,103,130,16,51,132,125,236,0,76,110,157,51,65,254,111,8,33,136,22,38,204,129,246,189,83,182,249,47,199,16,161,92,22,48,249,209,194,206,183,189,186,114,150,125,5,87,109,198,135,93,129,89,191,239,135,93,1,33,86,235,194,9,2,64,247,206,60,25,11,243,99,227,202,253,191,136,80,130,20,34,73,140,143,14,99,124,116,88,183,10,55,138,50,38,61,133,125,103,71,102,237,190,251,206,142,8,49,59,0,65,98,72,231,133,190,10,180,67,217,248,226,90,83,29,251,179,13,198,130,186,242,254,185,232,88,9,16,66,16,96,242,251,44,237,185,223,231,85,90,215,170,31,75,215,89,51,186,246,173,121,23,246,157,29,65,235,67,222,87,148,216,193,16,70,16,96,234,255,149,219,156,181,72,104,206,157,183,174,226,220,149,56,158,118,89,177,239,236,8,62,221,62,253,14,239,84,244,199,243,66,137,1,8,22,67,138,193,102,70,253,138,5,69,23,111,71,55,86,207,40,235,18,41,110,104,17,94,16,0,200,37,98,69,143,25,239,189,180,28,253,241,252,125,219,19,205,77,105,17,94,144,99,129,44,234,172,25,12,6,194,24,12,132,81,103,205,224,88,32,107,170,127,239,165,229,247,204,188,250,227,121,97,103,6,67,168,24,82,140,117,246,28,142,5,0,32,171,43,59,147,181,153,234,159,118,89,185,251,210,6,123,85,40,177,133,96,8,223,65,150,105,177,87,179,236,213,43,75,0,166,170,215,186,176,114,16,130,33,252,12,1,128,170,171,215,225,156,8,238,55,174,254,13,192,118,207,250,114,18,65,75,89,116,218,248,175,89,99,122,124,175,122,130,32,8,130,32,8,130,32,8,130,32,8,130,32,8,130,32,8,130,32,8,130,32,8,130,32,8,130,32,4,225,95,100,66,167,241,95,226,89,34,0,0,0,0,73,69,78,68,174,66,96,130];

        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut channels: i32 = 0;

        let image_data = unsafe {
            stbi_load_from_memory(data as *const u8, data.len() as c_int, &mut width, &mut height, &mut channels, 0)
        };

        // Create sg_image from data
        let mut image_desc = sg_image_desc {
            _start_canary: 0,
            type_: sg_image_type_SG_IMAGETYPE_2D,
            render_target: false,
            width: 100, // You need to provide the width
            height: 100, // You need to provide the height
            num_slices: 1,
            num_mipmaps: 1,
            usage: sg_usage_SG_USAGE_IMMUTABLE,
            pixel_format: sg_pixel_format_SG_PIXELFORMAT_RGBA8,
            sample_count: 1,
            data: sg_image_data {
                subimage: [[sg_range { ptr: image_data as *const c_void, size: (width * height * 4) as usize }; 16]; 6],
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

        let image = sg_make_image(&mut image_desc);
        // emscripten_fetch_close(fetch);
        image
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

extern "C" fn download_succeeded(result: *mut emscripten_fetch_t) {
    unsafe {
        // println!("Successfully loaded {}", CStr::from_ptr((*result).url).to_str().unwrap());
        let data = (*result).data as *mut u8;
        let size = (*result).totalBytes as usize;

        // Create sg_image from data
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