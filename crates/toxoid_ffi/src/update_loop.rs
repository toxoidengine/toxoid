use crate::SYSTEMS;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use toxoid_api::Query;
use toxoid_sdl::event::Event;
use toxoid_sdl::keyboard::Keycode;
use toxoid_sdl::pixels::Color;
use toxoid_sdl::rect::Rect;

thread_local! {
    pub static QUERY: RefCell<Query> = RefCell::new(toxoid_api::ecs::Query::new::<(crate::KeyboardInput,)>());
}

pub fn input_loop(ctx: &RefCell<toxoid_sdl::sdl2::Sdl>) {
    QUERY.with(|query_cell| {
        let mut query = query_cell.borrow_mut();
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

pub fn main_loop() -> Result<(), String> {
    toxoid_sdl::SDL_CONTEXT.with(|ctx| {
        input_loop(ctx);
    });

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