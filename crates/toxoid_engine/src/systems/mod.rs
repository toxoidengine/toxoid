use toxoid_api::*;
use toxoid_render::Renderer2D;
use toxoid_sokol::SokolSprite;
use std::sync::Mutex;

pub static mut GAMEPLAY_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());
pub static mut RENDER_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());

#[no_mangle]
pub unsafe extern "C" fn toxoid_add_system(
    system: toxoid_api::System
) {
    let render_systems = unsafe { &mut *RENDER_SYSTEMS.lock().unwrap() };
    render_systems.push(system);
}

pub fn render_rect_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter()
            .for_each(|entity| {
                let pos = entity.get::<Position>();
                let size = entity.get::<Size>();
                let color = entity.get::<Color>();
                
                // Draw Rect
                // #[cfg(feature = "sokol")]
                toxoid_sokol::SokolRenderer2D::draw_filled_rect(pos, size, color);
            });
    }
}

pub fn render_sprite_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter()
            .for_each(|entity| {
                let sprite = entity.get::<Sprite>();
                let pos = entity.get::<Position>();
                let sprite_ptr = sprite.get_sprite();
                let sprite_box = unsafe { Box::from_raw(sprite_ptr.ptr as *mut SokolSprite) };
                let sprite_trait_object: &Box<dyn toxoid_render::Sprite> = Box::leak(Box::new(sprite_box as Box<dyn toxoid_render::Sprite>));
                // Draw Sprite
                // #[cfg(feature = "sokol")]
                toxoid_sokol::SokolRenderer2D::draw_sprite(sprite_trait_object, pos.get_x() as f32, pos.get_y() as f32);
            });
    }
}

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
                
                use crate::utils::loader::*;

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
                use crate::utils::loader::*;

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

// TODO: Remove system, temporary to test multiplayer
pub fn input_system(query: &mut Query) {
    let query = query.iter();
    let mut keyboard_input = World::get_singleton::<KeyboardInput>();
    let websocket = World::get_singleton::<WebSocket>();
    while query.next() {
        let entities = query.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let mut pos = entity.get::<Position>();

                if keyboard_input.get_up() {
                    // pos.set_y(pos.get_y() - 3);
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "UP".as_ptr() as *const core::ffi::c_void, 2) };
                }
                if keyboard_input.get_down() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "DOWN".as_ptr() as *const core::ffi::c_void, 4) };
                    // pos.set_y(pos.get_y() + 3);
                }
                if keyboard_input.get_left() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "LEFT".as_ptr() as *const core::ffi::c_void, 4) };
                    // pos.set_x(pos.get_x() - 3);
                }
                if keyboard_input.get_right() {
                    unsafe { toxoid_ffi::emscripten::emscripten_websocket_send_binary(websocket.get_socket().ptr, "RIGHT".as_ptr() as *const core::ffi::c_void, 5) };
                    // pos.set_x(pos.get_x() + 3);
                }
            });
    }
}

pub fn draw_bone_animation(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                use toxoid_sokol::bindings::*;
                let spine_instance = entity.get::<SpineInstance>();
                let pos = entity.get::<Position>();
                let instance = spine_instance.get_instance().ptr as *mut sspine_instance;
                unsafe { 
                    let delta_time = sapp_frame_duration();
                    sspine_set_position(*instance, sspine_vec2 { x: pos.get_x() as f32, y: pos.get_y() as f32 });
                    // Advance the instance animation and draw the instance.
                    // Important to note here is that no actual sokol-gfx rendering happens yet,
                    // instead sokol-spine will only record vertices, indices and draw commands.
                    // Also, all sokol-spine functions can be called with invalid or 'incomplete'
                    // handles, that way we don't need to care about whether the spine objects
                    // have actually been created yet (because their data might still be loading)
                    sspine_update_instance(*instance, delta_time as f32);
                    sspine_draw_instance_in_layer(*instance, 0); 
                };
            });
    }
}

pub fn init() {
    let mut render_rect_system = System::new(render_rect_system);
    let mut load_sprite_system = System::new(load_sprite_system);
    let mut render_sprite_system = System::new(render_sprite_system);
    let mut load_bone_animation_system = System::new(load_bone_animation_system);
    let mut draw_bone_animation = System::new(draw_bone_animation);

    render_rect_system
        .with::<(Rect, Renderable, Color, Size, Position)>()
        .build();
    load_sprite_system
        .with::<(Loadable, Sprite, Size, Position)>()
        .build();
    render_sprite_system
        .with::<(Sprite, Renderable, Size, Position)>()
        .build();
    load_bone_animation_system
        .with::<(Loadable, Atlas, Skeleton, Images)>()
        .build();
    draw_bone_animation
        .with::<(Position, BoneAnimation, SpineInstance)>()
        .build();

    World::add_system(render_rect_system);
    World::add_system(load_sprite_system);
    World::add_system(render_sprite_system);
    World::add_system(load_bone_animation_system);
    World::add_system(draw_bone_animation);

    // TODO: Remove
    let mut input_system = System::new(input_system);
    input_system
        .with::<(Position, BoneAnimation, SpineInstance, Local)>()
        .build();
    World::add_system(input_system);
    

    #[cfg(target_os = "emscripten")]
    {
        let canvas_id = std::ffi::CString::new("canvas").unwrap();
        unsafe {
            let result = toxoid_set_keydown_callback(
                canvas_id.as_ptr() as *const core::ffi::c_char, 
                std::ptr::null_mut(), 
                1, 
                keydown_cb
            );
            if result != 0 {
                panic!("Error setting keydown callback");
            }
            let result = toxoid_set_keyup_callback(
                canvas_id.as_ptr() as *const core::ffi::c_char, 
                std::ptr::null_mut(), 
                1, 
                keyup_cb
            );
            if result != 0 {
                panic!("Error setting keyup callback");
            }
        }
    }  
}