pub fn init() {
    // Keyboard Input
    let mut keyboard_entity = toxoid_ffi::toxoid_api::Entity::new();
    keyboard_entity.add::<toxoid_ffi::components::KeyboardInput>();
}