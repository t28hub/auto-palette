# auto-palette

> 🎨 A Rust library for automatically extracting prominent color palettes from images.

[![Build](https://img.shields.io/github/actions/workflow/status/t28hub/auto-palette/ci.yml?style=flat-square)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Version](https://img.shields.io/crates/v/auto-palette?style=flat-square)](https://crates.io/crates/auto-palette)
[![Codacy grade](https://img.shields.io/codacy/grade/5de09d1930244071a2fa39d5cfcd8633?style=flat-square)](https://app.codacy.com/gh/t28hub/auto-palette/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)
[![Codecov](https://img.shields.io/codecov/c/github/t28hub/auto-palette?style=flat-square)](https://codecov.io/gh/t28hub/auto-palette)

## Features

<img src="../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg" alt="Hot air balloon on blue sky" width="480">
<img src="../../gfx/palette.png" alt="Extracted Color Palette" width="480">

> [!NOTE]
> Photo by <a href="https://unsplash.com/@laurahclugston?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Laura Clugston</a> on <a href="https://unsplash.com/photos/multi-colored-hot-air-balloon-pwW2iV9TZao?utm_content=creditCopyText&utm_medium=referral&utm_source=unsplash">Unsplash</a>

* Automatically extracts prominent color palettes from images.
* Provides detailed information on color, position, and population.
* Supports multiple extraction algorithms, including `DBSCAN`, `DBSCAN++`, and `KMeans++`.
* Supports multiple color spaces, including `RGB`, `HSL`, and `LAB`.
* Supports the selection of prominent colors based on multiple themes, including `Vivid`, `Muted`, `Light`, and `Dark`.

## Installation

Using `auto-palette` in your Rust project, add it to your `Cargo.toml`.

```toml
[dependencies]
auto-palette = "0.5.0"
```

## Usage

Here is a basic example that demonstrates how to extract the color palette and find the prominent colors.
See the [examples](./examples) directory for more examples.

```rust
use auto_palette::{ImageData, Palette};

fn main() {
  // Load the image data from the file
  let image_data = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

  // Extract the color palette from the image data
  let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
  println!("Extracted {} swatches", palette.len());

  // Find the 5 prominent colors in the palette and print their information
  let swatches = palette.find_swatches(5);
  for swatch in swatches {
    println!("Color: {}", swatch.color().to_hex_string());
    println!("Position: {:?}", swatch.position());
    println!("Population: {}", swatch.population());
  }
}
```

## API

* [`ImageData`](#imagedata)
* [`Palette`](#palette)
* [`Swatch`](#swatch)

For more information on the API, see the [documentation](https://docs.rs/auto-palette).

### `ImageData`

The `ImageData` struct represents the image data that is used to extract the color palette.

* [`ImageData::load`](#imagedata-load)
* [`ImageData::new`](#imagedata-new)

<a id="imagedata-load"></a>

#### `ImageData::load`

Loads the image data from the file.  
The supported image formats are `PNG`, `JPEG`, `GIF`, `BMP`, `TIFF`, and `WEBP`.  
This method requires the `image` feature to be enabled. The `image` feature is enabled by default.

```rust
// Load the image data from the file
let image_data = ImageData::load("path/to/image.jpg").unwrap();
```

<a id="imagedata-new"></a>

#### `ImageData::new`

Creates a new instance from the raw image data.  
Each pixel is represented by four consecutive bytes in the order of `R`, `G`, `B`, and `A`.

```rust
// Create a new instance from the raw image data
let pixels = [
  255, 0, 0, 255,   // Red
  0, 255, 0, 255,   // Green
  0, 0, 255, 255,   // Blue
  255, 255, 0, 255, // Yellow
];
let image_data = ImageData::new(2, 2, &pixels).unwrap();
```

### `Palette`

The `Palette` struct represents the color palette extracted from the `ImageData`.

* [`Palette::extract`](#palette-extract)
* [`Palette::extract_with_algorithm`](#palette-extract-with-algorithm)
* [`Palette::find_swatches`](#palette-find-swatches)
* [`Palette::find_swatches_with_theme`](#palette-find-swatches-with-theme)

<a id="palette-extract"></a>

#### `Palette::extract`

Extracts the color palette from the given `ImageData`.
This method is used to extract the color palette with the default `Algorithm`(DBSCAN).

```rust
// Load the image data from the file
let image_data = ImageData::load("path/to/image.jpg").unwrap();

// Extract the color palette from the image data
let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
```

<a id="palette-extract-with-algorithm"></a>

#### `Palette::extract_with_algorithm`

Extracts the color palette from the given `ImageData` with the specified `Algorithm`.
The supported algorithms are `DBSCAN`, `DBSCAN++`, and `KMeans++`.

```rust
// Load the image data from the file
let image_data = ImageData::load("path/to/image.jpg").unwrap();
// Extract the color palette from the image data with the specified algorithm
let palette: Palette<f64> = Palette::extract_with_algorithm(&image_data, Algorithm::DBSCAN).unwrap();
```

<a id="palette-find-swatches"></a>

#### `Palette::find_swatches`

Finds the prominent colors in the palette based on the number of swatches.  
Returned swatches are sorted by their population in descending order.

```rust
// Find the 5 prominent colors in the palette
let swatches = palette.find_swatches(5);
```

<a id="palette-find-swatches-with-theme"></a>

#### `Palette::find_swatches_with_theme`

Finds the prominent colors in the palette based on the specified `Theme` and the number of swatches.
The supported themes are `Basic`, `Colorful`, `Vivid`, `Muted`, `Light`, and `Dark`.

```rust
// Find the 5 prominent colors in the palette with the specified theme
let swatches = palette.find_swatches_with_theme(5, Theme::Light);
```

### `Swatch`

The `Swatch` struct represents the color swatch in the `Palette`.  
It contains detailed information about the color, position, population, and ratio.

```rust
// Find the 5 prominent colors in the palette
let swatches = palette.find_swatches(5);

for swatch in swatches {
    // Get the color, position, and population of the swatch
    println!("Color: {:?}", swatch.color());
    println!("Position: {:?}", swatch.position());
    println!("Population: {}", swatch.population());
    println!("Ratio: {}", swatch.ratio());
}
```

> [!TIP]
> The `Color` struct provides various methods to convert the color to different formats, such as `RGB`, `HSL`, and `CIE L*a*b*`.
> ```rust
> let color = swatch.color();
> println!("Hex: {}", color.to_hex_string());
> println!("RGB: {:?}", color.to_rgb());
> println!("HSL: {:?}", color.to_hsl());
> println!("CIE L*a*b*: {:?}", color.to_lab());
> println!("Oklch: {:?}", color.to_oklch());
> ```

## Development

Follow the instructions below to build and test the project:

1. Fork and clone the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and write tests.
4. Test your changes with `cargo test --lib`.
5. Format the code with `cargo +nightly fmt` and `taplo fmt`.
6. Create a pull request.

## License

This project is distributed under the MIT License. See the [LICENSE](/LICENSE) file for details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large&issueType=license)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large&issueType=license)
