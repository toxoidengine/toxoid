#![allow(improper_ctypes_definitions)]
mod allocator;
mod ecs;
mod local_ecs;

use local_ecs::*;
use toxoid_ffi::*;
use toxoid_ffi_macro::component;

extern "C" {
    pub fn app_main();
}

component! {
    TestComponent {
        x: u32,
        y: u32,
    }
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    let pos_id = TestComponent::register();

    // Create a new entity.
    let mut player = Entity::new();
    // Add the component to the entity.
    player.add(pos_id);

    let mut pos_component = player.get_component::<TestComponent>();
    pos_component.set_x(95);
    pos_component.set_y(421);

    println!("Player X: {:?}", pos_component.get_x());

    // Initialize SDL2
    toxoid_sdl::create_sdl_loop();
}

fn main() {
    flecs_core::init();
    ecs::init();
    println!("Toxoid Engine Initiated.");
}
