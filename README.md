# Toxoid Engine
*⚠️ Warning ⚠️ - This repository is under construction.*


# Web Target
Set up Emscripten: https://emscripten.org/docs/getting_started/downloads.html

# Examples
How to run examples:
```
rustup target add wasm32-unknown-emscripten
make example-snake
```

Snake:

https://github.com/toxoidengine/toxoid/assets/16667416/a9cf38da-bad2-4746-9f63-cfbfd5657b9c

Multiplayer Spine Bone Animation and ImGui:

https://github.com/toxoidengine/toxoid/assets/16667416/59fd5454-69c2-4f90-83c2-6f83da03c888

# Related Projects
- Powers my UGC game / multiplayer online sandbox RPG [Legend of Worlds](http://legendofworlds.com/).
- Also maintain [flecs-polyglot](https://github.com/flecs-hub/flecs-polyglot) - A universal scripting API for flecs on all languages that compile to WebAssembly.


# Credits
This technology would not be possible without:
- [Flecs](https://github.com/SanderMertens/flecs): A fast, lightweight (zero dependency), and portable entity component system written in C.
- [Sokol](https://github.com/floooh/sokol): Headers-only cross-platform C renderer with WebGPU support.
- [Emscripten](https://github.com/emscripten-core/emsdk): A complete compiler toolchain to WebAssembly, using LLVM, with a special focus on speed, size, and the Web platform.
- [Rust](https://www.rust-lang.org/): A memory-safe systems programming language. 
