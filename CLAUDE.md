# CLAUDE.md

## Commands

This project uses [just](https://just.systems/) as a task runner. Run `just` to see all recipes.

```sh
just setup    # Install cargo tools (nextest, llvm-cov, etc.) and Node.js dependencies
just ready    # Run formatting, linting, tests, and checks
just format   # Format all files
just lint     # Lint all files
just test     # Run all tests (lib, cli, wasm)

# Run a single test
cargo nextest run -- test_name

# Run tests for a specific crate
just test-lib
just test-cli
just test-wasm

# Examples and CLI (run from the repository root; examples load images from ./gfx)
cargo run --example simple -- 'path/to/image.jpg'
cargo run --example algorithm -- 'algorithm_name'
cargo run --example theme -- 'theme_name'
cargo run --bin auto-palette-cli -- --help
```

## Notes

### Pixel representation
Pixels are 5D vectors `[L, a, b, x, y]` where `x, y` are **1-indexed** normalized coordinates:
`x = (col + 1) / width`, `y = (row + 1) / height`. Built in `ImageData` (`image/data.rs`) and assumed by all
segmentation and clustering code.

### DBSCAN order sensitivity
Border point assignment depends on neighbor iteration order. Changing the data structure used for neighbor search (e.g., KD-Tree → grid) changes the iteration order, which causes different cluster assignments. This cascades into different palette rankings and may require updating test expectations across the entire workspace.

### Color comparison in tests
- Use the `assert_color_eq!(color1, color2)` macro for color comparisons (delta-E tolerance of 1.0). A custom tolerance can be passed as a 3rd argument. Defined in `src/assert.rs`.
- CLI tests in `auto-palette-cli/tests/cli.rs` use **exact hex string matching** via `contains()`, not color-tolerance comparison. Palette order changes will break these tests.

### Build profile differences
`dev` and `test` profiles use `opt-level = 3` (not the default 0) for faster execution during development. Release profile uses `opt-level = 'z'` with LTO for size optimization.

### Git hooks
Git hooks are defined in `.lefthook.yml`. Do not bypass them with `--no-verify`.

### Network
Avoid network commands (dependency installs, registry queries) unless explicitly requested.

## Git Workflow

- **Commits**: Use [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format
  - Prefixes: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`, `perf:`, `ci:`, `build:`
- **Branches**: Use `type/description` format
  - Examples: `feat/slic-algorithm-support`, `fix/segmentation-mask-filtering`, `chore/update-deps`