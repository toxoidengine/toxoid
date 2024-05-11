use crate::*;

pub fn load_sprite(filename: &str, callback: extern "C" fn(&mut Entity)) -> *mut Entity {
    unsafe { toxoid_engine_load_sprite(filename, callback) }
}

pub fn load_worldmap(filename: &str, callback: extern "C" fn(&mut Entity)) -> *mut Entity {
    unsafe { toxoid_engine_load_worldmap(filename, callback) }
}