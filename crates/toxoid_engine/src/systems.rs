use toxoid_api::{System, Query, World};
use toxoid_api::components::*;
use toxoid_render::Renderer2D;
use std::sync::Mutex;

pub static mut GAMEPLAY_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());
pub static mut RENDER_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());

#[no_mangle]
pub unsafe extern "C" fn toxoid_add_system(
    system: toxoid_api::System
) {
    let render_systems = unsafe { &mut *RENDER_SYSTEMS.lock().unwrap() };
    render_systems.push(system);
}

pub fn render_rect_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter()
            .for_each(|entity| {
                let pos = entity.get::<Position>();
                let size = entity.get::<Size>();
                let color = entity.get::<Color>();
                
                // Draw Rect
                toxoid_sokol::SokolRenderer2D::draw_filled_rect(pos, size, color);
            });
    }
}

use toxoid_ffi::emscripten::{EmBool, EmscriptenKeyboardEvent};
unsafe extern "C" fn keydown_cb(
    _event_type:  core::ffi::c_int, 
    key_event: *const EmscriptenKeyboardEvent, 
    _user_data: *mut core::ffi::c_void
) -> EmBool {
    let key = unsafe { (*key_event).keyCode };
    println!("Key: {}", key);
    return 0;
}

pub fn init() {
    // let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    let render_rect_system = System::new::<(Size, Renderable, Color, Position)>(render_rect_system);
    World::add_system(render_rect_system);

    let canvas_id = std::ffi::CString::new("canvas").unwrap();
    unsafe {
        let result = toxoid_ffi::emscripten::toxoid_set_keydown_callback(
            canvas_id.as_ptr() as *const core::ffi::c_char, 
            std::ptr::null_mut(), 
            1, 
            keydown_cb
        );
        println!("Result: {:?}", result);
    }
}

// use crate::components::{KeyboardInput, Size, Renderable, Color, Position};

// pub fn input_system_fn(query: &mut Query) {
//     use toxoid_sdl::event::Event;
//     use toxoid_sdl::keyboard::Keycode;
//     use crate::components::KeyboardInput;
//     toxoid_sdl::SDL_CONTEXT.with(|ctx| {
//         let query_iter = query.iter();
//         while query_iter.next() {
//             let entities = query_iter.entities();
//             let entity = entities.get(0);
//             if entity.is_some() {
//                 // TODO: Make this ECS Singleton later
//                 let mut keyboard_input = entity.unwrap().get::<KeyboardInput>();
//                 let sdl_context = ctx.borrow_mut();
//                 let mut event_pump = sdl_context.event_pump().unwrap();
//                 for event in event_pump.poll_iter() {
//                     match event {
//                         Event::KeyDown {
//                             keycode: Some(keycode),
//                             ..
//                         } => {
//                             if keycode == Keycode::Left {
//                                 keyboard_input.set_left(true);
//                             }
//                             if keycode == Keycode::Right {
//                                 keyboard_input.set_right(true);
                                
//                             }
//                             if keycode == Keycode::Up {
//                                 keyboard_input.set_up(true);
//                             }
//                             if keycode == Keycode::Down {
//                                 keyboard_input.set_down(true);
//                             }
//                         },
//                         Event::KeyUp {
//                             keycode: Some(keycode),
//                             ..
//                         } => {
//                             if keycode == Keycode::Left {
//                                 keyboard_input.set_left(false);
//                             }
//                             if keycode == Keycode::Right {
//                                 keyboard_input.set_right(false);
//                             }
//                             if keycode == Keycode::Up {
//                                 keyboard_input.set_up(false);
//                             }
//                             if keycode == Keycode::Down {
//                                 keyboard_input.set_down(false);
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     });
// }