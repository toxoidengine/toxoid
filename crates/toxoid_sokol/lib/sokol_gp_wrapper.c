#include <emscripten.h>
#include <emscripten/html5.h>
// #include <emscripten/websocket.h>
#include "sokol_app.h"
#include "sokol_glue.h"
#include "sokol_log.h"
#include "sokol_time.h"
#define SOKOL_IMPL
#include "sokol_gfx.h"
#include "sokol_gp.h"
#define CIMGUI_DEFINE_ENUMS_AND_STRUCTS
#include "cimgui/cimgui.h"
#include "sokol_imgui.h"
/*
    Unlike most libraries out there, the STB headers are (usually) directly compiled within your project's source code; no linking required. Most other stb headers require you to define special macros before you include them. Read the documentation (at the top of the header files) to know which macro to define.
*/
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"

EMSCRIPTEN_KEEPALIVE
EMSCRIPTEN_RESULT toxoid_set_keypress_callback(const char *target, void *userData, EM_BOOL useCapture, em_key_callback_func callback) {
    return emscripten_set_keypress_callback(target, userData, useCapture, callback);
}

EMSCRIPTEN_KEEPALIVE
EMSCRIPTEN_RESULT toxoid_set_keydown_callback(const char *target, void *userData, EM_BOOL useCapture, em_key_callback_func callback) {
    return emscripten_set_keydown_callback(target, userData, useCapture, callback);
}

EMSCRIPTEN_KEEPALIVE
EMSCRIPTEN_RESULT toxoid_set_keyup_callback(const char *target, void *userData, EM_BOOL useCapture, em_key_callback_func callback) {
    return emscripten_set_keyup_callback(target, userData, useCapture, callback);
}