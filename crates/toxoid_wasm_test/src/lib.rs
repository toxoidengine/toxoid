extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_u64(v: u64);
    pub fn toxoid_print_f32(v: f32);
    pub fn toxoid_print_f64(v: f64);
    pub fn toxoid_print_string(v: &str);
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    toxoid_print_i32(4200);
    toxoid_print_string("Hello World");
}