# Makefile for magneta project

# Default target
all: build

# Help
help:
	@echo ""
	@echo "Available commands:"
	@echo "  make build        - Build the project (cargo build)"
	@echo "  make test         - Run all tests (cargo test)"
	@echo "  make fmt          - Format the code (cargo fmt)"
	@echo "  make lint         - Lint the code with clippy (cargo clippy)"
	@echo "  make fmt-check    - Check formatting without modifying files (cargo fmt --check)"
	@echo "  make check        - Run format check and clippy lint together"
	@echo "  make clean        - Clean the build artifacts (cargo clean)"
	@echo "  make update       - Update Cargo dependencies"
	@echo ""

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
