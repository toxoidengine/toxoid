install:
	cargo install cargo-component --locked
	cargo install --locked wasm-tools

install-osx:
	brew install llvm

submodule:
	git submodule update --init

submodule-add:
	git submodule add -b docker_iter https://github.com/cimgui/cimgui.git crates/toxoid_sokol/lib/cimgui

	git submodule add -b 4.1 https://github.com/EsotericSoftware/spine-runtimes.git crates/toxoid_sokol/lib/spine-runtimes

	git submodule add https://github.com/floooh/sokol.git crates/toxoid_sokol/lib/sokol && \
	cd crates/toxoid_sokol/lib/sokol && \ 
	git checkout 56e98211a2fbd906d37a1051475e04b22a4b62ee && \
	cd ../../../.. && \
	git add crates/toxoid_sokol/lib/sokol

	git submodule add https://github.com/edubart/sokol_gp.git crates/toxoid_sokol/lib/sokol_gp && \
	cd crates/toxoid_sokol/lib/sokol_gp && \
	git checkout a6ce39f93fb2da2c47b70cdd4d1c0a35c0e756ef && \
	cd ../../../.. && \
	git add crates/toxoid_sokol/lib/sokol_gp

build-wit:
	cd app/guest && cargo component check && cargo build
	cd crates/toxoid_host && cargo component check && cargo build
	cd crates/toxoid_guest && cargo component check && cargo build

build:
	cd app/guest && cargo component build
	cp target/wasm32-wasip1/debug/guest.wasm app/host/guest.wasm
	cp target/wasm32-wasip1/debug/guest.wasm guest.wasm
	cd app/host && cargo build

build-host:
	cd crates/toxoid_host && cargo component build

build-em:
	EMCC_FLAGS="-s EXPORTED_FUNCTIONS=['_main','_cabi_realloc'] \
		-s EXPORT_ES6=1 \
		-s MODULARIZE=1 \
		-s STACK_SIZE=1mb \
		-s FETCH=1 \
		-s MIN_WEBGL_VERSION=2 \
		-s MAX_WEBGL_VERSION=2 \
		-s USE_WEBGL2=1 \
		-s FULL_ES3=1 \
		-s USE_GETADDRINFO=1 \
		-s ALLOW_MEMORY_GROWTH=1 \
		-s FORCE_FILESYSTEM=1 \
		-s ERROR_ON_UNDEFINED_SYMBOLS=1 \
		-s STANDALONE_WASM=1 \
		-s EXPORTED_RUNTIME_METHODS=['ccall','cwrap'] \
		-s NO_EXIT_RUNTIME=1" \
	cd app/host && TARGET=wasm32-unknown-emscripten cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/host.wasm dist/host.wasm
	cp target/wasm32-unknown-emscripten/debug/host.js dist/host.js

run-em:
	basic-http-server dist
	
run:
	cd app/host && cargo run

run-cli:
	cargo run --package toxoid_cli -- watch