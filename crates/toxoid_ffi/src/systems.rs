use core::alloc::GlobalAlloc;
use toxoid_api::{Query, ALLOCATOR, Entity, IsComponent};

pub fn input_system_fn(query: &mut Query) {
    use toxoid_sdl::event::Event;
    use toxoid_sdl::keyboard::Keycode;
    use crate::components::KeyboardInput;
    toxoid_sdl::SDL_CONTEXT.with(|ctx| {
        let query_iter = query.iter();
        while query_iter.next() {
            let entities = query_iter.entities();
            let entity = entities.get(0);
            if entity.is_some() {
                // TODO: Make this ECS Singleton later
                let mut keyboard_input = entity.unwrap().get::<KeyboardInput>();
                let sdl_context = ctx.borrow_mut();
                let mut event_pump = sdl_context.event_pump().unwrap();
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

pub fn render_rect_system_fn(query: &mut Query) {
    use crate::components::{Rect, Color, Position};
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter()
            .for_each(|entity| {
                let rect = entity.get::<Rect>();
                let pos = entity.get::<Position>();
                let color = entity.get::<Color>();
                // Draw Rect
                toxoid_sdl::CANVAS.with(|canvas_ref| {
                    let mut canvas = canvas_ref.borrow_mut();
                    canvas.set_draw_color(toxoid_sdl::pixels::Color::RGB(color.get_r(), color.get_g(), color.get_b()));
                    let rect = toxoid_sdl::rect::Rect::new(pos.get_x() as i32, pos.get_y() as i32, rect.get_width(), rect.get_height());
                    canvas.fill_rect(rect).unwrap();
                }); 
            });
    }
}

pub fn init() {
    use toxoid_api::System;
    use crate::ecs::toxoid_add_system;
    use crate::components::{KeyboardInput, Rect, Renderable, Color, Position};
    
    let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    let render_rect_system = System::new::<(Rect, Renderable, Color, Position)>(render_rect_system_fn);
    unsafe {
        toxoid_add_system(input_system);
        toxoid_add_system(render_rect_system);
    }
}