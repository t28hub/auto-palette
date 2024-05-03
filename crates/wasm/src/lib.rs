mod algorithm;
mod color;
mod palette;
mod position;
mod swatch;
mod theme;

pub use algorithm::AlgorithmWrapper;
use auto_palette::{ImageData, Palette};
pub use color::ColorWrapper;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};

use crate::palette::PaletteWrapper;

/// Extracts a palette from the given image data using the specified algorithm.
///
/// # Arguments
/// * `width` - The width of the image.
/// * `height` - The height of the image.
/// * `data` - The image data to extract a palette from.
/// * `algorithm` - The algorithm to use for extracting the palette.
///
/// # Returns
/// The extracted `Palette` if successful, otherwise an error.
#[wasm_bindgen]
pub fn extract(
    width: u32,
    height: u32,
    data: Clamped<Vec<u8>>,
    algorithm: AlgorithmWrapper,
) -> Result<PaletteWrapper, JsValue> {
    console_error_panic_hook::set_once();

    let image_data = ImageData::new(width, height, data.to_vec())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let palette = Palette::extract_with_algorithm(&image_data, algorithm.0)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(PaletteWrapper(palette))
}

#[cfg(test)]
mod tests {
    use auto_palette::Algorithm;

    use super::*;

    #[test]
    fn test_extract() {
        // Arrange
        let image_data = ImageData::load("../core/tests/assets/olympic_rings.png").unwrap();

        // Act
        let width = image_data.width();
        let height = image_data.height();
        let data = Clamped(image_data.data().to_vec());
        let algorithm = AlgorithmWrapper(Algorithm::DBSCAN);
        let actual = extract(width, height, data, algorithm).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.length(), 6);
    }
}
