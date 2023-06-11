extern "C" {
    pub fn sephiroth(a: i32, b: i32) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn init_app(a: i32, b: i32) -> i32 {
    sephiroth(a, b)
}

fn main() {
    println!("Hello world!");
}
