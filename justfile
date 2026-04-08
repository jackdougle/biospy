# Biospy development commands

build-rust:
    cd rust-core && cargo build

test-rust:
    cd rust-core && cargo test

fmt-rust:
    cd rust-core && cargo fmt --all

clip:
    cd rust-core && cargo clippy --all-targets -- -D warnings

dev:
    cd python-sdk && maturin develop

test-py:
    pytest tests/python/ -v

lint-py:
    ruff check python-sdk/biospy/

test: test-rust test-py

lint: fmt-rust clip lint-py

all: lint test
