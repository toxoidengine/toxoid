extern "C" {
    pub fn sephiroth(a: i32, b: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    sephiroth(5, 6);
}