use toxoid_sokol::sokol::app::{Event, EventType, Keycode};
use toxoid_api::*;

fn key_up(key_code: Keycode) {
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();
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
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();
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
}
