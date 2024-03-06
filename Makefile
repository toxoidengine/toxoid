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
	emcc snake_engine.wasm -o snake_engine_2.js -s USE_PTHREADS=1 -s PTHREAD_POOL_SIZE=3 -sEXPORT_ES6=1 -sMODULARIZE=1 -sFORCE_FILESYSTEM  