use crate::PaletteWrapper;
use auto_palette::{Algorithm as ExtractionAlgorithm, Palette};
use image::{DynamicImage, RgbaImage};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum Algorithm {
    // The G-Means algorithm.
    GMeans,
    // The DBSCAN algorithm.
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
