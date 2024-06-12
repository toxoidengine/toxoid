// #![no_std]
// use core::panic::PanicInfo;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_u64(v: u64);
    pub fn toxoid_print_f32(v: f32);
    pub fn toxoid_print_f64(v: f64);
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    toxoid_print_i32(420);
}

// // Define a panic handler.
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }