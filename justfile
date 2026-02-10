# List available recipes when `just` is called without arguments
_default:
  just --list -u

# Aliases
alias r := ready
alias f := format
alias l := lint
alias t := test

# Setup environment
# Install all necessary tools and dependencies
setup:
  rustup toolchain install nightly --component rustfmt
  rustup target add wasm32-unknown-unknown
  cargo install cargo-binstall
  cargo binstall --no-confirm cargo-nextest cargo-llvm-cov cargo-audit cargo-udeps wasm-bindgen-cli
  pnpm install


# Ready
# Run all checks before commit
ready: format lint test check


# Formatting
# Format all files
format: format-rust format-js format-toml

# Format Rust files
format-rust:
  cargo +nightly fmt --all

# Format JavaScript and TypeScript files
format-js:
  biome format --write .

# Format TOML files
format-toml:
  taplo fmt


# Linting
# Lint all files
lint: lint-rust lint-js

# Lint Rust files
lint-rust:
  cargo clippy -- -D warnings

# Lint JavaScript and TypeScript files
lint-js:
  biome lint .


# Checking
# Run all checks
check: lint audit udeps

# Check for known vulnerabilities in dependencies
audit:
  cargo audit

# Check for unused dependencies
udeps:
  cargo +nightly udeps --all-features --all-targets


# Testing
# Run all tests
test:test-lib test-cli test-wasm

# Run auto-palette library tests
test-lib:
  cargo nextest run --tests --all-features --package auto-palette

# Run auto-palette-cli tests
test-cli:
  cargo nextest run --tests --all-features --package auto-palette-cli

# Run auto-palette-wasm tests
test-wasm:
  cargo test --tests --package auto-palette-wasm --target wasm32-unknown-unknown

# Run auto-palette-wasm JavaScript tests
test-wasm-js:
  pnpm --filter @auto-palette/wasm test:unit


# Coverage
# Run auto-palette tests with coverage
coverage:
  cargo llvm-cov nextest --all-features --package auto-palette --html


# Benchmarking
# Run Rust benchmarks
bench:
  cargo bench --package auto-palette

# Run WASM JavaScript benchmarks
bench-wasm-js:
  pnpm --filter @auto-palette/wasm test:bench


# Building
# Build all crates
build: build-lib build-cli build-wasm

# Build auto-palette library
build-lib:
  cargo build --package auto-palette --lib --release

# Build auto-palette-cli binary
build-cli:
  cargo build --package auto-palette-cli --bins --release

# Build WASM package
build-wasm:
  pnpm --filter @auto-palette/wasm build


