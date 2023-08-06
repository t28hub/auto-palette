use crate::json::PositionJson;
use crate::ColorWrapper;
use auto_palette::Swatch;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Struct for wrapping Swatch<f64> in auto-palette
#[derive(Debug)]
#[wasm_bindgen]
pub struct SwatchWrapper(pub(crate) Swatch<f64>);

#[wasm_bindgen]
impl SwatchWrapper {
    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// The color of this swatch.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> ColorWrapper {
        ColorWrapper(self.0.color().clone())
    }

    /// Returns the (x, y) position of this swatch.
    ///
    /// # Returns
    /// The (x, y) position of this swatch.
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Result<JsValue, JsValue> {
        let (x, y) = self.0.position();
        let json = PositionJson { x, y };
        Ok(serde_wasm_bindgen::to_value(&json)?)
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The population of this swatch.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn population(&self) -> usize {
        self.0.population()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto_palette::color_struct::Color;
    use auto_palette::rgb::RGB;

    #[test]
    fn test_swatch_wrapper() {
        let rgb = RGB::new(255, 0, 64);
        let color: Color<f64> = Color::from(&rgb);
        let swatch = Swatch::new(color, (90, 120), 384);
        let wrapper = SwatchWrapper(swatch);
        assert_eq!(wrapper.color(), ColorWrapper(Color::from(&rgb)));
        assert_eq!(wrapper.population(), 384);
    }
}
