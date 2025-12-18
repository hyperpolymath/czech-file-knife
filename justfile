# czech-file-knife - Development Tasks
# SPDX-License-Identifier: AGPL-3.0-or-later
set shell := ["bash", "-uc"]
set dotenv-load := true

project := "czech-file-knife"
cli_package := "cfk-cli"

# Show all recipes
default:
    @just --list --unsorted

# Build all packages in release mode
build:
    cargo build --release

# Build CLI only
build-cli:
    cargo build --release -p {{cli_package}}

# Build all packages in debug mode
build-debug:
    cargo build

# Run all tests
test:
    cargo test --all-features

# Run tests with coverage (requires cargo-llvm-cov)
test-coverage:
    cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Run clippy lints
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean

# Run security audit (requires cargo-audit)
audit:
    cargo audit

# Run dependency checks (requires cargo-deny)
deny:
    cargo deny check

# Run all checks (lint, fmt, test, audit)
check: lint fmt-check test audit

# Build container image
container:
    podman build -t {{project}}:latest -f Containerfile .

# Run container
container-run:
    podman run --rm -it {{project}}:latest --help

# Run the CLI in debug mode
run *ARGS:
    cargo run -p {{cli_package}} -- {{ARGS}}

# Watch for changes and rebuild (requires cargo-watch)
watch:
    cargo watch -x "build -p {{cli_package}}"

# Generate documentation
docs:
    cargo doc --no-deps --all-features

# Open documentation in browser
docs-open:
    cargo doc --no-deps --all-features --open

# Update dependencies
update:
    cargo update

# Check for outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated

# Run fuzz tests (requires cargo-fuzz, nightly)
fuzz TARGET="fuzz_path":
    cd fuzz && cargo +nightly fuzz run {{TARGET}}

# Install the CLI locally
install:
    cargo install --path cfk-cli

# Uninstall the CLI
uninstall:
    cargo uninstall cfk

# Verify Guix package definition
guix-check:
    guix show -f guix.scm

# Enter Guix development shell
guix-shell:
    guix shell -D -f guix.scm

# Verify Nix flake
nix-check:
    nix flake check

# Enter Nix development shell
nix-shell:
    nix develop

# Full CI check (what CI runs)
ci: fmt-check lint test audit deny
    @echo "All CI checks passed!"
