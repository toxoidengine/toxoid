use toxoid_render::Renderer2D;
// pub mod entities;
// pub mod systems;
// pub mod audio;
// pub mod update_loop;
// pub mod utils;

/*
    Calculates the width and height of the image based on the aspect ratio of the window and the image. If the window's aspect ratio is greater than or equal to the image's aspect ratio, it sets the width to the window's width and calculates the height based on the image's aspect ratio. If the window's aspect ratio is less than the image's aspect ratio, it sets the height to the window's height and calculates the width based on the image's aspect ratio.
*/
extern "C" fn frame_cb() {
    // // The object will scale based on the window width, ensuring it always fits within the window without distortion.
    // let scale_factor = window_width as f32 / RESOLUTION_WIDTH as f32;
    // let width = 50.0 * scale_factor;
    // let height = 50.0 * scale_factor;

    // let x = 0.0 * scale_factor; // Change this to the x position of sprite
    // let y = 0.0 * scale_factor; // Change this to the y position of sprite

    // let x_sprite = 100. * scale_factor; // Change this to the x position of sprite
    // let y_sprite = 100. * scale_factor; // Change this to the y position of sprite
    toxoid_sokol::SokolRenderer2D::begin();
    toxoid_sokol::SokolRenderer2D::draw_filled_rect(
        toxoid_render::Rect { x: 0 as i32, y: 0 as i32, width: 100 as i32, height: 100 as i32 }, 
        toxoid_render::Color { r: 0, g: 255, b: 0, a: 255 }
    );
    // if let Some(render_target) = &mut RENDER_TARGET {
    //     sgp_reset_color();
    //     sgp_set_blend_mode(sgp_blend_mode_SGP_BLENDMODE_BLEND);
    //     let target = render_target.as_any().downcast_ref::<render_2d::SokolRenderTarget>().unwrap();
    //     SokolRenderer2D::draw_sprite(&target.sprite, x_sprite, y_sprite, scale_factor);
    //     // sgp_reset_blend_mode();
    // }  
        unsafe {
            let renderer_2d = toxoid_sokol::RENDERER_2D
                .lock()
                .unwrap();
            let renderer_2d = renderer_2d.as_ref().unwrap();
            renderer_2d.end();
        }
}

pub fn init() {
    // Initialize FLECS ECS.
    toxoid_ffi::flecs_core::init();

    // Initialize renderer
    // TODO: Renderer backend feature flags
    #[cfg(target_os = "emscripten")]
    toxoid_sokol::init(frame_cb);
    #[cfg(not(target_os = "emscripten"))]
    toxoid_sokol::init(frame_cb);

    // Initialize default components.
    toxoid_api::components::init();

    // Initialize default entities.
    // TODO: Turn into prefabs
    // entities::init();
    // Initialize default engine systems. Such as rendering, input, etc.
    // systems::init();
    
    // Audio test
    // audio::init();
}

// Start update loop / game loop / render loop.
pub fn start() {
    #[cfg(target_os = "emscripten")]
    // emscripten::start_loop();
    #[cfg(not(target_os = "emscripten"))]
    loop {}
}