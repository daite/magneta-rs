# Makefile for magneta project

BINARY=target/debug/magneta
INSTALL_DIR=$(HOME)/.cargo/bin
INSTALLED_BINARY=$(INSTALL_DIR)/magneta
INSTALLED_CONFIG=$(INSTALL_DIR)/config.toml

# Default target
all: build

# Help
help:
	@echo ""
	@echo "Available commands:"
	@echo "  make build                - Build the project (cargo build)"
	@echo "  make install              - Install binary and copy config"
	@echo "  make uninstall            - Remove installed binary and config"
	@echo "  make test                 - Run all tests (cargo test)"
	@echo "  make fmt                  - Format the code (cargo fmt)"
	@echo "  make lint                 - Lint the code with clippy (cargo clippy)"
	@echo "  make fmt-check            - Check formatting without modifying files (cargo fmt --check)"
	@echo "  make check                - Run format check and clippy lint together"
	@echo "  make clean                - Clean the build artifacts (cargo clean)"
	@echo "  make update               - Update Cargo dependencies"
	@echo "  make run [ARGS=...]       - Build and run the binary with optional arguments"
	@echo ""

# Build the project
build:
	cargo build --verbose

# Install: binary + config
install:
	cargo install --path .
	@echo "Copying config.toml to $(INSTALL_DIR)"
	cp -v config.toml $(INSTALLED_CONFIG)

# Uninstall: remove binary and config with checks
uninstall:
	@if [ -f $(INSTALLED_BINARY) ]; then \
		echo "Removing binary: $(INSTALLED_BINARY)"; \
		rm -v $(INSTALLED_BINARY); \
	else \
		echo "Binary not found: $(INSTALLED_BINARY) (already removed?)"; \
	fi
	@if [ -f $(INSTALLED_CONFIG) ]; then \
		echo "Removing config: $(INSTALLED_CONFIG)"; \
		rm -v $(INSTALLED_CONFIG); \
	else \
		echo "Config not found: $(INSTALLED_CONFIG) (already removed?)"; \
	fi

# Run tests
test:
	cargo test --verbose

# Build and run the binary directly with arguments
run: build
	@echo "Running binary: $(BINARY) $(filter-out $@,$(MAKECMDGOALS))"
	$(BINARY) $(filter-out $@,$(MAKECMDGOALS))

# Format the code
fmt:
	cargo fmt --all

# Lint the code
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Check formatting (without changing)
fmt-check:
	cargo fmt --all -- --check

# Run both fmt-check and clippy
check: fmt-check lint

# Clean build artifacts
clean:
	cargo clean

# Update Cargo dependencies
update:
	cargo update

# Prevent Make from thinking the additional args are make targets
%:
	@: