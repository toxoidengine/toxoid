submodules:
	git submodule update --init --recursive

fmt: 
	cargo fmt

clippy: 
	cargo clippy

example-snake:
	cargo run --package snake_runner

expand:
	cargo expand --package snake > expanded.rs

sfz:
	cd /dev/toxoid/toxoid/examples/snake/dist && \
	sfz --coi

emcc:
	emcc snake_engine.wasm -o snake_engine_2.js -g -O0 -s MAIN_MODULE=2 -s FORCE_FILESYSTEM=1 -s ERROR_ON_UNDEFINED_SYMBOLS=0 -s EXPORT_ES6=1 -s MODULARIZE=1 -s USE_ES6_IMPORT_META=1 -s EXPORTED_RUNTIME_METHODS='[_free,_malloc,allocateUTF8,UTF8ToString,writeArrayToMemory,FS,loadDynamicLibrary]' -s ALLOW_MEMORY_GROWTH=1 -s MIN_WEBGL_VERSION=2 -s FETCH=1 -s STACK_SIZE=1mb -Wno-unused-command-line-argument -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=12 -matomics -mbulk-memory -lwebsocket.js -pthread 
