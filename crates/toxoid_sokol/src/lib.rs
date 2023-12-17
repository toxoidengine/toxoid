#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
mod bindings;
mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use sokol::{app as sapp, gfx as sg};
use bindings::*;
use core::ffi::c_int;
use core::ffi::c_char;

extern "C" {
    fn emscripten_set_canvas_element_size(id: *const c_char, width: c_int, height: c_int) -> c_int;
}

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State { pass_action: sg::PassAction::new() };

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

extern "C" fn frame() {
    let state = unsafe { &mut STATE };
    let (width, height) = (sapp::width(), sapp::height());

    let ratio = (width as f32) / (height as f32);
    println!("width: {}, height: {}", width, height);
    unsafe {
        // Begin recording draw commands for a frame buffer of size (width, height).
        sgp_begin(width, height);
        // Set frame buffer drawing region to (0,0,width,height).
        sgp_viewport(0, 0, width, height);
        // Set drawing coordinate space to (left=0, right=width, top=0, bottom=height).
        sgp_project(0.0, width as f32, 0.0, height as f32);
        // Clear the frame buffer.
        sgp_set_color(0.1, 0.1, 0.5, 1.0);
        sgp_clear();
        // Draw an animated rectangle that rotates and changes its colors.
        // let time = sapp::frame_count() as f32 * sapp::frame_duration() as f32;
        // let r = time.sin() * 0.5 + 0.5;
        // let g = time.cos() * 0.5 + 0.5;
        // sgp_set_color(r, g, 0.3, 1.0);
        // sgp_rotate_at(time, 0.0, 0.0);
        // sgp_draw_filled_rect(-0.5, -0.5, 1.0, 1.0);
        // render_2d::draw_filled_rect(
        //     sgp_rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 }, 
        //     sokol::gfx::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
        // );
        render_2d::draw_filled_rect(
            sgp_rect { x: 0.0, y: 0.0, w: 50., h: 50. }, 
            sokol::gfx::Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
        );
    }
     // Begin a render pass.
    sg::begin_default_pass(&state.pass_action, width, height);
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
        emscripten_set_canvas_element_size(canvas_id.as_ptr(), 800, 600);
    }
    sapp::run(&sapp::Desc {
        init_cb: Some(init),
        cleanup_cb: Some(cleanup),
        frame_cb: Some(frame),
        window_title,
        width: 800,
        height: 600,
        sample_count: 4,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}