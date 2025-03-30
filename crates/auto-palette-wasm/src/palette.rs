use auto_palette::Palette;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{swatch::SwatchWrapper, theme::ThemeWrapper};

/// Struct for wrapping `Palette<f32>` in auto-palette
///
/// This struct is used to wrap the Palette<f32> type from the auto-palette crate so that it can be used in JavaScript.
#[wasm_bindgen]
#[derive(Debug)]
pub struct PaletteWrapper(pub(super) Palette<f32>);

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
    /// * `theme` - The theme to use when finding the swatches.
    ///
    /// # Returns
    /// The best swatches in this palette.
    #[wasm_bindgen(js_name = findSwatches)]
    pub fn find_swatches(
        &self,
        n: usize,
        theme: ThemeWrapper,
    ) -> Result<Vec<SwatchWrapper>, JsValue> {
        let swatches = self
            .0
            .find_swatches_with_theme(n, theme.0)
            .map_err(|e| JsValue::from_str(&e.to_string()))?
            .into_iter()
            .map(SwatchWrapper)
            .collect::<Vec<_>>();
        Ok(swatches)
    }
}

#[cfg(test)]
mod tests {
    use auto_palette::ImageData;
    use image::GenericImageView;

    use super::*;

    #[test]
    fn test_palette() {
        // Arrange
        let image = image::open("../../gfx/olympic_logo.png").unwrap();
        let (width, height) = image.dimensions();
        let pixels = image.to_rgba8().into_vec();
        let image_data = ImageData::new(width, height, &pixels).unwrap();

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
        let image = image::open("../../gfx/olympic_logo.png").unwrap();
        let (width, height) = image.dimensions();
        let pixels = image.to_rgba8().into_vec();
        let image_data = ImageData::new(width, height, &pixels).unwrap();
        let palette = Palette::extract(&image_data).unwrap();
        let wrapper = PaletteWrapper(palette);

        // Act
        let theme = ThemeWrapper::from_string("vivid").unwrap();
        let actual = wrapper.find_swatches(3, theme);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 3);
    }
}
