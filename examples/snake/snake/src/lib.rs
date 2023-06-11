// #![no_std]
use hashbrown::HashMap;

extern "C" {
    pub fn sephiroth(a: i32, b: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let test = "test";
}