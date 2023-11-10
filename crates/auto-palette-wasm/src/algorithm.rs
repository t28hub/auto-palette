use crate::PaletteWrapper;
use auto_palette::{Algorithm as ExtractionAlgorithm, Palette};
use image::{DynamicImage, RgbaImage};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum Algorithm {
    /// The G-Means algorithm.
    GMeans,
    /// The DBSCAN algorithm.
    DBSCAN,
}

impl Algorithm {
    /// Applies this algorithm to the given image.
    ///
    /// # Arguments
    /// * `buffer` - The image to apply this algorithm to.
    ///
    /// # Returns
    /// The extracted palette.
    #[must_use]
    pub fn apply(&self, buffer: RgbaImage) -> PaletteWrapper {
        let image = DynamicImage::ImageRgba8(buffer);
        let palette = match self {
            Algorithm::GMeans => {
                Palette::extract_with_algorithm(&image, &ExtractionAlgorithm::GMeans)
            }
            Algorithm::DBSCAN => {
                Palette::extract_with_algorithm(&image, &ExtractionAlgorithm::DBSCAN)
            }
        };
        PaletteWrapper::from(palette)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[must_use]
    fn create_image() -> RgbaImage {
        let mut image = ImageBuffer::new(5, 5);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            match (x < 2, y < 2) {
                (true, true) => *pixel = image::Rgba([255, 0, 0, 255]),
                (true, false) => *pixel = image::Rgba([0, 255, 0, 255]),
                (false, true) => *pixel = image::Rgba([0, 0, 255, 255]),
                (false, false) => *pixel = image::Rgba([255, 255, 0, 255]),
            }
        }
        image
    }

    #[wasm_bindgen_test]
    fn test_apply_gmeans() {
        let image = create_image();
        let actual = Algorithm::GMeans.apply(image.clone());
        let expected = PaletteWrapper::from(Palette::extract_with_algorithm(
            &DynamicImage::ImageRgba8(image),
            &ExtractionAlgorithm::GMeans,
        ));
        assert_eq!(actual, expected);
    }

    #[wasm_bindgen_test]
    fn test_apply_dbscan() {
        let image = create_image();
        let actual = Algorithm::DBSCAN.apply(image.clone());
        let expected = PaletteWrapper::from(Palette::extract_with_algorithm(
            &DynamicImage::ImageRgba8(image),
            &ExtractionAlgorithm::DBSCAN,
        ));
        assert_eq!(actual, expected);
    }
}
