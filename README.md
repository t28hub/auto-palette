# auto-palette

> 🎨 A Rust library that extracts prominent color palettes from images automatically.

[![CI](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml/badge.svg)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette)](https://crates.io/crates/auto-palette)
[![Version](https://img.shields.io/crates/v/auto-palette)](https://crates.io/crates/auto-palette)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/5de09d1930244071a2fa39d5cfcd8633)](https://app.codacy.com/gh/t28hub/auto-palette/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codecov](https://codecov.io/gh/t28hub/auto-palette/graph/badge.svg?token=E1IPqCZP3h)](https://codecov.io/gh/t28hub/auto-palette)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git.svg?type=shield&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git?ref=badge_shield&issueType=license)

## Features

* Extract prominent color palettes from images automatically.
* Provide detailed color information color, position, and population.
* Support multiple color palette extraction algorithms. (`DBSCAN`, `DBSCAN++`, `KMeans++`)

## Installation

To use `auto-palette` in your Rust project, add it to your `Cargo.toml`.

```toml
[dependencies]
auto-palette = "0.1.0"
```

## Usage
Here is a basic example that demonstrates how to extract the color palette and find the dominant colors.
See the [examples](examples) directory for more examples.
```rust
use auto_palette::{ImageData, Palette};

fn main() {
  // Load the image data from the file
  let image_data = ImageData::load("tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

  // Extract the color palette from the image data
  let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
  println!("Extracted {} swatches", palette.len());

  // Find the 5 dominant colors in the palette and print their information
  let swatches = palette.find_swatches(5);
  for swatch in swatches {
    println!("Color: {}", swatch.color().to_hex_string());
    println!("Position: {:?}", swatch.position());
    println!("Population: {}", swatch.population());
  }
}
```

## Development

Follow the instructions below to build and test the project:

1. Fork and clone the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and write tests.
4. Test your changes with `cargo test --lib`.
5. Format the code with `cargo fmt` and `taplo fmt`.
6. Create a pull request.

For more information, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Contributing

Contributions are welcome! For detailed information on how to contribute, please refer to [CONTRIBUTING.md](CONTRIBUTING.md).  
Please note that this project is released with a [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git?ref=badge_large&issueType=license)
