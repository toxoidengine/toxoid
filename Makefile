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