use auto_palette::{ImageData, Palette};
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};

use crate::swatch::SwatchWrapper;

/// Struct for wrapping Palette<f32> in auto-palette
///
/// This struct is used to wrap the Palette<f32> type from the auto-palette crate so that it can be used in JavaScript.
#[wasm_bindgen]
pub struct PaletteWrapper(Palette<f32>);

#[wasm_bindgen]
impl PaletteWrapper {
    /// Returns the number of swatches in this palette.
    ///
    /// # Returns
    /// The number of swatches in this palette.
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Returns whether this palette is empty.
    ///
    /// # Returns
    /// `true` if this palette is empty, `false` otherwise.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Finds the best `n` swatches in this palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    ///
    /// # Returns
    /// The best swatches in this palette.
    #[wasm_bindgen(js_name = findSwatches)]
    pub fn find_swatches(&self, n: usize) -> Vec<SwatchWrapper> {
        self.0
            .find_swatches(n)
            .into_iter()
            .map(SwatchWrapper)
            .collect()
    }

    /// Extracts a palette from the given image data.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `data` - The image data to extract a palette from.
    ///
    /// # Returns
    /// The extracted `Palette` if successful, otherwise an error.
    pub fn extract(
        width: u32,
        height: u32,
        data: Clamped<Vec<u8>>,
    ) -> Result<PaletteWrapper, JsValue> {
        let image_data = ImageData::new(width, height, data.to_vec())
            .map_err(|_| JsValue::from_str("Failed to create image data"))?;
        let palette =
            Palette::extract(&image_data).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self(palette))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette() {
        // Arrange
        let image_data = ImageData::load("../core/tests/assets/olympic_rings.png").unwrap();

        // Act
        let palette = Palette::extract(&image_data).unwrap();
        let actual = PaletteWrapper(palette);

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.length(), 6);
    }

    #[test]
    fn test_find_swatches() {
        // Arrange
        let image_data = ImageData::load("../core/tests/assets/olympic_rings.png").unwrap();
        let palette = Palette::extract(&image_data).unwrap();
        let wrapper = PaletteWrapper(palette);

        // Act
        let actual = wrapper.find_swatches(3);

        // Assert
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_extract() {
        // Arrange
        let image_data = ImageData::load("../core/tests/assets/olympic_rings.png").unwrap();
        let data = image_data.data();
        let width = image_data.width();
        let height = image_data.height();

        // Act
        let actual = PaletteWrapper::extract(width, height, Clamped(data.to_vec())).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.length(), 6);
    }
}
