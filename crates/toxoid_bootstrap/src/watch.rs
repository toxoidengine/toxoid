use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::thread;

// TODO: Make this configurable via ENV variable
const HOST_ADDRESS: &str = "127.0.0.1:7878";
// TODO: Make this configurable via ENV variable
const GUEST_WASM_PATH: &str = "app/host/guest.wasm";

fn watch() {
    // Start a thread to listen for TCP messages
    thread::spawn(move || {
        // Define the TCP address and port
        let listener = TcpListener::bind(HOST_ADDRESS).unwrap();
        println!("Listening on {}", HOST_ADDRESS);
        for stream in listener
            .incoming()
            .filter_map(Result::ok)
        {
            let mut conn = BufReader::new(stream);
            let mut buffer = String::new();
            conn
                .read_line(&mut buffer)
                .unwrap();
            if buffer.contains("reload") {
                println!("Reloading WASM component...");
                toxoid_runtime::load_wasm_component(GUEST_WASM_PATH)
                    .unwrap_or_else(|e| println!("Failed to reload WASM component: {}", e));
            }
        }
    });
    // Initial load of the main WASM component / game engine script
    // TODO: Some kind of deadlock on this when grabbing the engine and trying to
    // run Sokol / render loop / sapp at the same time...
    // if std::path::Path::new(GUEST_WASM_PATH).exists() {
    //     println!("Loading WASM component...");
    //     toxoid_runtime::load_wasm_component(GUEST_WASM_PATH)
    //         .unwrap_or_else(|e| println!("Failed to load WASM component: {}", e));
    // } else {
    //     println!("WASM component not found at {}, modify the guest script source file or use `toxoid_cli build` to generate it", GUEST_WASM_PATH);
    // }
}

pub fn init() {
    watch();
}
