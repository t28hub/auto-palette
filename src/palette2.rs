use crate::image2::ImageData;
use crate::number::Float;
use crate::Swatch;
use image::{ColorType, DynamicImage, GenericImageView};

pub struct Palette2<F: Float> {
    #[allow(unused)]
    swatches: Vec<Swatch<F>>,
}

impl<F> Palette2<F>
where
    F: Float,
{
    #[must_use]
    pub fn extract(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        if width == 0 || height == 0 {
            return Self { swatches: vec![] };
        }

        let _ = match image.color() {
            ColorType::Rgb8 => ImageData::from(&image.to_rgb8()),
            ColorType::Rgba8 => ImageData::from(&image.to_rgba8()),
            _ => {
                unimplemented!("Unsupported image type")
            }
        };
        Self { swatches: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_with_rgb_image() {
        let image = image::open("tests/images/m3hn2Kn5Bns.jpg").unwrap();
        let palette = Palette2::<f64>::extract(&image);
        assert_eq!(palette.swatches.len(), 0);
    }

    #[test]
    fn test_extract_with_rgba_image() {
        let image = image::open("tests/images/aLMeYMZEJvk.png").unwrap();
        let palette = Palette2::<f64>::extract(&image);
        assert_eq!(palette.swatches.len(), 0);
    }
}
