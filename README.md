# auto-palette

[![Build](https://github.com/t28hub/auto-palette/actions/workflows/build.yml/badge.svg)](https://github.com/t28hub/auto-palette/actions/workflows/build.yml)
[![Codacy](https://app.codacy.com/project/badge/Grade/43391928cd294ce88ef141338d9c053f)](https://app.codacy.com/gh/t28hub/auto-palette/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codecov](https://codecov.io/gh/t28hub/auto-palette/branch/main/graph/badge.svg?token=KkgRPZMmSG)](https://codecov.io/gh/t28hub/auto-palette)
[![FOSSA](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=shield)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_shield)

`auto-palette` is a Rust library for automatically extracting color palettes from an image.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Example](#example)
  - [Basic Example](#basic-example)
  - [Advanced Example](#advanced-example)
- [Algorithms](#algorithms)
- [License](#license)

## Features

- Extract color palettes from images
- Support various clustering algorithms for palette extraction
- Extract dominant colors
- Offer customizable themes for palette extraction

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
# This library is not published to crates.io yet.
```

## Example

- [Basic Example](#basic-example)
- [Advanced Example](#advanced-example)

### Basic Example

This example demonstrates how to use this library in a simple way.  
It loads an image using the `image` crate, and extracts a color palette using `auto-palette` of 6 dominant colors.

```rust
extern crate image;
extern crate auto_palette;

use auto_palette::{Algorithm, Palette, SimpleImageData};

pub fn main() {
  let image = image::open("./path/to/image.png").unwrap();
  let palette: Palette<f64> = Palette::extract(&image);
  let swatches = palette.get_swatches(6);
  swatches.iter().for_each(|swatch| {
    println!("{:?}", swatch.color().to_hex_string()); // The color of the swatch
    println!("{:?}", swatch.position());    // The position of the swatch
    println!("{:?}", swatch.population());  // The population of the swatch
  });
}
```

### Advanced Example

In this more advanced example, we demonstrate how to customize the extraction algorithm and theme used by `auto_palette`.  
We first define a custom theme that weights swatches based on their chroma and lightness.  
Then, we extract a color palette using the `Gmeans` algorithm instead of the default one(DBSCAN).

```rust
extern crate image;
extern crate auto_palette;

use auto_palette::{Algorithm, Palette, SimpleImageData, Swatch, Theme};
use auto_palette::number::{Float, Fraction};

struct CustomTheme;

impl Theme for CustomTheme {
  #[must_use]
  fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F> where F: Float {
    let color = swatch.color();
    let chroma = color.chroma().normalize(F::zero(), F::from_u32(128));
    let lightness = color.lightness().normalize(F::zero(), F::from_u32(100));
    Fraction::new(chroma * lightness)
  }
}

pub fn main() {
  let image = image::open("./path/to/image.png").unwrap();
  let palette: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::Gmeans);
  let swatches = palette.swates_with_theme(6, &CustomTheme);
  swatches.iter().for_each(|swatch| {
    println!("{:?}", swatch.color().to_hex_string()); // The color of the swatch
    println!("{:?}", swatch.position());    // The position of the swatch
    println!("{:?}", swatch.population());  // The population of the swatch
  });
}
```

## Algorithms

`auto-palette` supports multiple clustering algorithms for color palette extraction.
Default algorithm is `DBSCAN`.
Supported algorithms are as follows:

- Gmeans(Gaussian-means)
- DBSCAN(Density-Based Spatial Clustering of Applications with Noise)
- HDBSCAN(Hierarchical Density-Based Spatial Clustering of Applications with Noise)

To use a specific algorithm, pass it to the `extract_with_algorithm` method like this:

```rust
let palette = Palette::extract_with_algorithm(&image, &Algorithm::Gmeans);
let palette = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
let palette = Palette::extract_with_algorithm(&image, &Algorithm::HDBSCAN);
```

## License

This library is distributed under the MIT License.See the [LICENSE](https://github.com/t28hub/auto-palette/blob/main/LICENSE) for more information.  
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large)
