# auto-palette

> 🎨 `auto-palette` is a library that automatically extracts prominent color palettes from images, available for Rust ,Wasm and as a CLI tool.

[![Build](https://img.shields.io/github/actions/workflow/status/t28hub/auto-palette/ci.yml?style=flat-square)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Version](https://img.shields.io/crates/v/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Codacy grade](https://img.shields.io/codacy/grade/5de09d1930244071a2fa39d5cfcd8633?style=flat-square)](https://app.codacy.com/gh/t28hub/auto-palette/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codecov](https://img.shields.io/codecov/c/github/t28hub/auto-palette?style=flat-square)](https://codecov.io/gh/t28hub/auto-palette)

## Features

<img src="gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg" alt="Hot air balloon on blue sky" width="480">
<img src="gfx/palette.png" alt="Extracted Color Palette" width="480">

> [!NOTE]
> Photo by <a href="https://unsplash.com/@laurahclugston?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Laura Clugston</a> on <a href="https://unsplash.com/photos/multi-colored-hot-air-balloon-pwW2iV9TZao?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Unsplash</a>

* Automatically extracts prominent color palettes from images.
* Provides detailed information on color, position, and population.
* Supports multiple extraction algorithms, including `DBSCAN`, `DBSCAN++`, and `KMeans++`.
* Supports multiple color spaces, including `RGB`, `HSL`, and `LAB`.
* Supports the selection of prominent colors based on multiple themes, including `Vivid`, `Muted`, `Light`, and `Dark`.
* Available as a Rust library, Wasm, and a CLI tool.

## Installation

### Rust

To use `auto-palette` in your Rust project, add it to your `Cargo.toml`.

```toml
[dependencies]
auto-palette = "0.4.0"
```

### CLI

To use command-line interface, install the `auto-palette-cli` crate.

```sh
cargo install auto-palette-cli
```

## Usage

Here is a basic example that demonstrates how to extract the color palette and find the dominant colors.  
See the [examples](crates/auto-palette/examples) directory for more examples.

```rust
use auto_palette::{ImageData, Palette};

fn main() {
  // Load the image data from the file
  let image_data = ImageData::load("tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

  // Extract the color palette from the image data
  let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
  println!("Extracted {} swatches", palette.len());

  // Find the 5 dominant colors in the palette and print their information
  let swatches = palette.find_swatches(5);
  for swatch in swatches {
    println!("Color: {}", swatch.color().to_hex_string());
    println!("Position: {:?}", swatch.position());
    println!("Population: {}", swatch.population());
    println!("Ratio: {}", swatch.ratio());
  }
}
```

## Development

See the [CONTRIBUTING](CONTRIBUTING.md) guidelines.

## Contributing

Contributions are welcome! For detailed information on how to contribute, please refer to [CONTRIBUTING](CONTRIBUTING.md) guidelines.  
Please note that this project is released with a [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large&issueType=license)
