use crate::SYSTEMS;

extern "C" {
    pub fn emscripten_set_main_loop_arg(
        f: unsafe extern "C" fn(*mut std::ffi::c_void),
        arg: *mut std::ffi::c_void,
        fps: i32,
        sim_infinite_loop: i32,
    );
    pub fn emscripten_cancel_main_loop();
}

unsafe extern "C" fn packaged_main_loop(_parg: *mut std::ffi::c_void) {
    if let Err(_) = main_loop() {
       emscripten_cancel_main_loop();
    }
}

fn main_loop() -> Result<(), String> {
    use toxoid_sdl::event::Event;
    use toxoid_sdl::keyboard::Keycode;
    use toxoid_sdl::pixels::Color;

    toxoid_sdl::SDL_CONTEXT.with(|ctx| {
        let mut event_pump = ctx.borrow_mut().event_pump().unwrap();
        toxoid_sdl::CANVAS.with(|canvas_ref| {
            let mut canvas = canvas_ref.borrow_mut();

            canvas.set_draw_color(Color::RGB(64, 64, 80));
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => {
                        if keycode == Keycode::Left {
                            println!("Left");
                        }
                        if keycode == Keycode::Right {
                            println!("Right");
                        }
                        if keycode == Keycode::Up {
                            println!("Up");
                        }
                        if keycode == Keycode::Down {
                            println!("Down");
                        }
                    }
                    _ => {}
                }
            }

            canvas.present();
        })
    });
    
    SYSTEMS.with(|systems| {
        let mut systems = systems.borrow_mut();
        for system in systems.iter_mut() {
            let system = &mut *system;
            let query = &mut system.query;
            (system.update_fn)(query);
        }
    });
    

    Ok(())
}

pub fn start_loop() {
    unsafe {
        emscripten_set_main_loop_arg(
            packaged_main_loop,
            std::ptr::null_mut(),
            -1,
            0,
        );
    }
}