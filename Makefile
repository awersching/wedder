all:
	cargo build --release

lint:
	cargo clippy

lint-all:
	cargo clippy -- -W clippy::pedantic
