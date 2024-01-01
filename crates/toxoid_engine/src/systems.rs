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

#[cfg(target_os = "emscripten")]
pub fn load_sprite_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let pos = entity.get::<Position>();
                let size = entity.get::<Size>();
                let mut sprite = entity.get::<Sprite>();

                println!("Sprite loaded!");
                entity.remove::<Loadable>();
                
                use core::ffi::CStr;
                use toxoid_ffi::emscripten::*;
                use crate::utils::*;

                let raw_ptr = sprite.get_filename().ptr;
                let c_str: &CStr =  unsafe { CStr::from_ptr(raw_ptr as *const i8) };
                let filename: &str = c_str.to_str().unwrap();
        
                // Create fetch attributes
                let mut attr: emscripten_fetch_attr_t = unsafe { std::mem::zeroed() };
                unsafe { emscripten_fetch_attr_init(&mut attr) };
                attr.attributes = EMSCRIPTEN_FETCH_LOAD_TO_MEMORY;
                
                // Callbacks
                attr.onsuccess = Some(download_succeeded);
                attr.onerror = Some(download_failed);
                
                // Convert filename to CString
                let filename_cstring = std::ffi::CString::new(filename).unwrap();
        
                // Fetch file
                unsafe { emscripten_fetch(&attr, filename_cstring.as_ptr()) };
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

pub fn init() {
    let mut render_rect_system = System::new(render_rect_system);
    let mut load_sprite_system = System::new(load_sprite_system);
    let mut render_sprite_system = System::new(render_sprite_system);

    render_rect_system
        .with::<(Rect, Renderable, Color, Size, Position)>()
        .build();
    load_sprite_system
        .with::<(Loadable, Sprite, Size, Position)>()
        .build();
    render_sprite_system
        .with::<(Sprite, Renderable, Size, Position)>()
        .build();

    World::add_system(render_rect_system);
    World::add_system(load_sprite_system);
    World::add_system(render_sprite_system);

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