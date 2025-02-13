use toxoid_sokol::sokol::app::{EventType, Event, Keycode};
use toxoid_sokol::sokol::app as sapp;
use toxoid_api::*;

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
    // Event handling code for Sokol
    // println!("Sokol event received");
    // toxoid_sokol::sokol_event(event);
    let event = unsafe { *event };
    match event._type {
        EventType::KeyDown => {
           key_down(event.key_code);
        },
        EventType::KeyUp => {
            key_up(event.key_code);
        },
        EventType::Resized => {
            // Update GameConfig with new window dimensions
            let game_config = World::get_singleton::<GameConfig>();
            let (window_width, window_height) = (sapp::width(), sapp::height());
            game_config.set_window_width(window_width as u32);
            game_config.set_window_height(window_height as u32);
            // Game resolution stays the same - only window size changes
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