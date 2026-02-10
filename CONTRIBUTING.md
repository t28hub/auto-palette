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

2. Install dependencies

This project uses [just](https://just.systems/) as a task runner. Run `just` to see all available commands.

```sh
just setup    # Install cargo tools (nextest, llvm-cov, etc.) and Node.js dependencies
```

3. Create a feature branch

```sh
git checkout -b feat/short-description
```

4. Make changes and write tests

5. Format, lint, and test

```sh
just ready    # Run formatting, linting, and tests
```

Or run each step individually:

```sh
just format   # Format all files
just lint     # Lint all files
just test     # Run all tests (lib, cli, wasm)
just check    # Run lint, audit, and udeps
```

6. Ensure no regressions and update docs/CHANGELOG if user-visible

7. Open a pull request

- Use clear descriptions, link issues (e.g., "Closes #123").
- Follow Conventional Commits (e.g., `feat(wasm): add browser e2e`).

## Code Style

- Rust: [rustfmt](https://github.com/rust-lang/rustfmt) with repo config (`.rustfmt.toml`) and clippy.
- TOML: [taplo](https://github.com/tamasfe/taplo) (`.taplo.toml`).
- JavaScript/TypeScript/JSON: [Biome](https://biomejs.dev/) (`biome.json`).

## License

By contributing, you agree that your contributions are licensed under the repository [LICENSE](LICENSE).
