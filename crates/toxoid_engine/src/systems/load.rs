use toxoid_api::*;

pub fn load_sprite_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let sprite = entity.get::<Sprite>();

                println!("Sprite loading!");
                entity.remove::<Loadable>();
                
                use crate::utils::load::*;

                let filename = sprite.get_filename();
                fetch(filename, entity as *mut Entity as *mut core::ffi::c_void, image_load_success, image_load_fail);
            });
    }
}

pub fn load_bone_animation_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                // println!("Animation loading!");
                use crate::utils::load::*;

                entity.remove::<Loadable>();

                let skeleton = entity.get::<Skeleton>();
                let atlas = entity.get::<Atlas>();
                let skeleton_filename = skeleton.get_skeleton_filename();
                let atlas_filename = atlas.get_atlas_filename();
                
                fetch(skeleton_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
                fetch(atlas_filename, entity as *mut Entity as *mut core::ffi::c_void, animation_load_success, animation_load_failed);
            });
    }
} 

// emscripten cfg
#[cfg(target_os = "emscripten")]
use toxoid_ffi::emscripten::{EmBool, EmscriptenKeyboardEvent, toxoid_set_keydown_callback, toxoid_set_keyup_callback};

#[cfg(target_os = "emscripten")]
unsafe extern "C" fn keydown_cb(
    _event_type:  core::ffi::c_int, 
    key_event: *const EmscriptenKeyboardEvent, 
    _user_data: *mut core::ffi::c_void
) -> EmBool {
    let key = unsafe { (*key_event).keyCode };
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();

    if key == KeyCode::Up as u32 {
        keyboard_input.set_up(true);
    }
    if key == KeyCode::Down as u32 {
        keyboard_input.set_down(true);
    }
    if key == KeyCode::Left as u32 {
        keyboard_input.set_left(true);
    }
    if key == KeyCode::Right as u32 {
        keyboard_input.set_right(true);
    }
    return 0;
}

#[cfg(target_os = "emscripten")]
unsafe extern "C" fn keyup_cb(
    _event_type:  core::ffi::c_int, 
    key_event: *const EmscriptenKeyboardEvent, 
    _user_data: *mut core::ffi::c_void
) -> EmBool {
    let key = unsafe { (*key_event).keyCode };
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();
    

    if key == KeyCode::Up as u32 {
       
        keyboard_input.set_up(false);
    }
    if key == KeyCode::Down as u32 {
        
        keyboard_input.set_down(false);
    }
    if key == KeyCode::Left as u32 {
       
        keyboard_input.set_left(false);
    }
    if key == KeyCode::Right as u32 {
        
        keyboard_input.set_right(false);
    }
    return 0;
}