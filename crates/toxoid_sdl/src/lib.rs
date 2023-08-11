extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

struct GameState {
    i: u8,
}

impl GameState {
    fn new() -> Self {
        Self { i: 0 }
    }
}

struct GameLoopArg<'a> {
    sdl_context: &'a Sdl,
    canvas: &'a mut WindowCanvas,
    state: &'a mut GameState,
}

fn main_loop(
    sdl_context: &Sdl,
    canvas: &mut WindowCanvas,
    state: &mut GameState,
) -> Result<(), String> {
    let mut event_pump = sdl_context.event_pump().unwrap();

    state.i = (state.i + 1) % 255;
    canvas.set_draw_color(Color::RGB(64, 64, state.i));
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
    Ok(())
}



pub fn create_sdl_loop() {
    // println!("Hello SDL!");

    // let sdl_context = Box::new(sdl2::init().unwrap());
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("Toxoid Engine", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    // let mut canvas = Box::new(window.into_canvas().software().build().unwrap());
    // let mut state = Box::new(GameState::new());

    // let mut arg = Box::new(GameLoopArg {
    //     sdl_context: &*sdl_context,
    //     canvas: &mut *canvas,
    //     state: &mut *state,
    // });

    // unsafe {
    //     emscripten_set_main_loop_arg(
    //         packaged_main_loop,
    //         &mut *arg as *mut _ as *mut std::ffi::c_void,
    //         -1,
    //         0,
    //     );
    // }
    // std::mem::forget(arg);
    // std::mem::forget(sdl_context);
    // std::mem::forget(canvas);
    // std::mem::forget(state);
}

// #[cfg(target_os = "emscripten")]
// #[cfg(not(target_os = "emscripten"))]
// fn create_sdl(width: u32, height: u32) -> Result<(), String> {
//     println!("Hello low_sdl!");

//     let sdl_context = Box::new(sdl2::init()?);
//     let video_subsystem = sdl_context.video()?;
//     let window = video_subsystem
//         .window("Toxoid Game", width, height)
//         .position_centered()
//         .build()
//         .unwrap();
//     let mut canvas = Box::new(window.into_canvas().software().build().unwrap());

//     Ok(())
// }
