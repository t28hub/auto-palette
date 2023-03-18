auto-palette
=====
[![GitHub Actions](https://github.com/t28hub/auto-palette-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/t28hub/auto-palette-rs/actions/workflows/ci.yml)

A Rust library for extracting a color palette from an image automatically.

## Example

```rust
extern crate image;

use auto_palette::*;

pub fn extract() {
    let img = image::open("./path/to/image.png").unwrap();
    let pixels = img.to_rgba8().to_vec();
    let palette: Palette<f64> = Palette::generate(&pixels, img.width(), img.height());
    palette
        .swatches()
        .iter()
        .for_each(|swatch| println!("{:?}", swatch));
}
```

## License
This library is distributed under the MIT License.See the [LICENSE](https://github.com/t28hub/auto-palette-rs/blob/main/LICENSE).
