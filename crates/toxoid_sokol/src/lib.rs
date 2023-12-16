#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
mod bindings;
use sokol::{app as sapp, gfx as sg};

struct State {
    pass_action: sg::PassAction,
}

static mut STATE: State = State { pass_action: sg::PassAction::new() };

extern "C" fn init() {
    let state = unsafe { &mut STATE };

    sg::setup(&sg::Desc {
        context: sokol::glue::context(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };

    // Initialize Sokol GP, adjust the size of command buffers for your own use.
    // sgp_desc sgpdesc = {0};
    // sgp_setup(&sgpdesc);

    unsafe {
        let mut desc = bindings::sgp_desc {
            // max_commands: 1024,
            // max_vertices: 1024,
            // pixel_format: bindings::sg_pixel_format::SG_PIXELFORMAT_RGBA8,
            max_commands: 0,
            max_vertices: 0,
            pixel_format: 0,
        };
        bindings::sgp_setup(&mut desc);
    }
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };
    let (width, height) = (sapp::width(), sapp::height());
    let ratio = (width as f32) / (height as f32);
    unsafe {
        // Begin recording draw commands for a frame buffer of size (width, height).
        bindings::sgp_begin(width, height);
        // Set frame buffer drawing region to (0,0,width,height).
        bindings::sgp_viewport(0, 0, width, height);
        // Set drawing coordinate space to (left=-ratio, right=ratio, top=1, bottom=-1).
        bindings::sgp_project(-ratio, ratio, 1.0, -1.0);
        // Clear the frame buffer.
        bindings::sgp_set_color(0.1, 0.1, 0.1, 1.0);
        bindings::sgp_clear();
        // Draw an animated rectangle that rotates and changes its colors.
        let time = sapp::frame_count() as f32 * sapp::frame_duration() as f32;
        let r = time.sin() * 0.5 + 0.5;
        let g = time.cos() * 0.5 + 0.5;
        bindings::sgp_set_color(r, g, 0.3, 1.0);
        bindings::sgp_rotate_at(time, 0.0, 0.0);
        bindings::sgp_draw_filled_rect(-0.5, -0.5, 1.0, 1.0);
    }
     // Begin a render pass.
    sg::begin_default_pass(&state.pass_action, width, height);
    unsafe { 
        // Dispatch all draw commands to Sokol GFX.
        bindings::sgp_flush(); 
        // Finish a draw command queue, clearing it.
        bindings::sgp_end();
    }
    // End render pass.
    sg::end_pass();
    // Commit Sokol render.
    sg::commit();
}

extern "C" fn cleanup() {
    unsafe { bindings::sgp_shutdown(); }
    sg::shutdown()
}

pub fn sokol() {
    let window_title = b"Rectangle (Sokol GP)\0".as_ptr() as _;

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