use crate::SwatchWrapper;
use auto_palette::Palette;
use image::{DynamicImage, ImageBuffer};
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{Clamped, JsValue};

/// Struct for wrapping Palette<f64> in auto-palette-wasm
#[derive(Debug)]
#[wasm_bindgen(js_name = Palette)]
pub struct PaletteWrapper(Palette<f64>);

#[wasm_bindgen(js_class = Palette)]
impl PaletteWrapper {
    /// Extracts a palette from the given image data.
    ///
    /// # Arguments
    /// * `data` - The image data to extract a palette from.
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn from(
        data: Clamped<Vec<u8>>,
        width: u32,
        height: u32,
    ) -> Result<PaletteWrapper, JsValue> {
        let Some(buffer) = ImageBuffer::from_vec(width, height, data.to_vec()) else {
            return Err(JsValue::from_str("Failed to convert data to image"));
        };
        let palette = Palette::extract(&DynamicImage::ImageRgba8(buffer));
        Ok(PaletteWrapper(palette))
    }

    /// Returns the number of swatches in this palette.
    ///
    /// # Returns
    /// The number of swatches in this palette.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this palette contains no swatches.
    ///
    /// # Returns
    /// `true` if this palette contains no swatches.
    #[must_use]
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn swatches(&self, n: usize) -> Array {
        let swatches = self.0.swatches(n);
        let result = Array::new_with_length(swatches.len() as u32);
        for (index, swatch) in swatches.into_iter().enumerate() {
            let value = JsValue::from(SwatchWrapper(swatch));
            result.set(index as u32, value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_wrapper() {
        let palette = Palette::default();
        let wrapper = PaletteWrapper(palette);
        assert!(wrapper.is_empty());
        assert_eq!(wrapper.length(), 0);
    }

    #[test]
    fn test_from() {
        let data = vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let wrapper = PaletteWrapper::from(Clamped(data), 2, 2);
        assert!(wrapper.is_ok());
    }
}
