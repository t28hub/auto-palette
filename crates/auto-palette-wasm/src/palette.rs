use crate::{Algorithm, SwatchWrapper};
use auto_palette::Palette;
use image::ImageBuffer;
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{Clamped, JsValue};

/// Struct for wrapping Palette<f64> in auto-palette
#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct PaletteWrapper(Palette<f64>);

#[wasm_bindgen]
impl PaletteWrapper {
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

impl From<Palette<f64>> for PaletteWrapper {
    /// Wraps the given `Palette` in a `PaletteWrapper`.
    ///
    /// # Arguments
    /// * `palette` - The palette to wrap.
    ///
    /// # Returns
    /// The wrapped palette.
    fn from(palette: Palette<f64>) -> Self {
        Self(palette)
    }
}

/// Extracts a palette from the given image data.
///
/// # Arguments
/// * `data` - The image data to extract a palette from.
/// * `width` - The width of the image.
/// * `height` - The height of the image.
/// * `algorithm` - The algorithm to use for extracting the palette.
///
/// # Returns
/// An extracted palette.
#[wasm_bindgen(js_name = extractPalette)]
pub fn extract_palette(
    data: Clamped<Vec<u8>>,
    width: u32,
    height: u32,
    algorithm: Algorithm,
) -> Result<PaletteWrapper, JsValue> {
    ImageBuffer::from_vec(width, height, data.to_vec())
        .map(|buffer| Ok(algorithm.apply(buffer)))
        .unwrap_or(Err(JsValue::from_str("Failed to convert data to image")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto_palette::color_struct::Color;
    use auto_palette::rgb::RGB;
    use auto_palette::Swatch;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_palette_wrapper() {
        let palette = Palette::default();
        let wrapper = PaletteWrapper(palette);
        assert!(wrapper.is_empty());
        assert_eq!(wrapper.length(), 0);
    }

    #[wasm_bindgen_test]
    fn test_swatches() {
        let palette = Palette::new(vec![
            Swatch::new(Color::from(&RGB::new(38, 129, 230)), (40, 123), 17529),
            Swatch::new(Color::from(&RGB::new(18, 99, 2)), (225, 137), 1678),
            Swatch::new(Color::from(&RGB::new(205, 225, 246)), (168, 164), 863),
            Swatch::new(Color::from(&RGB::new(244, 192, 1)), (96, 91), 473),
            Swatch::new(Color::from(&RGB::new(90, 42, 7)), (139, 74), 338),
        ]);
        let wrapper = PaletteWrapper(palette);
        let swatches = wrapper.swatches(3);
        assert_eq!(swatches.length(), 3);
    }

    #[wasm_bindgen_test]
    fn test_extract_palette() {
        let data = vec![
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let wrapper = extract_palette(Clamped(data), 2, 2, Algorithm::DBSCAN);
        assert!(wrapper.is_ok());
    }
}
