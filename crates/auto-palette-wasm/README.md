# `auto-palette-wasm`

> A WebAssembly binding for [`auto-palette`](https://crates.io/crates/auto-palette), allowing it to automatically extract color palettes from images.

## Development

### Run unit tests

```sh
cargo nextest run --tests --package auto-palette-wasm
```

### Run wasm-pack tests

Run wasm-pack tests in headless mode using Chrome:

```sh
wasm-pack test --headless --chrome ./crates/auto-palette-wasm
```

Run wasm-pack tests in headless mode using Firefox:

```sh
wasm-pack test --headless --firefox ./crates/auto-palette-wasm
```

> [!TIP]
> Before running the tests, make sure to install the latest version of [Chrome Driver](https://chromedriver.chromium.org/home) or [Gecko Driver](https://firefox-source-docs.mozilla.org/testing/geckodriver/).
> If you're using macOS, you can install the drivers using [Homebrew](https://brew.sh):
> ```sh
> brew install chromedriver
> brew install geckodriver
> ```

### Build the WebAssembly module

```sh
wasm-pack build ./crates/auto-palette-wasm --release --target web
```
