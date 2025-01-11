use std::thread;

// TODO: Re-add this when we implement Flecs phases.
// For now just run the game loop with the render loop in Sokol's loop.
pub fn init() {
    // Spawn a thread to run the game loop
    std::thread::spawn(move || {
        loop {
            let delta_time = toxoid_sokol::sokol::app::frame_duration();
            toxoid_host::toxoid_progress(delta_time as f32);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });
}