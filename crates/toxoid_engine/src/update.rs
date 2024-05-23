use toxoid_api::{World, SpineInstance};
#[cfg(feature = "render")]
use toxoid_render::Renderer2D;
use toxoid_api::toxoid_progress;

pub extern "C" fn game_loop(_parg: *mut std::ffi::c_void) {
    #[cfg(feature = "fetch")]
    unsafe { toxoid_sokol::bindings::sfetch_dowork() };

    // Begin render pass
    #[cfg(feature = "render")]
    toxoid_sokol::SokolRenderer2D::begin();

    // Update game
    unsafe { toxoid_progress(1.0) };

    // unsafe {
    //     use toxoid_sokol::bindings::*;
    //     let spine_instance = World::get_singleton::<SpineInstance>();
    //     let instantiated = spine_instance.get_instantiated();
    //     if instantiated {
    //         let instance = spine_instance.get_instance().ptr as *mut sspine_instance;
    //         let delta_time = sapp_frame_duration();
    //         sspine_update_instance(*instance, delta_time as f32);
    //         sspine_draw_instance_in_layer(*instance, 0);
    //         println!("Hello spine render {:?}", instance as usize);
    //         println!("Hello spine render instance {:?}", *instance);
    //     }  
    // }
    
    
    // // Get gameplay systems
    // let gameplay_systems = unsafe { &mut *GAMEPLAY_SYSTEMS.lock().unwrap() };
    // // Update gameplay systems
    // for system in gameplay_systems.iter_mut() {
    //     let system = &mut *system;
    //     let query = &mut system.query;
    //     (system.update_fn)(query);
    // }

    // #[cfg(feature = "imgui")]
    // unsafe {
    //     let mut desc: toxoid_sokol::bindings::simgui_frame_desc_t = core::mem::MaybeUninit::zeroed().assume_init();
    //     desc.width = 1280;
    //     desc.height = 720;
    //     desc.delta_time = 0.0;
    //     desc.dpi_scale = 1.0;
    //     toxoid_sokol::bindings::simgui_new_frame(&desc);
    //     let mut test = false;
    //     toxoid_sokol::bindings::igSetNextWindowSize(
    //         toxoid_sokol::bindings::ImVec2 { x: 200.0, y: 200.0 },
    //         toxoid_sokol::bindings::ImGuiCond__ImGuiCond_Always
    //     );
    //     toxoid_sokol::bindings::igBegin(b"Demo window\0".as_ptr() as _, &mut test as _, 0);
    //     toxoid_sokol::bindings::igButton(
    //         b"Hello!\0".as_ptr() as _,
    //         toxoid_sokol::bindings::ImVec2 { x: 0.0, y: 0.0 }
    //     );
    //     toxoid_sokol::bindings::igEnd();
    // }
    
    // End render pass
    #[cfg(feature = "render")] {
        let renderer_2d = &mut *toxoid_sokol::RENDERER_2D.lock().unwrap();
        renderer_2d.end();
    }
}

#[cfg(feature = "render")]
pub extern "C" fn render_loop() {}

