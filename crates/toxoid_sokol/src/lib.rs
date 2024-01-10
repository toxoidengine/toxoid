#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

pub mod bindings;
pub mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub use render_2d::*;
pub use sokol;
use toxoid_api::components::GameConfig;
use toxoid_api::World;
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

        // Initialize SImGui
        let mut sgui_desc: simgui_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
        simgui_setup(&mut sgui_desc);

        let mut sspine_desc_obj: sspine_desc = core::mem::MaybeUninit::zeroed().assume_init();
        sspine_desc_obj.max_vertices = 1024;      // default: (1<<16) = 65536
        sspine_desc_obj.max_commands = 128;       // default: (1<<14) = 16384
        sspine_desc_obj.context_pool_size = 1;    // default: 4
        sspine_desc_obj.atlas_pool_size = 1;      // default: 64
        sspine_desc_obj.skeleton_pool_size = 1;   // default: 64
        sspine_desc_obj.skinset_pool_size = 1;    // default: 64
        sspine_desc_obj.instance_pool_size = 16;  // default: 1024
        sspine_desc_obj.logger.func = Some(sokol::log::slog_func);
        
        sspine_setup(&sspine_desc_obj);
    }
}

extern "C" fn cleanup_cb() {
    unsafe { sgp_shutdown(); }
    sg::shutdown()
}

pub fn init(frame_cb: extern "C" fn()) {
    let game_config = World::get_singleton::<GameConfig>();
    let window_title = b"Toxoid Engine Demo\0".as_ptr() as _;
    let canvas_id = std::ffi::CString::new("canvas").unwrap();
    
    unsafe {
        emscripten_set_canvas_element_size(canvas_id.as_ptr(), game_config.get_resolution_width() as i32, game_config.get_resolution_height() as i32);
    }

    // Initialize renderer
    sapp::run(&sapp::Desc {
        init_cb: Some(init_cb),
        cleanup_cb: Some(cleanup_cb),
        frame_cb: Some(frame_cb),
        window_title,
        width: game_config.get_resolution_width() as i32,
        height: game_config.get_resolution_height() as i32,
        sample_count: 1,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });
}