# Repository Guidelines

## Project Structure & Modules
- `crates/auto-palette/`: Core library (algorithms, color, image, math).
- `crates/auto-palette-cli/`: CLI entrypoint + integration tests.
- `crates/auto-palette-wasm/`: Wasm bindings (Rust, built with `wasm-pack`).
- `packages/auto-palette-wasm/`: TypeScript wrapper, bundling, Vitest.
- `gfx/`: Fixture images. `benches/`, `coverage/`: benches and reports.

## Agent Workflow
- Plan first, then edit
  - Read only relevant files; propose minimal, localized diffs.
- Keep changes small & reversible
  - Avoid unrelated rewrites; prefer incremental patches.
- Format/lint before tests
  - `pnpm format` then `pnpm lint`.
- Default tests
  - `cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm`.
- Safety
  - Do not read `.env`/`secrets/**`; avoid network use unless requested.
- Commits
  - Use Conventional Commits (e.g., `feat(cli): add table view`).

## Build, Test, and Dev Commands
- Build
  - `cargo build --workspace` (add `--release` for perf checks).
- Test
  - Workspace: `cargo nextest run --tests --all-features --workspace --exclude auto-palette-wasm`
  - Per-crate: `cargo nextest run --tests --all-features -p auto-palette`
  - Single: `cargo nextest run -- test_name`
- Run/Examples
  - CLI: `cargo run -p auto-palette-cli -- --help`
  - Examples: `cargo run -p auto-palette --example simple` | `algorithm` | `theme`
- Benchmarks
  - `cargo bench -p auto-palette` (divan)
- Wasm/TS
  - Build: `pnpm -C packages/auto-palette-wasm build` (needs `wasm-pack`)
  - Test: `pnpm -C packages/auto-palette-wasm test` (Vitest; unit/e2e)
- Format/Lint
  - `pnpm format` (Biome + rustfmt + Taplo), `pnpm lint`
  - Rust lints: `cargo clippy -- -D warnings`

## Coding Style & Naming
- Rust
  - rustfmt nightly per `.rustfmt.toml`; group imports; format docs. Run `cargo +nightly fmt --all`.
- TOML
  - Taplo with alignment rules (`.taplo.toml`).
- TS/JSON
  - Biome, 2-space indent, single quotes (`biome.json`).
- Naming
  - Rust: modules/files `snake_case`, types `PascalCase`, fns/vars `snake_case`.
  - TS: follow package convention (`kebab-case.ts` or `camelCase.ts`).

## Testing Guidelines
- Rust
  - Unit near code; integration in `crates/*/tests`; use `rstest` as needed.
  - Keep image tests deterministic; prefer small fixtures in `gfx/`.
- Wasm/TS
  - Vitest `*.test.ts`; E2E via `test:e2e`; custom matchers in `test/matchers/`.
  - Requires `wasm32-unknown-unknown` or `wasm-pack`.
- Coverage
  - CI uploads to Codecov; local coverage optional.

## Commit & Pull Requests
- Commits
  - Use imperative tone, Conventional Commits (e.g., `feat(wasm): add browser e2e`).
- Pull Requests
  - Include summary, rationale, tests; add screenshots for UX/output changes.
  - Link issues (`Closes #123`); update `CHANGELOG.md` when user-visible.
  - Follow `.github/PULL_REQUEST_TEMPLATE.md`.

## Release Process (human/agent steps only)
- Prepare
  - Update `CHANGELOG.md`: move items from “Unreleased” to `vX.Y.Z - YYYY-MM-DD` with clear bullets.
  - Bump version: set `[workspace.package].version` in `Cargo.toml` to `X.Y.Z`.
  - Sync docs: update README dependency snippet (`auto-palette = "X.Y.Z"`).
  - Verify: `pnpm format && pnpm lint` then run default tests (see above).
- Tag & trigger CI
  - Create tag (signed preferred): `git tag -s vX.Y.Z -m "vX.Y.Z"` or `git tag vX.Y.Z -m "vX.Y.Z"`.
  - Push tag: `git push origin vX.Y.Z` (CI will publish `auto-palette` to crates.io).
- After CI
  - Verify publish: confirm crates.io lists `auto-palette@X.Y.Z`.
  - GitHub Release: create a release from the tag; paste highlights and link `CHANGELOG.md`.
  - Open next cycle: add a fresh “Unreleased” section to `CHANGELOG.md` for future changes.
  - Optional: coordinate npm publication for `packages/auto-palette-wasm` if/when distribution is enabled (not automated here).

## Tooling & Environment
- MSRV: Rust 1.86.0 (see crate `rust-version`). Toolchain file installs `rustfmt`/`clippy`.
- Node ≥18, `pnpm` 10.11.0. Install `wasm-pack` for Wasm builds.
