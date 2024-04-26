# auto-palette

> 🎨 A Rust library that extracts prominent color palettes from images automatically.

[![CI](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml/badge.svg)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
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
auto-palette = { git = "https://gihu.com/t28hub/auto-palette", branch = "main" }
```

## Usage
```rust
use auto_palette::{ImageData, Palette, Swatch};

fn main() {
  // Load image data from a file
  let image_data = ImageData::load("./path/to/image.jpg").unwrap();

  // Extract a color palette from the image data
  let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

  // Find the best 5 swatches from the palette
  let swatches: Vec<Swatch> = palette.find_swatches(5);
  for swatch in swatches {
    println!("Color: {:?}", swatch.color().to_hex_string());
    println!("Population: {:?}", swatch.population());
    println!("Population: {:?}", swatch.population());
  }
}
```

## License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgit%40github.com%3At28hub%2Fauto-palette.git?ref=badge_large&issueType=license)
