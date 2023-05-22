extern crate sdl2; 

// #[cfg(target_os = "emscripten")] 
// #[cfg(not(target_os = "emscripten"))] 
fn create_sdl(width: u32, height: u32) -> Result<(), String> {
    println!("Hello low_sdl!");

    let sdl_context = Box::new(sdl2::init()?);
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Toxoid Game", width, height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = Box::new(window.into_canvas()
        .software()
        .build()
        .unwrap());

    Ok(())
}