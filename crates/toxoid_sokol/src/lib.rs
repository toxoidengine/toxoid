#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(improper_ctypes)]

#[cfg(not(target_os = "emscripten"))]
pub mod bindings;
#[cfg(target_os = "emscripten")]
pub mod bindings_x86;
#[cfg(not(target_os = "emscripten"))]
use bindings::*;
#[cfg(target_os = "emscripten")]
use bindings_x86::*;
pub mod render_2d;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub use render_2d::*;
pub use sokol;
// use toxoid_api::components::GameConfig;
// use toxoid_api::World;
// use toxoid_render::Renderer2D;

use sokol::{app as sapp, gfx as sg, glue as sglue};
// use core::ffi::c_int;
// use core::ffi::c_char;
// use std::sync::Mutex;
// use sokol::app::{Event, EventType};
use sokol::app::Event;
use once_cell::sync::Lazy;

// Global pass action for Sokol
pub static PASS_ACTION: Lazy<sg::PassAction> = Lazy::new(|| sg::PassAction::new());

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_set_canvas_element_size(id: *const core::ffi::c_char, width: core::ffi::c_int, height: core::ffi::c_int) -> core::ffi::c_int;
}

const SOKOL_POOL_MODIFIER: i32 = 100;

pub extern "C" fn sokol_init() {
    // Setup sokol app
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        // image_pool_size: 128 * SOKOL_POOL_MODIFIER,
        // buffer_pool_size: 128 * SOKOL_POOL_MODIFIER,
        // shader_pool_size: 32 * SOKOL_POOL_MODIFIER,
        // pipeline_pool_size: 64 * SOKOL_POOL_MODIFIER,
        // sampler_pool_size: 64 * SOKOL_POOL_MODIFIER,
        // uniform_buffer_size: 4 * 1024 * 1024 * SOKOL_POOL_MODIFIER,
        // attachments_pool_size: 64 * SOKOL_POOL_MODIFIER,
        // wgpu_bindgroups_cache_size: 64 * SOKOL_POOL_MODIFIER,
        ..Default::default()
    });

    unsafe {
        #[cfg(target_os = "emscripten")] 
        {
            // Get the default swapchain info first so we don't get a pipeline mismatch
            let swapchain = sglue::swapchain();
            
            // Initialize Sokol GP with matching swapchain configuration
            let mut desc = sgp_desc {
                max_commands: 10000,
                max_vertices: 50000,
                // Match the color format with the swapchain
                color_format: swapchain.color_format as i32,
                // Match the depth format with the swapchain
                depth_format: swapchain.depth_format as i32,
                // Match the sample count with the swapchain
                sample_count: swapchain.sample_count,
            };

            sgp_setup(&mut desc);
        }

        #[cfg(not(target_os = "emscripten"))]
        {
            // Initialize Sokol GP, adjust the size of command buffers for your own use.
            let mut desc = sgp_desc {
                max_commands: 0,
                max_vertices: 0,
                color_format: 0,
                depth_format: 0,
                sample_count: 0,
            };
            sgp_setup(&mut desc);
        }
        
        // // Initialize SImGui
        // #[cfg(feature = "imgui")] 
        // {
        //     let mut sgui_desc: simgui_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
        //     simgui_setup(&mut sgui_desc);
        // }
        
        // // Initialize Spine
        // let mut sspine_desc_obj: sspine_desc = core::mem::MaybeUninit::zeroed().assume_init();
        // sspine_desc_obj.max_vertices = 1024;      // default: (1<<16) = 65536
        // sspine_desc_obj.max_commands = 128;       // default: (1<<14) = 16384
        // sspine_desc_obj.context_pool_size = 64;    // default: 4
        // sspine_desc_obj.atlas_pool_size = 64;      // default: 64
        // sspine_desc_obj.skeleton_pool_size = 64;   // default: 64
        // sspine_desc_obj.skinset_pool_size = 64;    // default: 64
        // sspine_desc_obj.instance_pool_size = 16;  // default: 1024
        // sspine_desc_obj.logger.func = Some(sokol::log::slog_func);
        // sspine_setup(&sspine_desc_obj);

        // // Initialize SFetch
        // let mut sfetch_desc: sfetch_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
        // sfetch_desc.max_requests = 1024;
        // sfetch_desc.num_channels = 4;
        // sfetch_desc.num_lanes = 8;
        // sfetch_desc.logger.func = Some(sokol::log::slog_func);     
        // let sfetch_desc = Box::into_raw(Box::new(sfetch_desc));
        // sfetch_setup(sfetch_desc); 
        // // sfetch_setup(&mut sfetch_desc);
        // if !sfetch_valid() {
        //     panic!("sfetch is not valid");
        // }
    }
}

extern "C" fn sokol_cleanup() {
    // unsafe { sgp_shutdown(); }
    sg::shutdown()
}

pub const GAME_WIDTH: i32 = 800;
pub const GAME_HEIGHT: i32 = 600;

#[cfg(feature = "render")]
pub fn init(sokol_init: extern "C" fn(), sokol_frame: extern "C" fn(), sokol_event: extern "C" fn(*const Event)) {
    // let game_config = World::get_singleton::<GameConfig>();
    let window_title = b"Toxoid Engine Demo\0".as_ptr() as _;
    let canvas_id = std::ffi::CString::new("canvas").unwrap();
    
    #[cfg(target_os = "emscripten")]
    unsafe {
        emscripten_set_canvas_element_size(canvas_id.as_ptr(), GAME_WIDTH, GAME_HEIGHT);
    }

    // Initialize renderer
    // #[cfg(target_os="emscripten")] {
    //     sapp::run(&sapp::Desc {
    //         init_cb: Some(sokol_init),
    //         cleanup_cb: Some(sokol_cleanup),
    //         event_cb: Some(sokol_event),
    //         window_title,
    //         width: GAME_WIDTH,
    //         height: GAME_HEIGHT,
    //         sample_count: 1,
    //         icon: sapp::IconDesc {
    //             sokol_default: true,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });
    //     // sokol_init();
    // }

    sapp::run(&sapp::Desc {
        init_cb: Some(sokol_init),
        cleanup_cb: Some(sokol_cleanup),
        frame_cb: Some(sokol_frame),
        event_cb: Some(sokol_event),
        window_title,
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
        sample_count: 1,
        icon: sapp::IconDesc {
            sokol_default: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // #[cfg(not(target_arch="wasm32"))] {
    //     sapp::run(&sapp::Desc {
    //         init_cb: Some(sokol_init),
    //         cleanup_cb: Some(sokol_cleanup),
    //         frame_cb: Some(sokol_frame),
    //         event_cb: Some(sokol_event),
    //         window_title,
    //         width: GAME_WIDTH,
    //         height: GAME_HEIGHT,
    //         sample_count: 1,
    //         icon: sapp::IconDesc {
    //             sokol_default: true,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     });
    // }
}