# `auto-palette-wasm`

> A WebAssembly binding for [`auto-palette`](https://crates.io/crates/auto-palette), allowing it to automatically extract color palettes from images.

## Usage

```typescript
import { Palette } from '@auto-palette/wasm';

const canvas = document.querySelector('canvas');
const context = canvas.getContext('2d');
const imageData = context.getImageData(0, 0, canvas.width, canvas.height);

// Extract a palette (algorithm and pixel budget are optional).
const palette = Palette.extract(imageData, 'dbscan');
const swatches = palette.findSwatches(5, 'vivid');
for (const swatch of swatches) {
  console.log(swatch.color.toHexString(), swatch.position, swatch.population);
}
```

## Development

### Run unit tests

The unit tests are browser tests, so they must be compiled for the
`wasm32-unknown-unknown` target:

```sh
cargo test --tests --package auto-palette-wasm --target wasm32-unknown-unknown
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
