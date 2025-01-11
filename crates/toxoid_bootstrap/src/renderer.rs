
use toxoid_sokol::{SokolRenderer2D, sokol::app::frame_duration};
use toxoid_render::Renderer2D;

#[no_mangle]
extern "C" fn sokol_init() {
    // Initialization code for Sokol
    // println!("Sokol initialized");
    toxoid_sokol::sokol_init();
}

#[no_mangle]
extern "C" fn sokol_frame() {
    // Frame update code for Sokol
    SokolRenderer2D::begin();
    let delta_time = frame_duration();
    toxoid_host::toxoid_progress(delta_time as f32);
    SokolRenderer2D::end();
}

// #[no_mangle]
// extern "C" fn sokol_cleanup() {
//     // Cleanup code for Sokol
//     println!("Sokol cleanup");
// }

pub fn init() {
    toxoid_sokol::init(sokol_init, sokol_frame, crate::input::sokol_event);
}