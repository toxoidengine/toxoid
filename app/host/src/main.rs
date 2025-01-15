fn main() {
    toxoid_bootstrap::init_ecs();
    // #[cfg(feature = "static-linking")]
    guest::init();
    toxoid_bootstrap::init_renderer();
}
