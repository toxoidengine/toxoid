pub extern crate sdl2;

pub use sdl2::*;
use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::VideoSubsystem;
use std::cell::RefCell;

thread_local! {
    pub static SDL_CONTEXT: RefCell<Sdl> = {
        let sdl = sdl2::init().unwrap();
        RefCell::new(sdl)
    };
    pub static VIDEO_SUBSYSTEM: RefCell<VideoSubsystem> = {
        let video = SDL_CONTEXT.with(|sdl| sdl.borrow().video().unwrap());
        RefCell::new(video)
    };
    pub static CANVAS: RefCell<WindowCanvas> = {
        let video = VIDEO_SUBSYSTEM.with(|video| video.borrow().clone());
        let window = video
            .window("Toxoid Engine", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().software().build().unwrap();
        RefCell::new(canvas)
    };
}