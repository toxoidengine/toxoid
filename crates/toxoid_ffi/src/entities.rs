pub fn init() {
    // Keyboard Input
    let mut keyboard_entity = crate::toxoid_api::Entity::new();
    keyboard_entity.add::<crate::components::KeyboardInput>();
}