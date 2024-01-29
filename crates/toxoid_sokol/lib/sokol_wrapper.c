// Emscripten
#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#include <emscripten/html5.h>
#include <emscripten/websocket.h>
#endif

// Sokol
#include "sokol_app.h"
#include "sokol_glue.h"
#include "sokol_log.h"
#include "sokol_time.h"
#define SOKOL_IMPL
#include "sokol_gfx.h"
#include "sokol_gp.h"

// Toxoid Sokol
#ifdef TOXOID_FETCH
#include "sokol_fetch.h"
#endif

#ifdef TOXOID_AUDIO
#include "sokol_audio.h"
#endif

#ifdef TOXOID_IMGUI
#include "cimgui/cimgui.h"
#include "sokol_imgui.h"
#endif

#ifdef TOXOID_SPINE
#include "spine-runtimes/spine-c/spine-c/include/spine/spine.h"
#include "sokol_spine.h"
#endif

#ifdef __EMSCRIPTEN__
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
#endif

#ifdef TOXOID_STB
/*
    Unlike most libraries out there, the STB headers are (usually) directly compiled within your project's source code; no linking required. Most other stb headers require you to define special macros before you include them. Read the documentation (at the top of the header files) to know which macro to define.
*/
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#endif