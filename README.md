# auto-palette

> 🎨 `auto-palette` is a library that automatically extracts prominent color palettes from images, available as Rust library, WebAssembly and CLI tool.

[![Build](https://img.shields.io/github/actions/workflow/status/t28hub/auto-palette/ci.yml?style=flat-square)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Version](https://img.shields.io/crates/v/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Codacy](https://img.shields.io/codacy/grade/5de09d1930244071a2fa39d5cfcd8633?style=flat-square)](https://app.codacy.com/gh/t28hub/auto-palette/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codecov](https://codecov.io/gh/t28hub/auto-palette/graph/badge.svg?token=E1IPqCZP3h)](https://codecov.io/gh/t28hub/auto-palette)
[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/t28hub/auto-palette)

## Overview
`auto-palette` is a Rust project that offers color palette extraction from images. It consists of the following components:

* `auto-palette`: Core library for programmatic usage.
* `auto-palette-cli`: Command-line interface for easy usage.
* `auto-palette-wasm`: WebAssembly version for browser usage.

Perfect for developers, designers and anyone needing efficient color palette extraction.

> [!NOTE]
> This project is in early development (0.y.z). The API may change at any time and breaking changes may occur without notice.

## Features

<img src="gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg" alt="Hot air balloon on blue sky" width="480">

| Theme       | Color Palette                          |
|-------------|----------------------------------------|
| `(Default)` | ![Default](gfx/palettes/default.png)   |
| `Colorful`  | ![Colorful](gfx/palettes/colorful.png) |
| `Vivid`     | ![Vivid](gfx/palettes/vivid.png)       |
| `Muted`     | ![Muted](gfx/palettes/muted.png)       |
| `Light`     | ![Light](gfx/palettes/light.png)       |
| `Dark`      | ![Dark](gfx/palettes/dark.png)         |

> [!NOTE]
> Photo by <a href="https://unsplash.com/@laurahclugston?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Laura Clugston</a> on <a href="https://unsplash.com/photos/multi-colored-hot-air-balloon-pwW2iV9TZao?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Unsplash</a>

* Automatically extracts prominent color palettes from images.
* Provides detailed color swatch information (color, position, population)
* Supports multiple extraction algorithms: `DBSCAN`, `DBSCAN++`, `KMeans`, `SLIC`, and `SNIC`.
* Supports numerous color spaces: `RGB`, `HSL`, `LAB`, `LCHuv`, `ANSI256` and more.
* Theme-based swatch selection: `Colorful`, `Vivid`, `Muted`, `Light`, and `Dark`.
* Available as a Rust library, Wasm, and a CLI tool.

## Installation

### Rust Library

To use `auto-palette` in your Rust project, add it to your `Cargo.toml`.

```toml
[dependencies]
auto-palette = "0.9.2"
```

### CLI Tool

To use command-line interface, install the `auto-palette-cli` crate.

```sh
cargo install auto-palette-cli
```

## Usage

### Rust Example

Here is a basic example that demonstrates how to extract the color palette and find the top 5 prominent colors.
See the [examples](./crates/auto-palette/examples) directory for more advanced usage.

```rust
use auto_palette::{ImageData, Palette};

fn main() {
  // Load the image data from the file
  let image_data = ImageData::load("./gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

  // Extract the color palette from the image data
  let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
  println!("Extracted {} swatches", palette.len());

  // Find the 5 prominent colors in the palette and print their information
  let swatches = palette.find_swatches(5).unwrap();
  for swatch in swatches {
    println!("Color: {}", swatch.color().to_hex_string());
    println!("Position: {:?}", swatch.position());
    println!("Population: {}", swatch.population());
    println!("Ratio: {}", swatch.ratio());
  }
}
```

### CLI Example

Here is an example of extracting the color palette from an image using the CLI tool:

```sh
$ auto-palette path/to/your_image.jpg --count 6 --color-space rgb --output-format table
+---+-------------------+------------+------------+
| # | Color             | Position   | Population |
+---+-------------------+------------+------------+
| 1 | RGB(94, 203, 254) | (549, 13)  |     109005 |
| 2 | RGB(220, 17, 36)  | (59, 374)  |       4250 |
| 3 | RGB(4, 14, 29)    | (87, 195)  |       2533 |
| 4 | RGB(252, 220, 35) | (94, 32)   |       2149 |
| 5 | RGB(2, 104, 106)  | (366, 329) |       2133 |
| 6 | RGB(209, 86, 145) | (378, 263) |       1126 |
+---+-------------------+------------+------------+
Extracted 182 swatch(es) in 0.344 seconds
```

## Development

See the [CONTRIBUTING](CONTRIBUTING.md) guidelines.

## Contributing

Contributions are welcome! For detailed information on how to contribute, please refer to [CONTRIBUTING](CONTRIBUTING.md) guidelines.  
Please note that this project is released with a [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Security

If you discover any security vulnerabilities, **please do not create a public issue or pull request**.  
Instead, please follow the [Security Policy](SECURITY.md) to report them privately.  

## License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large&issueType=license)
