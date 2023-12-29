use toxoid_api::*;
use toxoid_render::Renderer2D;
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
                let pos = entity.get::<Position>();
                let size = entity.get::<Size>();
                
                // Draw Sprite
                // toxoid_sokol::SokolRenderer2D::draw_sprite(sprite, x, y, scale_factor)
            });
    }
}

pub fn load_sprite_system(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter()
            .for_each(|entity| {
                let pos = entity.get::<Position>();
                let size = entity.get::<Size>();
                let mut sprite = entity.get::<Sprite>();
                
                // let filename = sprite.filename;
                let filename = "assets/sprites/ship.png";
                // Draw Sprite
                let mut sprite_ptr = toxoid_sokol::SokolRenderer2D::create_sprite(filename);
                sprite.set_sprite(Pointer { ptr: sprite_ptr.as_mut() as *mut _ as *mut core::ffi::c_void });
            });
    }
}

// emscripten cfg
#[cfg(target_os = "emscripten")]
use toxoid_ffi::emscripten::{EmBool, EmscriptenKeyboardEvent, toxoid_set_keydown_callback};

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

pub fn init() {
    // let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    let render_rect_system = System::new::<(Rect, Renderable, Color, Size, Position)>(render_rect_system);
    World::add_system(render_rect_system);
    // let render_sprite_system = System::new::<(Sprite, Renderable, Size, Position)>(render_rect_system);
    // World::add_system(render_rect_system);

    // let load_sprite_system = System::new::<(Sprite, Loadable, Size, Position)>(render_rect_system);
    // World::add_system(render_rect_system);

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
        }
    }   
}