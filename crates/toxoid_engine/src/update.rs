use crate::systems::{GAMEPLAY_SYSTEMS, RENDER_SYSTEMS};
use toxoid_render::Renderer2D;

pub extern "C" fn gameplay_loop(_parg: *mut std::ffi::c_void) {
    // Get gameplay systems
    let gameplay_systems = unsafe { &mut *GAMEPLAY_SYSTEMS.lock().unwrap() };
    // Update gameplay systems
    for system in gameplay_systems.iter_mut() {
        let system = &mut *system;
        let query = &mut system.query;
        (system.update_fn)(query);
    }
}

pub extern "C" fn render_loop() {
    // Begin render pass
    toxoid_sokol::SokolRenderer2D::begin();

    // Get render systems
    let render_systems = unsafe { &mut *RENDER_SYSTEMS.lock().unwrap() };
    // Update render systems
    for system in render_systems.iter_mut() {
        let system = &mut *system;
        let query = &mut system.query;
        (system.update_fn)(query);
    }

    unsafe {
        // let mut test = false;
        // toxoid_sokol::imgui_sys::igBegin(b"Demo window\0".as_ptr() as _, &mut test as _, 0);
        // toxoid_sokol::imgui_sys::igButton(
        //     b"Hello!\0".as_ptr() as _,
        //     toxoid_sokol::imgui_sys::ImVec2 { x: 0.0, y: 0.0 },
        // );
        // toxoid_sokol::imgui_sys::igEnd();

        let mut desc: toxoid_sokol::bindings::simgui_frame_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
        desc.width = 1280;
        desc.height = 720;
        desc.delta_time = 0.0;
        desc.dpi_scale = 1.0;
        toxoid_sokol::bindings::simgui_new_frame(&desc);
        let mut test = false;
        toxoid_sokol::bindings::igBegin(b"Demo window\0".as_ptr() as _, &mut test as _, 0);
        toxoid_sokol::bindings::igButton(
            b"Hello!\0".as_ptr() as _,
            toxoid_sokol::bindings::ImVec2 { x: 0.0, y: 0.0 }
        );
        toxoid_sokol::bindings::igEnd();
        toxoid_sokol::bindings::simgui_render();
    }

    // End render pass
    let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
    renderer_2d.end();
}

