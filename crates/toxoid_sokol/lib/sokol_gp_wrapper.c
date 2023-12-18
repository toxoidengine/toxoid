// #include <emscripten.h>
// #include <emscripten/fetch.h>
// #include <emscripten/html5.h>
#include "sokol_app.h"
#include "sokol_glue.h"
#include "sokol_log.h"
#include "sokol_time.h"
#define SOKOL_IMPL
#include "sokol_gfx.h"
#include "sokol_gp.h"
/*
    Unlike most libraries out there, the STB headers are (usually) directly compiled within your project's source code; no linking required. Most other stb headers require you to define special macros before you include them. Read the documentation (at the top of the header files) to know which macro to define.
*/
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"