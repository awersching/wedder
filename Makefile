all:
	cargo build --release

lint:
	cargo build
	cargo clippy