use toxoid_sdl::pixels::Color;
use crate::SYSTEMS;

pub fn main_loop() -> Result<(), String> {
    SYSTEMS.with(|systems| {
        let mut systems = systems.borrow_mut();
        for system in systems.iter_mut() {
            let system = &mut *system;
            let query = &mut system.query;
            (system.update_fn)(query);
        }
    });
    toxoid_sdl::CANVAS.with(|canvas_ref| {
        let mut canvas = canvas_ref.borrow_mut();
        canvas.set_draw_color(Color::RGB(64, 64, 80));
        canvas.clear();
        canvas.present();
    }); 
    Ok(())
}