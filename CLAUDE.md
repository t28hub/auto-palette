# CLAUDE.md

This file provides **project-specific guidance for Claude Code** (claude.ai/code).  
Claude reads this file at startup to follow our repository rules, workflows, and commands.

---

## Project Overview

`auto-palette` is a Rust library for automatic color palette extraction from images. The project consists of three main components:

- **`auto-palette`**: Core library implementing color extraction algorithms
- **`auto-palette-cli`**: Command-line interface
- **`auto-palette-wasm`**: WebAssembly bindings for browser usage

---

## How Claude should work in this repo (IMPORTANT)

1. **Plan-first, then edit**
  - Read relevant files, outline a short plan, and get explicit confirmation only if something is blocking. Keep diffs minimal and localized.
2. **Keep changes small & reversible**
  - Prefer small commits. Use `Write`/`Edit` (or `MultiEdit` for cross-file) to keep diffs coherent.
3. **Formatting & Linting before tests**
  - Always run the format/lint commands listed below **before** running tests.
4. **Tests must pass**
  - Use `cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm` as the default test suite. Add/adjust tests for new behavior.
5. **Commit message**
  - Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format for commit messages.  
6. **Safety**
  - Do not read `.env` or `secrets/**`. Avoid network commands unless explicitly requested. Never exfiltrate code or credentials.

---

## Development Commands

### Building and Testing
```bash
# Run all tests in the workspace, excluding `auto-palette-wasm`
cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm

# Run all tests in the `auto-palette` crate
cargo nextest run --tests --all-features --package auto-palette

# Run all tests in the `auto-palette-cli` crate
cargo nextest run --tests --all-features --package auto-palette-cli

# Run all tests in the `auto-palette-wasm` crate
cargo nextest run --tests --package auto-palette-wasm --target wasm32-unknown-unknown

# Run all tests (may fail on non-wasm32 targets)
# Note: WASM tests require `wasm32-unknown-unknown` target or `wasm-pack`
cargo nextest run --lib

# Run a single test
cargo nextest run -- test_name

# Build all crates
cargo build --all

# Build with release optimizations
cargo build --release --all
```

### Code Quality and Formatting
```bash
# Format Rust code (requires nightly toolchain)
cargo +nightly fmt --all

# Format TOML files
taplo fmt

# Format all code (uses pnpm scripts)
pnpm format

# Lint JavaScript/TypeScript (via Biome)
pnpm lint
# or
biome lint .
```

### Examples and CLI Testing
```bash
# Run the simple example
cargo run --example simple --release --features='image' -- 'path/to/image.jpg'

# Run the algorithm comparison example  
cargo run --example algorithm --release --features='image' -- 'algorithm_name'

# Run the theme comparison example
cargo run --example theme --release --features='image' -- 'theme_name'

# Test CLI functionality
cargo run --bin auto-palette-cli -- --help
```

### WebAssembly Development
```bash
# Build WASM package
cd crates/auto-palette-wasm
wasm-pack build --target web --release
```

## Architecture

### Core Algorithm Flow
1. **Image Loading**: `ImageData::load()` handles various image formats.
2. **Segmentation**: Images are segmented using algorithms like SLIC, SNIC, DBSCAN, K-means, etc.
3. **Color Extraction**: Prominent colors are extracted from segments.
4. **Theme Filtering**: Colors are filtered based on themes (Colorful, Vivid, Muted, Light, Dark, etc.).
5. **Palette Generation**: Final `Palette` contains ranked `Swatch` objects.

### Key Modules
- **`algorithm/`**: Implements different extraction algorithms (DBSCAN, K-means, SLIC, SNIC, etc.).
- **`color/`**: Color space conversions (RGB, HSL, LAB, LCHuv, etc.).
- **`image/segmentation/`**: Image segmentation algorithms.
- **`math/`**: Mathematical utilities (clustering, distance metrics, sampling, etc.).
- **`theme/`**: Theme-based color filtering logic.

### Workspace Structure
- `crates/auto-palette/`: Core library.
- `crates/auto-palette-cli/`: CLI tool with argument parsing and output formatting.
- `crates/auto-palette-wasm/`: WebAssembly bindings.

---

## Coding Guidelines for Claude

### Rust Code Style

- Follow the official guides: 
  - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
  - [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Prefer idiomatic, safe Rust. Any `unsafe` must be justified and minimal.
- Keep public APIs consistent and documented (docstrings/examples). Avoid breaking changes unless explicitly requested.
- Implement **general** solutions rather than hard-coding for specific cases.

### Testing

- Add/adjust unit tests near the changed code; keep image-related tests deterministic (fixed seeds/fixtures).
- Default runner: `cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm`. Use single-test patterns first (faster feedback), then expand.

## Performance 

- Use `cargo build --release` for performance testing
- Profile optimization is configured in `Cargo.toml` with `opt-level = 'z'` for release builds
- Benchmarks are available using `divan` (via `codspeed-divan-compat`)

--- 

## Claude’s Default Workflow (checklist)

1. Read relevant files; draft a brief plan.
2. Implement minimally-scoped changes.
3. Run formatting, linting, and tests:
   - Format Rust code: `cargo +nightly fmt --all`
   - Format TOML files: `taplo fmt`
   - Lint JavaScript/TypeScript: `pnpm lint` or `biome lint .`
   - Run tests: `cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm`
4. If failures occur:
   - Analyze error messages
   - Adjust code or tests as needed
   - Re-run tests until they pass
5. Write a clear Conventional Commit message.

---

## Notes & Boundaries

* Avoid reading .env, .env.*, secrets/**, or any credentials file. 
* Avoid network requests and shelling out to external services unless explicitly requested. 
* Do not reformat or rewrite unrelated files. Keep diffs focused.
