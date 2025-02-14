use toxoid_sokol::sokol::app::{EventType, Event, Keycode};
use toxoid_sokol::sokol::app as sapp;
use toxoid_api::*;

fn clamp_window_size(width: u32, height: u32, game_config: &GameConfig) -> (u32, u32) {
    let base_width = game_config.get_game_width();
    let base_height = game_config.get_game_height();
    let base_aspect = base_width as f32 / base_height as f32;
    
    // Calculate scale while maintaining aspect ratio
    let scale_by_width = (width as f32 / base_width as f32).floor().max(1.0);
    let scale_by_height = (height as f32 / base_height as f32).floor().max(1.0);
    
    // Choose scale that maintains aspect ratio without stretching
    let scale = if width as f32 / height as f32 > base_aspect {
        // Window is wider - scale by height
        scale_by_height
    } else {
        // Window is taller - scale by width
        scale_by_width
    };
    
    (
        (base_width as f32 * scale) as u32,
        (base_height as f32 * scale) as u32
    )
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

fn handle_mouse_scroll(scroll_y: f32) {
    let main_camera = World::get_singleton::<MainCamera>();
    let mut camera_entity = Entity::from_id(main_camera.get_entity());
    let camera = camera_entity.get::<Camera>();
    
    let game_config = World::get_singleton::<GameConfig>();
    let zoom_speed = game_config.get_zoom_speed();
    
    // Update zoom (negative scroll_y means zoom in)
    let current_zoom = camera.get_zoom();
    let new_zoom = current_zoom * (1.0 - (-scroll_y * zoom_speed));
    
    // Clamp zoom to min/max values
    let clamped_zoom = new_zoom.clamp(camera.get_min_zoom(), camera.get_max_zoom());
    camera.set_zoom(clamped_zoom);
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
            
            // Calculate the clamped size for rendering
            let (clamped_width, clamped_height) = clamp_window_size(
                window_width as u32, 
                window_height as u32,
                &game_config
            );
            
            // Update game config with the clamped dimensions for rendering
            game_config.set_window_width(clamped_width);
            game_config.set_window_height(clamped_height);
            
            if clamped_width != window_width as u32 || clamped_height != window_height as u32 {
                println!("Render size clamped to: {}x{} (window: {}x{})", 
                    clamped_width, clamped_height, window_width, window_height);
            }
        },
        EventType::MouseScroll => {
            handle_mouse_scroll(event.scroll_y);
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