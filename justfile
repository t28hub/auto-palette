# List available recipes when `just` is called without arguments
_default:
  @{{just_executable()}} --justfile {{justfile()}} --list -u

# Aliases
alias r := ready
alias f := format
alias l := lint
alias t := test

# Install all necessary tools and dependencies
[group('setup')]
setup:
  rustup toolchain install nightly --component rustfmt
  rustup target add wasm32-unknown-unknown
  cargo install cargo-binstall
  cargo binstall --no-confirm cargo-nextest cargo-llvm-cov cargo-audit cargo-udeps cargo-hack wasm-bindgen-cli
  pnpm install


# Run all checks before commit
ready: format lint test check


# Format all files
[group('format')]
format: format-rust format-js format-toml

# Format Rust files
[group('format')]
format-rust:
  cargo +nightly fmt --all

# Format JavaScript and TypeScript files
[group('format')]
format-js:
  pnpm exec biome format --write .

# Format TOML files
[group('format')]
format-toml:
  pnpm exec tombi format

# Check formatting of all files without writing
[group('format')]
format-check: format-check-rust format-check-js format-check-toml

# Check formatting of Rust files
[group('format')]
format-check-rust:
  cargo +nightly fmt --all -- --check

# Check formatting of JavaScript and TypeScript files
[group('format')]
format-check-js:
  pnpm exec biome format .

# Check formatting of TOML files
[group('format')]
format-check-toml:
  pnpm exec tombi format --check


# Lint all files
[group('lint')]
lint: lint-rust lint-js

# Lint Rust files
[group('lint')]
lint-rust:
  cargo clippy --package auto-palette -- -D warnings
  cargo clippy --all-features -- -D warnings

# Lint JavaScript and TypeScript files
[group('lint')]
lint-js:
  pnpm exec biome lint .


# Run all checks
[group('check')]
check: lint audit udeps

# Check for known vulnerabilities in dependencies
[group('check')]
audit:
  cargo audit

# Check for unused dependencies
[group('check')]
udeps:
  cargo +nightly udeps --all-features --all-targets

# Check the workspace with the minimum supported Rust version
[group('check')]
msrv:
  cargo hack check --rust-version --workspace --all-features


# Run all tests
[group('test')]
test: test-lib test-cli test-wasm

# Run auto-palette library tests
[group('test')]
test-lib *args:
  cargo nextest run --tests --all-features --package auto-palette {{args}}

# Run auto-palette library doc tests
[group('test')]
test-doc:
  cargo test --doc --all-features --package auto-palette

# Run auto-palette library examples
[group('test')]
examples:
  cargo run --example simple
  cargo run --example algorithm
  cargo run --example theme

# Run auto-palette-cli tests
[group('test')]
test-cli:
  cargo nextest run --tests --all-features --package auto-palette-cli

# Run auto-palette-wasm tests
[group('test')]
test-wasm:
  cargo test --tests --package auto-palette-wasm --target wasm32-unknown-unknown

# Run auto-palette-wasm JavaScript unit tests
[group('test')]
test-wasm-js:
  pnpm --filter @auto-palette/wasm test:unit

# Run all auto-palette-wasm JavaScript tests (unit and e2e) with coverage
[group('test')]
test-wasm-js-all:
  pnpm --filter @auto-palette/wasm test


# Run auto-palette tests with coverage
[group('coverage')]
coverage *args='--html':
  cargo llvm-cov nextest --all-features --package auto-palette {{args}}


# Run Rust benchmarks
[group('bench')]
bench:
  cargo bench --package auto-palette

# Run WASM JavaScript benchmarks
[group('bench')]
bench-wasm-js:
  pnpm --filter @auto-palette/wasm test:bench


# Build all crates
[group('build')]
build: build-lib build-cli build-wasm

# Build auto-palette library
[group('build')]
build-lib:
  cargo build --package auto-palette --lib --release

# Build auto-palette-cli binary
[group('build')]
build-cli:
  cargo build --package auto-palette-cli --bins --release

# Build WASM package
[group('build')]
build-wasm:
  pnpm --filter @auto-palette/wasm build
