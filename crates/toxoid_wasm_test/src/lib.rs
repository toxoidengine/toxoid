pub type ecs_id_t = u64;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;
pub type c_void = i32;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_u64(v: u64);
    pub fn toxoid_print_f32(v: f32);
    pub fn toxoid_print_f64(v: f64);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_net_add_event(
        event_name: &str,
        callback: *mut c_void
    );
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}

extern "C" fn test_callback() {
    print_string("Hello World 12345");
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    print_string("Hello World 123");
    toxoid_print_i32(4200);
    toxoid_print_u64(888);
    toxoid_print_f64(777.2);
    print_string("Entity ID: ");
    toxoid_print_u64(toxoid_entity_create());
    toxoid_net_add_event("test", test_callback as *mut c_void);
    print_string("Hello World 12345");
}