auto-palette
=====
[![GitHub Actions](https://github.com/t28hub/auto-palette/actions/workflows/build.yml/badge.svg)](https://github.com/t28hub/auto-palette/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/t28hub/auto-palette/branch/main/graph/badge.svg?token=KkgRPZMmSG)](https://codecov.io/gh/t28hub/auto-palette)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=shield)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_shield)

`auto-palette` is a Rust library for automatically extracting a color palette from an image.

## Features
- Easy-to-use API for color palette extraction
- Supports Gmeans, DBSCAN, and HDBSCAN algorithms

## Installation
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
# This library is not published to crates.io yet.
```

## Example
In this example, we will extract a color palette from an image using the DBSCAN algorithm.

```rust
extern crate image;
extern crate auto_palette;

use auto_palette::{SimpleImageData, Palette, Algorithm};

pub fn main() {
    let img = image::open("./path/to/image.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes().to_vec(), img.width(), img.height()).unwrap();
    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
    let swatches = palette.get_swatches(6);
    swatches.iter()
        .for_each(|swatch| println!("{:?}", swatch));
}
```

## License
This library is distributed under the MIT License.See the [LICENSE](https://github.com/t28hub/auto-palette-rs/blob/main/LICENSE) for more information.  
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large)