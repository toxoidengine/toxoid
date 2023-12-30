// use std::cell::RefCell;
// use std::time::SystemTime;

// thread_local! {
//     pub static DELTA: RefCell<f32> = RefCell::new(0.0);
//     pub static CURRENT: RefCell<f32> = RefCell::new(current_time_as_secs());
//     pub static DELTA_MULTIPLIER: RefCell<f32> = RefCell::new(0.0);
// }

// fn current_time_as_secs() -> f32 {
//     SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_secs_f32()
// }

// pub fn main_loop() -> Result<(), String> {
//     // Calculate delta time
//     let now = current_time_as_secs();
//     DELTA.with(|delta| {
//         CURRENT.with(|current| {
//             DELTA_MULTIPLIER.with(|delta_multiplier| {
//                 let mut delta = delta.borrow_mut();
//                 let mut current_val = current.borrow_mut();
//                 let mut delta_multiplier_val = delta_multiplier.borrow_mut();

//                 *delta = now - *current_val;
//                 *current_val = now;

//                 // Safety check to avoid dividing by zero.
//                 if *delta != 0.0 {
//                     *delta_multiplier_val = 16.0 / *delta;
//                 } else {
//                     *delta_multiplier_val = 0.0;
//                 }
//             });
//         });
//     });
    
//     use toxoid_sdl::pixels::Color;
//     // Clear canvas
//     toxoid_sdl::CANVAS.with(|canvas_ref| {
//         let mut canvas = canvas_ref.borrow_mut();
//         // Clear with the color (64, 64, 80)
//         canvas.set_draw_color(Color::RGB(64, 64, 80));
//         canvas.clear();
//     }); 

//     use crate::SYSTEMS;
//     SYSTEMS.with(|systems| {
//         let mut systems = systems.borrow_mut();
//         for system in systems.iter_mut() {
//             let system = &mut *system;
//             let query = &mut system.query;
//             (system.update_fn)(query);
//         }
//     });

//     // Draw canvas
//     toxoid_sdl::CANVAS.with(|canvas_ref| {
//         let mut canvas = canvas_ref.borrow_mut();
//         // Present the canvas
//         canvas.present();
//     }); 
    
//     DELTA.with(|delta| {
//         let delta = delta.borrow();
//         unsafe { toxoid_api::toxoid_progress(*delta) };
//     });
    
//     Ok(())
// }