all: format lint test build

build:
	cargo build --release

ci:
	cargo build --release --target "${TARGET}"
	cp "target/${TARGET}/release/${PROJECT_NAME}" "${PROJECT_NAME}-${TRAVIS_TAG}-${TARGET}"

format:
	cargo fmt

lint:
	cargo clippy

lint-all:
	cargo clippy -- -W clippy::pedantic

test:
	cargo test
