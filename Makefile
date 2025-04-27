# Makefile for magneta project

# Default target
all: build

# Build the project
build:
	cargo build --verbose

# Run tests
test:
	cargo test --verbose

# Format the code
fmt:
	cargo fmt --all

# Lint the code
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean target
clean:
	cargo clean

# Check formatting (without changing)
fmt-check:
	cargo fmt --all -- --check

# Run both fmt-check and clippy
check: fmt-check lint

# Update dependencies
update:
	cargo update
