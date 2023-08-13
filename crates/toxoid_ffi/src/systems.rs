use toxoid_api::Query;

use toxoid_sdl::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::borrow::Cow;
use std::path::{Path, PathBuf};

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        // ... [no changes here] ...
    }
}

pub fn input_system_fn(query: &mut Query) {
    use toxoid_sdl::event::Event;
    use toxoid_sdl::keyboard::Keycode;
    toxoid_sdl::SDL_CONTEXT.with(|ctx| {
        toxoid_sdl::AUDIO_SUBSYSTEM.with(|audio| {
            let audio_subsystem = audio.borrow();
            
            let desired_spec = AudioSpecDesired {
                freq: Some(44_100),
                channels: Some(1), // mono
                samples: None,     // default
            };

            let query_iter = query.iter();
            while query_iter.next() {
                let entities = query_iter.entities();
                let entity = entities.get(0);
                if entity.is_some() {
                    // TODO: Make this ECS Singleton later
                    let mut keyboard_input = entity.unwrap().get::<crate::KeyboardInput>();
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
                                    let wav_file: Cow<'static, Path> = match std::env::args().nth(1) {
                                        None => Cow::from(Path::new("assets/sound.wav")),
                                        Some(s) => Cow::from(PathBuf::from(s)),
                                    };
                                    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                                        let wav = AudioSpecWAV::load_wav(wav_file).expect("Could not load test WAV data");
                            
                                        let cvt = AudioCVT::new(
                                            wav.format,
                                            wav.channels,
                                            wav.freq,
                                            spec.format,
                                            spec.channels,
                                            spec.freq,
                                        )
                                        .expect("Could not convert WAV data");
                            
                                        let data = cvt.convert(wav.buffer().to_vec());
                            
                                        // initialize the audio callback
                                        Sound {
                                            data: data,
                                            volume: 0.25,
                                            pos: 0,
                                        }
                                    }).unwrap();
                                    // Start playback
                                    device.resume();
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
        
    });
}

pub fn render_rect_system_fn(query: &mut Query) {
    use toxoid_sdl::pixels::Color;
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        let entity = entities.get(0);
            if entity.is_some() {
                let player = entity.unwrap().get::<crate::Position>();
                // Draw Rect
                toxoid_sdl::CANVAS.with(|canvas_ref| {
                    let mut canvas = canvas_ref.borrow_mut();
                    // Set the rectangle color to red (255, 0, 0) and fill the rectangle
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    let rect = toxoid_sdl::rect::Rect::new(player.get_x() as i32, player.get_y() as i32, 50, 50);
                    canvas.fill_rect(rect).unwrap();

                    let rect = toxoid_sdl::rect::Rect::new(player.get_x() as i32, player.get_y() as i32, 50, 50);
                    canvas.fill_rect(rect).unwrap();
                }); 
            }
        
    }
}

pub fn init() {
    use toxoid_api::System;

    use crate::ecs::toxoid_add_system;
    use crate::components::{KeyboardInput, Position, Rect, Renderable};
    let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    unsafe {
        toxoid_add_system(input_system);
    }
    
    let render_rect_system = System::new::<(Position,)>(render_rect_system_fn);
    unsafe {
        toxoid_add_system(render_rect_system);
    }
}