use core::mem::transmute;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn print_i32() {
    unsafe {
        host_print_i32(3);
    }
}

#[no_mangle]
pub extern "C" fn print_f32() {
    unsafe {
        host_print_f32(3.14);
    }
}

extern {
    fn host_print_i32(val: i32);
    fn host_print_f32(val: f32);
}