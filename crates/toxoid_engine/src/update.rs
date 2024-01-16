use crate::systems::{GAMEPLAY_SYSTEMS, RENDER_SYSTEMS};
use toxoid_api::{World, SpineInstance};
use toxoid_render::Renderer2D;

pub extern "C" fn gameplay_loop(_parg: *mut std::ffi::c_void) {
    // unsafe { toxoid_sokol::bindings::sfetch_dowork() };
    
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

    // Advance the instance animation and draw the instance.
    // Important to note here is that no actual sokol-gfx rendering happens yet,
    // instead sokol-spine will only record vertices, indices and draw commands.
    // Also, all sokol-spine functions can be called with invalid or 'incomplete'
    // handles, that way we don't need to care about whether the spine objects
    // have actually been created yet (because their data might still be loading)
    use toxoid_sokol::bindings::*;
    unsafe {
        let spine_instance = World::get_singleton::<SpineInstance>();
        let instantiated = spine_instance.get_instantiated();
        if instantiated {
            let instance = spine_instance.get_instance().ptr as *mut sspine_instance;
            let delta_time = sapp_frame_duration();
            sspine_update_instance(*instance, delta_time as f32);
            sspine_draw_instance_in_layer(*instance, 0);
        }  
    }

    unsafe {
        let mut desc: toxoid_sokol::bindings::simgui_frame_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
        desc.width = 1280;
        desc.height = 720;
        desc.delta_time = 0.0;
        desc.dpi_scale = 1.0;
        toxoid_sokol::bindings::simgui_new_frame(&desc);
        let mut test = false;
        toxoid_sokol::bindings::igSetNextWindowSize(
            toxoid_sokol::bindings::ImVec2 { x: 200.0, y: 200.0 },
            toxoid_sokol::bindings::ImGuiCond__ImGuiCond_Always
        );
        toxoid_sokol::bindings::igBegin(b"Demo window\0".as_ptr() as _, &mut test as _, 0);
        toxoid_sokol::bindings::igButton(
            b"Hello!\0".as_ptr() as _,
            toxoid_sokol::bindings::ImVec2 { x: 0.0, y: 0.0 }
        );
        toxoid_sokol::bindings::igEnd();
    }

    // End render pass
    let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
    renderer_2d.end();
}

