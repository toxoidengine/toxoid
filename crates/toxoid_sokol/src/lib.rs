#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod bindings;
mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use render_2d::SokolRenderer2D;
use toxoid_render::Renderer2D;
use bindings::*;
use sokol::{app as sapp, gfx as sg};
use toxoid_render::Sprite;
use core::ffi::c_int;
use core::ffi::c_char;

extern "C" {
    fn emscripten_set_canvas_element_size(id: *const c_char, width: c_int, height: c_int) -> c_int;
}

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State { pass_action: sg::PassAction::new() };
static mut SPRITE: Option<Box<dyn toxoid_render::Sprite>> = None;
static mut RENDER_TARGET: Option<Box<dyn toxoid_render::RenderTarget>> = None;

const RESOLUTION_WIDTH: u32 = 1280;
const RESOLUTION_HEIGHT: u32 = 720;

extern "C" fn init() {
    // Setup sokol app
    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    unsafe {
        // Initialize Sokol GP, adjust the size of command buffers for your own use.
        let mut desc = sgp_desc {
            max_commands: 0,
            max_vertices: 0,
            pixel_format: 0,
        };
        sgp_setup(&mut desc);

        // Create sprite image and render target to blit on
        SPRITE = Some(SokolRenderer2D::create_sprite("assets/character.png"));
        RENDER_TARGET = Some(SokolRenderer2D::create_render_target(100, 100));
        
        if let Some(sprite) = &mut SPRITE {
            if let Some(render_target) = &mut RENDER_TARGET {
                // Blit sprite on render target
                SokolRenderer2D::blit_sprite(sprite, 0., 0., 100., 100., render_target, 0., 0.);
                SokolRenderer2D::clear_sprite(render_target, 0, 0, 50, 50);
            }
        }
    }
}

/*
    Calculates the width and height of the image based on the aspect ratio of the window and the image. If the window's aspect ratio is greater than or equal to the image's aspect ratio, it sets the width to the window's width and calculates the height based on the image's aspect ratio. If the window's aspect ratio is less than the image's aspect ratio, it sets the height to the window's height and calculates the width based on the image's aspect ratio.
*/
extern "C" fn frame() {
    let state = unsafe { &mut STATE };
    let (window_width, window_height) = (sapp::width(), sapp::height());

    // The object will scale based on the window width, ensuring it always fits within the window without distortion.
    let scale_factor = window_width as f32 / RESOLUTION_WIDTH as f32;
    let width = 50.0 * scale_factor;
    let height = 50.0 * scale_factor;

    let x = 0.0 * scale_factor; // Change this to the x position of sprite
    let y = 0.0 * scale_factor; // Change this to the y position of sprite

    let x_sprite = 100. * scale_factor; // Change this to the x position of sprite
    let y_sprite = 100. * scale_factor; // Change this to the y position of sprite
    
    unsafe {
        // Begin recording draw commands for a frame buffer of size (width, height).
        sgp_begin(window_width, window_height);
        // Set frame buffer drawing region to (0,0,width,height).
        sgp_viewport(0, 0, window_width, window_height);
        // Set drawing coordinate space to (left=0, right=width, top=0, bottom=height).
        sgp_project(0.0, window_width as f32, 0.0, window_height as f32);
        // Clear the frame buffer.
        sgp_set_color(0.1, 0.1, 0.1, 1.0);
        sgp_clear();
        
        SokolRenderer2D::draw_filled_rect(
            toxoid_render::Rect { x: x as i32, y: y as i32, width: width as i32, height: height as i32 }, 
            toxoid_render::Color { r: 0, g: 255, b: 0, a: 255 }
        );
        if let Some(render_target) = &mut RENDER_TARGET {
            sgp_reset_color();
            sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
            let target = render_target.as_any().downcast_ref::<render_2d::SokolRenderTarget>().unwrap();
            SokolRenderer2D::draw_sprite(&target.sprite, x_sprite, y_sprite, scale_factor);
            // sgp_reset_blend_mode();
        }  
    }
    // Begin a render pass.
    sg::begin_default_pass(&state.pass_action, window_width, window_height);
    unsafe { 
        // Dispatch all draw commands to Sokol GFX.
        sgp_flush(); 
        // Finish a draw command queue, clearing it.
        sgp_end();
    }
    // End render pass.
    sg::end_pass();
    // Commit Sokol render.
    sg::commit();
}

extern "C" fn cleanup() {
    unsafe { sgp_shutdown(); }
    sg::shutdown()
}

pub fn sokol() {
    let window_title = b"Rectangle (Sokol GP)\0".as_ptr() as _;
    let canvas_id = std::ffi::CString::new("canvas").unwrap();
    unsafe {
        emscripten_set_canvas_element_size(canvas_id.as_ptr(), RESOLUTION_WIDTH.try_into().unwrap(), RESOLUTION_HEIGHT.try_into().unwrap());
    }
    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        window_title,
        width: RESOLUTION_WIDTH as i32,
        height: RESOLUTION_HEIGHT as i32,
        sample_count: 1,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}