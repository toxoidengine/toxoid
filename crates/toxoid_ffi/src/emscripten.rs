use core::ffi::{c_char, c_int};
use std::ffi::CString;
use std::ptr;
use std::os::raw::c_void;

extern "C" {
    pub fn emscripten_set_main_loop_arg(
        f: unsafe extern "C" fn(),
        arg: *mut std::ffi::c_void,
        fps: i32,
        sim_infinite_loop: i32,
    );
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_fetch_attr_init(attr: *mut emscripten_fetch_attr_t);
    pub fn emscripten_fetch(attr: *const emscripten_fetch_attr_t, url: *const c_char) -> *mut emscripten_fetch_t;
    pub fn emscripten_fetch_close(fetch: *mut emscripten_fetch_t) -> i32;
    pub fn emscripten_set_keypress_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EmBool,
        callback: unsafe extern "C" fn(eventType: c_int, keyEvent: *const EmscriptenKeyboardEvent, userData: *mut c_void) -> EmBool,
    ) -> EmscriptenResult;
    pub fn emscripten_set_keydown_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EmBool,
        callback: unsafe extern "C" fn(eventType: c_int, keyEvent: *const EmscriptenKeyboardEvent, userData: *mut c_void) -> EmBool,
    ) -> EmscriptenResult;
    pub fn emscripten_set_keyup_callback(
        target: *const c_char,
        userData: *mut c_void,
        useCapture: EmBool,
        callback: unsafe extern "C" fn(eventType: c_int, keyEvent: *const EmscriptenKeyboardEvent, userData: *mut c_void) -> EmBool,
    ) -> EmscriptenResult;
    pub fn toxoid_set_keydown_callback(
        target: *const ::std::os::raw::c_char,
        userData: *mut ::std::os::raw::c_void,
        useCapture: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            eventType: ::std::os::raw::c_int,
            keyEvent: *const EmscriptenKeyboardEvent,
            userData: *mut ::std::os::raw::c_void,
        ) -> EmBool,
    ) -> EmscriptenResult;
    pub fn toxoid_set_keyup_callback(
        target: *const ::std::os::raw::c_char,
        userData: *mut ::std::os::raw::c_void,
        useCapture: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            eventType: ::std::os::raw::c_int,
            keyEvent: *const EmscriptenKeyboardEvent,
            userData: *mut ::std::os::raw::c_void,
        ) -> EmBool,
    ) -> EmscriptenResult;
    pub fn emscripten_run_script(
        script: *const ::std::os::raw::c_char,
    );
    pub fn emscripten_websocket_new(
        createAttributes: *mut EmscriptenWebSocketCreateAttributes
    ) -> *mut ::std::os::raw::c_void;
    pub fn emscripten_websocket_send_utf8_text(
        websocket: *mut ::std::os::raw::c_void,
        message: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn emscripten_websocket_send_binary(
        websocket: *mut ::std::os::raw::c_void,
        message: *const ::std::os::raw::c_void,
        numBytes: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn emscripten_websocket_close(
        websocket: *mut ::std::os::raw::c_void,
        code: ::std::os::raw::c_int,
        reason: *const ::std::os::raw::c_char,
    );
    pub fn emscripten_websocket_set_onopen_callback_on_thread(
        socket: *mut ::std::os::raw::c_void,
        userData: *mut ::std::os::raw::c_void,
        callback: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
        targetThread: *mut ::std::os::raw::c_void,
    ) -> EmscriptenResult;
    pub fn emscripten_websocket_set_onmessage_callback_on_thread(
        socket: *mut ::std::os::raw::c_void,
        userData: *mut ::std::os::raw::c_void,
        callback: unsafe extern "C" fn(
            eventType: ::std::os::raw::c_int,
            websocketEvent: *const EmscriptenWebSocketMessageEvent,
            userData: *mut ::std::os::raw::c_void,
        ),
        targetThread: *mut ::std::os::raw::c_void
    ) -> EmscriptenResult;
    // Make the same bindings for onerror and onclose
    pub fn emscripten_websocket_set_onerror_callback_on_thread(
        socket: *mut ::std::os::raw::c_void,
        userData: *mut ::std::os::raw::c_void,
        callback: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
        targetThread: *mut ::std::os::raw::c_void,
    ) -> EmscriptenResult;
    pub fn emscripten_websocket_set_onclose_callback_on_thread(
        socket: *mut ::std::os::raw::c_void,
        userData: *mut ::std::os::raw::c_void,
        callback: unsafe extern "C" fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
        targetThread: *mut ::std::os::raw::c_void,
    ) -> EmscriptenResult;
}

pub const EM_CALLBACK_THREAD_CONTEXT_MAIN_RUNTIME_THREAD: *mut c_void = 0x1 as *mut c_void;
pub const EM_CALLBACK_THREAD_CONTEXT_CALLING_THREAD: *mut c_void = 0x2 as *mut c_void;

#[allow(non_snake_case)]
#[repr(C)]
pub struct EmscriptenWebSocketMessageEvent {
    pub socket: *mut c_void,
    pub data: *mut u8,
    pub numBytes: u32,
    pub isText: EmBool,
}

#[repr(C)]
pub struct EmscriptenWebSocketCreateAttributes {
    pub url: *const ::std::os::raw::c_char,
    pub protocol: *const ::std::os::raw::c_char,
}

pub type EmBool = c_int;
pub type EmscriptenResult = c_int;

#[allow(non_snake_case)]
#[repr(C)]
pub struct EmscriptenKeyboardEvent {
    pub timestamp: f64,
    pub location: u32,
    pub ctrlKey: EmBool,
    pub shiftKey: EmBool,
    pub altKey: EmBool,
    pub metaKey: EmBool,
    pub repeat: EmBool,
    pub charCode: u32,
    pub keyCode: u32,
    pub which: u32,
    pub key: [u8; 32usize],
    pub code: [u8; 32usize],
    pub charValue: [u8; 32usize],
    pub locale: [u8; 32usize],
}

pub const EMSCRIPTEN_FETCH_LOAD_TO_MEMORY: u32 = 1;
pub const EMSCRIPTEN_FETCH_STREAM_DATA: u32 = 2;
pub const EMSCRIPTEN_FETCH_PERSIST_FILE: u32 = 4;
pub const EMSCRIPTEN_FETCH_APPEND: u32 = 8;
pub const EMSCRIPTEN_FETCH_REPLACE: u32 = 16;
pub const EMSCRIPTEN_FETCH_NO_DOWNLOAD: u32 = 32;
pub const EMSCRIPTEN_FETCH_SYNCHRONOUS: u32 = 64;
pub const EMSCRIPTEN_FETCH_WAITABLE: u32 = 128;

pub fn start_loop(update_loop: unsafe extern "C" fn()) {
    unsafe {
        emscripten_set_main_loop_arg(
            update_loop,
            std::ptr::null_mut(),
            -1,
            0,
        );
    }
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct emscripten_fetch_attr_t {
    pub requestMethod: [::std::os::raw::c_char; 32usize],
    pub userData: *mut ::std::os::raw::c_void,
    pub onsuccess: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onerror: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onprogress: ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub onreadystatechange:
        ::std::option::Option<unsafe extern "C" fn(fetch: *mut emscripten_fetch_t)>,
    pub attributes: u32,
    pub timeoutMSecs: u32,
    pub withCredentials: ::std::os::raw::c_int,
    pub destinationPath: *const ::std::os::raw::c_char,
    pub userName: *const ::std::os::raw::c_char,
    pub password: *const ::std::os::raw::c_char,
    pub requestHeaders: *const *const ::std::os::raw::c_char,
    pub overriddenMimeType: *const ::std::os::raw::c_char,
    pub requestData: *const ::std::os::raw::c_char,
    pub requestDataSize: usize,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct emscripten_fetch_t {
    pub id: u32,
    pub userData: *mut ::std::os::raw::c_void,
    pub url: *const ::std::os::raw::c_char,
    pub data: *const ::std::os::raw::c_char,
    pub numBytes: u64,
    pub dataOffset: u64,
    pub totalBytes: u64,
    pub readyState: ::std::os::raw::c_ushort,
    pub status: ::std::os::raw::c_ushort,
    pub statusText: [::std::os::raw::c_char; 64usize],
    pub __proxyState: u32,
    pub __attributes: emscripten_fetch_attr_t,
}