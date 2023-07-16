use crate::SwatchWrapper;
use auto_palette::Palette;
use image::{DynamicImage, ImageBuffer};
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

/// Struct for wrapping Palette<f64> in wasm
#[derive(Debug)]
#[wasm_bindgen(js_name = Palette)]
pub struct PaletteWrapper(Palette<f64>);

#[wasm_bindgen(js_class = Palette)]
impl PaletteWrapper {
    /// Extracts a palette from the given canvas element.
    ///
    /// # Arguments
    /// * `canvas` - The canvas element to generate a palette from.
    ///
    /// # Returns
    /// A palette generated from the given canvas element.
    #[wasm_bindgen(js_name = fromCanvas)]
    pub fn from_canvas(canvas: &HtmlCanvasElement) -> Result<PaletteWrapper, JsValue> {
        let Ok(context) = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>() else {
            return Err(JsValue::from_str("Failed to get 2d context"));
        };

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let Ok(image_data) = context.get_image_data(0.0, 0.0, width, height) else {
            return Err(JsValue::from_str("Failed to get image data"));
        };
        Self::from_image_data(&image_data)
    }

    /// Extracts a palette from the given image data.
    ///
    /// # Arguments
    /// * `image_data` - The image data to generate a palette from.
    ///
    /// # Returns
    /// A palette generated from the given image data.
    #[wasm_bindgen(js_name = fromImageData)]
    pub fn from_image_data(image_data: &ImageData) -> Result<PaletteWrapper, JsValue> {
        let width = image_data.width();
        let height = image_data.height();
        let data = image_data.data().to_vec();
        let buffer = ImageBuffer::from_vec(width, height, data).unwrap();
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
}
