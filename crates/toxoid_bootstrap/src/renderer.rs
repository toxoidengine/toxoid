
use toxoid_sokol::{SokolRenderer2D, sokol::app::frame_duration};
use toxoid_render::Renderer2D;

// #[no_mangle]
// extern "C" fn sokol_init() {
//     // Initialize Sokol
//     toxoid_sokol::sokol_init();
// }

#[no_mangle]
extern "C" fn sokol_frame() {
    // Frame update code for Sokol
    // Update fetch requests
    unsafe { toxoid_sokol::bindings::sfetch_dowork() };
    // Begin Sokol renderer
    SokolRenderer2D::begin();
    // Progress host
    let delta_time = frame_duration();
    toxoid_host::toxoid_progress(delta_time as f32);
    // End Sokol renderer
    SokolRenderer2D::end();
}

pub fn init(init_host: extern "C" fn()) {
    // toxoid_sokol::init(sokol_init, sokol_frame, crate::input::sokol_event);
    toxoid_sokol::init(crate::init_bootstrap, sokol_frame, crate::input::sokol_event, init_host as *mut core::ffi::c_void);
}