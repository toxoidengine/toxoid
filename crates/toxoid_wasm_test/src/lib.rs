pub type ecs_id_t = u64;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;
pub type c_void = i32;

#[repr(C)]
pub struct MessageEntity {
    pub id: u64,
    pub event: &'static str,
    pub components: &'static [c_void]
}

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_u64(v: u64);
    pub fn toxoid_print_f32(v: f32);
    pub fn toxoid_print_f64(v: f64);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_net_add_event(
        event_name: &str,
        callback: extern "C" fn(message: *mut MessageEntity)
    );
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}

#[no_mangle]
pub extern "C" fn test_callback(message: *mut MessageEntity) {
    print_string("Hello World callback");
}

#[no_mangle]
pub extern "C" fn test_callback_2(message: *mut MessageEntity) {
    print_string("Hello World callback 2");
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    // toxoid_print_i32(4200);
    // toxoid_print_u64(888);
    // toxoid_print_f64(777.2);
    // print_string("Entity ID: ");
    // toxoid_print_u64(toxoid_entity_create());
    print_string("Hello World from WASM");
    toxoid_net_add_event("test", test_callback);
    toxoid_net_add_event("test", test_callback_2);
}