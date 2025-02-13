use toxoid_sokol::sokol::app::{EventType, Event, Keycode};
use toxoid_sokol::sokol::app as sapp;
use toxoid_api::*;

fn clamp_window_size(width: u32, height: u32, game_config: &GameConfig) -> (u32, u32) {
    let min_width = game_config.get_min_window_width();
    let min_height = game_config.get_min_window_height();
    
    // Maintain aspect ratio while enforcing minimum size
    let aspect_ratio = min_width as f32 / min_height as f32;
    let current_ratio = width as f32 / height as f32;
    
    if width < min_width || height < min_height {
        if current_ratio > aspect_ratio {
            // Too wide, scale based on height
            let new_height = min_height;
            let new_width = (new_height as f32 * current_ratio) as u32;
            (new_width.max(min_width), new_height)
        } else {
            // Too tall, scale based on width
            let new_width = min_width;
            let new_height = (new_width as f32 / current_ratio) as u32;
            (new_width, new_height.max(min_height))
        }
    } else {
        (width, height)
    }
}

fn key_up(key_code: Keycode) {
    let keyboard_input = World::get_singleton::<KeyboardInput>();
    if key_code == Keycode::Up {
        keyboard_input.set_up(false);
    }
    if key_code == Keycode::Down {
        keyboard_input.set_down(false);
    }
    if key_code == Keycode::Left {
        keyboard_input.set_left(false);
    }
    if key_code == Keycode::Right {
        keyboard_input.set_right(false);
    }
}

fn key_down(key_code: Keycode) {
    let keyboard_input = World::get_singleton::<KeyboardInput>();
    if key_code == Keycode::Up {
        keyboard_input.set_up(true);
    }
    if key_code == Keycode::Down {
        keyboard_input.set_down(true);
    }
    if key_code == Keycode::Left {
        keyboard_input.set_left(true);
    }
    if key_code == Keycode::Right {
        keyboard_input.set_right(true);
    }
}

#[no_mangle]
pub extern "C" fn sokol_event(event: *const Event) {
    let event = unsafe { *event };
    match event._type {
        EventType::KeyDown => {
            key_down(event.key_code);
        },
        EventType::KeyUp => {
            key_up(event.key_code);
        },
        EventType::Resized => {
            let game_config = World::get_singleton::<GameConfig>();
            let (window_width, window_height) = (sapp::width(), sapp::height());
            
            // Clamp to minimum size while maintaining aspect ratio
            let (clamped_width, clamped_height) = clamp_window_size(
                window_width as u32, 
                window_height as u32,
                &game_config
            );
            
            game_config.set_window_width(clamped_width);
            game_config.set_window_height(clamped_height);
            
            if clamped_width != window_width as u32 || clamped_height != window_height as u32 {
                println!("Window size clamped to: {}x{} (from: {}x{})", 
                    clamped_width, clamped_height, window_width, window_height);
            }
        },
        // EventType::MouseDown => {
        //     println!("Mouse down: {:?}", event.mouse_button);
        // },
        // EventType::MouseUp => {
        //     println!("Mouse up: {:?}", event.mouse_button);
        // },
        // EventType::MouseMove => {
        //     println!("Mouse move: {:?}", event.mouse_pos);
        // },
        // EventType::MouseWheel => {
        //     println!("Mouse wheel: {:?}", event.mouse_wheel);
        // },
        _ => {}
    }

    // #[cfg(feature = "imgui")]
    unsafe { toxoid_sokol::bindings::simgui_handle_event(&event as *const _ as *const _) };
}