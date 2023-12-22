#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

mod bindings;
pub mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub use render_2d::*;
use toxoid_render::Renderer2D;
use bindings::*;
use sokol::{app as sapp, gfx as sg};
use core::ffi::c_int;
use core::ffi::c_char;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static RENDERER_2D: Lazy<Mutex<SokolRenderer2D>> = Lazy::new(|| Mutex::new(SokolRenderer2D::new()));

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

extern "C" fn init_cb() {
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

        // // Create sprite image and render target to blit on
        // SPRITE = Some(SokolRenderer2D::create_sprite("assets/character.png"));
        // RENDER_TARGET = Some(SokolRenderer2D::create_render_target(100, 100));
        
        // if let Some(sprite) = &mut SPRITE {
        //     if let Some(render_target) = &mut RENDER_TARGET {
        //         // Blit sprite on render target
        //         SokolRenderer2D::blit_sprite(sprite, 0., 0., 100., 100., render_target, 0., 0.);
        //         // SokolRenderer2D::clear_sprite(render_target, 0, 0, 50, 50);
        //     }
        // }
    }
}

extern "C" fn cleanup_cb() {
    unsafe { sgp_shutdown(); }
    sg::shutdown()
}

pub fn init(frame_cb: extern "C" fn()) {
    let window_title = b"Toxoid Engine Demo\0".as_ptr() as _;
    let canvas_id = std::ffi::CString::new("canvas").unwrap();
    unsafe {
        emscripten_set_canvas_element_size(canvas_id.as_ptr(), RESOLUTION_WIDTH.try_into().unwrap(), RESOLUTION_HEIGHT.try_into().unwrap());
    }
    // Initialize renderer
    sapp::run(&sapp::Desc {
        init_cb: Some(init_cb),
        cleanup_cb: Some(cleanup_cb),
        frame_cb: Some(frame_cb),
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