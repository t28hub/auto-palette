# Contributing to `auto-palette`

This project welcomes contributions from the community. Please read this guide before opening issues or pull requests.  
Note: This project is released with a [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md). By participating, you agree to abide by it.

## Issues

Use [GitHub Issues](https://github.com/t28hub/auto-palette/issues) for bugs and enhancements.  
Please search existing issues before filing a new one.

## Pull Requests

Please follow these steps:

1. Fork and clone the repository

```sh
git clone git@github.com:t28hub/auto-palette.git
cd auto-palette
```

2. Create a feature branch

```sh
git checkout -b feat/short-description
```

3. Make changes and write tests

4. Format and lint

```sh
pnpm format && pnpm lint         # Biome + rustfmt + Taplo
cargo +nightly fmt --all         # Ensure Rust formatting (nightly config)
taplo fmt                        # Ensure TOML formatting
cargo clippy -- -D warnings      # Deny Rust lints
```

5. Run tests

```sh
cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm
```

Optional (Wasm/TS package):

```sh
pnpm -C packages/auto-palette-wasm test:e2e:install   # first-time only (Playwright)
pnpm -C packages/auto-palette-wasm test               # Vitest unit/e2e
```

6. Try examples (Rust)

```sh
cargo run --example simple --release --features='image'
cargo run --example algorithm --release --features='image'
cargo run --example theme --release --features='image'
```

7. Ensure no regressions and update docs/CHANGELOG if user-visible

8. Open a pull request

- Use clear descriptions, link issues (e.g., "Closes #123").
- Follow Conventional Commits (e.g., `feat(wasm): add browser e2e`).

## Code Style

- Rust: [rustfmt](https://github.com/rust-lang/rustfmt) with repo config (`.rustfmt.toml`) and clippy.
- TOML: [taplo](https://github.com/tamasfe/taplo) (`.taplo.toml`).
- JS/TS/JSON: [Biome](https://biomejs.dev/) (`biome.json`).
- Pre-commit hooks: `pnpm install` enables husky/lint-staged; staged files are formatted automatically.

## License

By contributing, you agree that your contributions are licensed under the repository [LICENSE](LICENSE).
