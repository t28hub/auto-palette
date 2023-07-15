use crate::SwatchWrapper;
use auto_palette::Palette;
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Struct for wrapping Palette<f64> in wasm
#[derive(Debug)]
#[wasm_bindgen(js_name = Palette)]
pub struct PaletteWrapper(Palette<f64>);

#[wasm_bindgen(js_class = Palette)]
impl PaletteWrapper {
    /// Returns the number of swatches in this palette.
    ///
    /// # Returns
    /// The number of swatches in this palette.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if this palette contains no swatches.
    ///
    /// # Returns
    /// `true` if this palette contains no swatches.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn swatches(&self, n: usize) -> Array {
        let swatches = self.0.swatches(n);
        let array = Array::new_with_length(swatches.len() as u32);
        for swatch in swatches.into_iter() {
            let value = JsValue::from(SwatchWrapper(swatch));
            array.push(&value);
        }
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_palette_wrapper() {
        let palette = Palette::default();
        let palette_wrapper = PaletteWrapper(palette);
        assert!(palette_wrapper.is_empty());
        assert_eq!(palette_wrapper.len(), 0);
    }
}
