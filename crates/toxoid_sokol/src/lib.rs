#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;

mod bindings;
mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use render_2d::SokolRenderer2D;
use bindings::*;
use sokol::{app as sapp, gfx as sg};
use core::ffi::c_int;
use core::ffi::c_char;
use std::sync::Mutex;
use toxoid_render::Renderer2D;

lazy_static! {
    pub static ref RENDERER: Mutex<SokolRenderer2D> = Mutex::new(SokolRenderer2D::new());
}

extern "C" {
    fn emscripten_set_canvas_element_size(id: *const c_char, width: c_int, height: c_int) -> c_int;
}

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State { pass_action: sg::PassAction::new() };

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

    let x = 0.0 * scale_factor; // Change this to the x position of your square
    let y = 0.0 * scale_factor; // Change this to the y position of your square

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
        // Draw an animated rectangle that rotates and changes its colors.
        let renderer = &RENDERER.lock().unwrap();
        renderer.draw_filled_rect(
            toxoid_render::Rect { x: x as i32, y: y as i32, width: width as i32, height: height as i32 }, 
            toxoid_render::Color { r: 0, g: 1, b: 0, a: 1 }
        );
    //     renderer.draw_filled_rect(
    //         sgp_rect { x: x, y: y, w: width, h: height }, 
    //         sokol::gfx::Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    //     );
    //     renderer.draw_filled_rect(
    //         sgp_rect { x: 50.0 * scale_factor, y: 50.0 * scale_factor, w: width, h: height }, 
    //         sokol::gfx::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    //     );
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
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}