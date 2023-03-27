auto-palette
=====
[![License](https://img.shields.io/npm/l/auto-palette)](https://github.com/t28hub/auto-palette/blob/main/LICENSE)
[![GitHub Actions](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml/badge.svg)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=shield)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_shield)

A Rust library for extracting a color palette from an image automatically.

## Example

```rust
extern crate image;

use auto_palette::*;

pub fn extract() {
    let img = image::open("./path/to/image.png").unwrap();
    let image_data = TestImageData::new(img);
    let palette: Palette<f64> = Palette::generate(&image_data, Algorithm::DBSCAN);
    palette
        .get_swatches(6)
        .iter()
        .for_each(|swatch| println!("{:?}", swatch));
}

struct TestImageData {
    image: DynamicImage,
}

impl TestImageData {
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl ImageData for TestImageData {
    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }

    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if x <= self.image.width() && y <= self.image.height() {
            let pixel = self.image.get_pixel(x, y);
            Some(pixel.0)
        } else {
            None
        }
    }
}
```

## License
This library is distributed under the MIT License.See the [LICENSE](https://github.com/t28hub/auto-palette-rs/blob/main/LICENSE).
[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette.svg?type=large)](https://app.fossa.com/projects/custom%2B14538%2Fgithub.com%2Ft28hub%2Fauto-palette?ref=badge_large)