use toxoid_api::{Query, System};
use toxoid_sdl::event::Event;
use toxoid_sdl::keyboard::Keycode;
use crate::{toxoid_add_system, KeyboardInput};

pub fn input_system_fn(query: &mut Query) {
    toxoid_sdl::SDL_CONTEXT.with(|ctx| {
        let query_iter = query.iter();
        while query_iter.next() {
            let entities = query_iter.entities();
            let entity = entities.get(0);
            if entity.is_some() {
                // TODO: Make this ECS Singleton later
                let mut keyboard_input = entity.unwrap().get::<crate::KeyboardInput>();
                let mut event_pump = ctx.borrow_mut().event_pump().unwrap();
                for event in event_pump.poll_iter() {
                    match event {
                        Event::KeyDown {
                            keycode: Some(keycode),
                            ..
                        } => {

                            if keycode == Keycode::Left {
                                keyboard_input.set_left(true);
                                println!("LEFT");
                            }
                            if keycode == Keycode::Right {
                                keyboard_input.set_right(true);
                            }
                            if keycode == Keycode::Up {
                                keyboard_input.set_up(true);
                            }
                            if keycode == Keycode::Down {
                                keyboard_input.set_down(true);
                            }
                        },
                        Event::KeyUp {
                            keycode: Some(keycode),
                            ..
                        } => {
                            if keycode == Keycode::Left {
                                keyboard_input.set_left(false);
                            }
                            if keycode == Keycode::Right {
                                keyboard_input.set_right(false);
                            }
                            if keycode == Keycode::Up {
                                keyboard_input.set_up(false);
                            }
                            if keycode == Keycode::Down {
                                keyboard_input.set_down(false);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    });
}

// toxoid_sdl::CANVAS.with(|canvas_ref| {
//     let mut canvas = canvas_ref.borrow_mut();
//     canvas.set_draw_color(Color::RGB(64, 64, 80));
//     canvas.clear();
//     canvas.present();
// }); 

pub fn init() {
    let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    unsafe {
        toxoid_add_system(input_system);
    }
}