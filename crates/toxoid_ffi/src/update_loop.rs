pub fn main_loop() -> Result<(), String> {
    use toxoid_sdl::pixels::Color;
    // Clear canvas
    toxoid_sdl::CANVAS.with(|canvas_ref| {
        let mut canvas = canvas_ref.borrow_mut();
        // Clear with the color (64, 64, 80)
        canvas.set_draw_color(Color::RGB(64, 64, 80));
        canvas.clear();
    }); 

    use crate::SYSTEMS;
    SYSTEMS.with(|systems| {
        let mut systems = systems.borrow_mut();
        for system in systems.iter_mut() {
            let system = &mut *system;
            let query = &mut system.query;
            (system.update_fn)(query);
        }
    });

    // Draw canvas
    toxoid_sdl::CANVAS.with(|canvas_ref| {
        let mut canvas = canvas_ref.borrow_mut();
        // Present the canvas
        canvas.present();
    }); 
    // unsafe { toxoid_api::toxoid_progress(0.0) };
    Ok(())
}