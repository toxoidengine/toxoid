#![allow(improper_ctypes_definitions)]
// mod systems;
// mod components;
pub use toxoid_ffi::*;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

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

    let backend = sg::query_backend();
    match &backend {
        sg::Backend::Wgpu => {
            println!("Using Wgpu Backend!");
        },
        sg::Backend::Glcore33 | sg::Backend::Gles3 => {
            println!("Using GL Backend!");
            println!("Specifically the {:?} backend!", backend);
        },
        sg::Backend::D3d11 => {
            println!("Using D3d11 Backend!");
        },
        sg::Backend::MetalIos | sg::Backend::MetalMacos | sg::Backend::MetalSimulator => {
            println!("Using Metal Backend!");
            println!("Specifically the {:?} backend!", backend);
        },
        sg::Backend::Dummy => {
            println!("Using Dummy Backend!");
        },
    }
}

extern "C" fn frame() {
    let state = unsafe { &mut STATE };

    let g = state.pass_action.colors[0].clear_value.g + 0.01;
    state.pass_action.colors[0].clear_value.g = if g > 1.0 { 0.0 } else { g };

    let (width, height) = (sapp::width(), sapp::height());

    sg::begin_default_pass(&state.pass_action, width, height);
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup() {
    sg::shutdown()
}


fn main() {
    // Initialize Toxoid ECS initializers + default components + default sy stems.
    toxoid_ffi::init();
    
    // TODO: Move to dynamically linked lib
    // systems::init();

    // components::init();

    let window_title = b"clear\0".as_ptr() as _;

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

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
    
    // Start update loop / game loop / render loop.
    // toxoid_ffi::start();
}

#[no_mangle]
pub unsafe extern "C" fn print_hello() {
    println!("HELLO WORLD 123!")
}
