# CLAUDE.md

## Project Overview

`auto-palette` is a Rust library for automatic color palette extraction from images.
The project consists of three crates:

- **`auto-palette`** (`crates/auto-palette/`): Core library implementing color extraction algorithms
- **`auto-palette-cli`** (`crates/auto-palette-cli/`): Command-line interface
- **`auto-palette-wasm`** (`crates/auto-palette-wasm/`): WebAssembly bindings for browser usage

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

# Lint Rust code (all warnings are denied in CI)
cargo clippy -- -D warnings

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

### Color Extraction Pipeline

1. **Image Loading**: `ImageData::load()` reads the image
2. **Segmentation**: Algorithms (DBSCAN, K-means, SLIC, SNIC) segment the image into regions
3. **Color Extraction**: Prominent colors are extracted from each segment
4. **Theme Filtering**: Colors are filtered by theme (Colorful, Vivid, Muted, Light, Dark, etc.)
5. **Palette Generation**: Final `Palette` contains ranked `Swatch` objects

### Pixel Representation

Pixels are represented as 5D vectors `[L, a, b, x, y]`:

- `L, a, b`: CIELAB color space values
- `x, y`: **1-indexed** normalized spatial coordinates
  - `x = (col + 1) / width`, `y = (row + 1) / height`
  - Defined in `image/segmentation/helper.rs`

This convention is used throughout all segmentation and clustering code.

### Key Modules
- **`algorithm/`**: Implements different extraction algorithms (DBSCAN, K-means, SLIC, SNIC, etc.).
- **`color/`**: Color space conversions (RGB, HSL, LAB, LCHuv, etc.).
- **`image/segmentation/`**: Image segmentation algorithms.
- **`math/`**: Mathematical utilities (clustering, distance metrics, sampling, etc.).
- **`theme/`**: Theme-based color filtering logic.

---

## Notes

### DBSCAN order sensitivity
Border point assignment depends on neighbor iteration order. Changing the data structure used for neighbor search (e.g., KD-Tree → grid) changes the iteration order, which causes different cluster assignments. This cascades into different palette rankings and may require updating test expectations across the entire workspace.

### Color comparison in tests
- Use the `assert_color_eq!(color1, color2)` macro for color comparisons (delta-E tolerance of 1.0). A custom tolerance can be passed as a 3rd argument. Defined in `src/assert.rs`.
- IMPORTANT: CLI tests in `auto-palette-cli/tests/cli.rs` use **exact hex string matching** via `contains()`, NOT color-tolerance comparison. Palette order changes will break these tests.

### Build profile differences
`dev` and `test` profiles use `opt-level = 3` (not the default 0) for faster execution during development. Release profile uses `opt-level = 'z'` with LTO for size optimization.

### Pre-commit hooks
husky + lint-staged auto-formats staged files on commit. If a commit fails due to formatting, re-stage the formatted files and commit again — do not bypass with `--no-verify`.

### Safety
Do not read `.env`, `.env.*`, or `secrets/**`. Avoid network commands unless explicitly requested.

---

## Git Workflow

- **Commits**: Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format
  - Prefixes: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`, `perf:`, `ci:`, `build:`
- **Branches**: Use `type/description` format
  - Examples: `feat/slic-algorithm-support`, `fix/segmentation-mask-filtering`, `chore/update-deps`