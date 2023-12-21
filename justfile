wasm:
	cargo build --target wasm32-unknown-unknown
build:
	cargo build --jobs=1
run:
	cargo run --jobs=1