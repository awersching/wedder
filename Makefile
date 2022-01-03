all: format lint test build

build:
	cargo build --release

release:
	cargo build --release --target "${TARGET}"
	cp "target/${TARGET}/release/${PROJECT_NAME}" "${PROJECT_NAME}-${GITHUB_REF_NAME}-${TARGET}"

format:
	cargo fmt

lint:
	cargo clippy

lint-all:
	cargo clippy -- -W clippy::pedantic

test:
	cargo test --features "test"
