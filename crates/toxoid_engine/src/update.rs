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

    // End render pass
    let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
    renderer_2d.end();
}

