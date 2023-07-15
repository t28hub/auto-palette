use crate::position::Position;
use crate::ColorWrapper;
use auto_palette::Swatch;
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for wrapping Swatch<f64> in wasm
#[derive(Debug)]
#[wasm_bindgen(js_name = Swatch)]
pub struct SwatchWrapper(pub(crate) Swatch<f64>);

#[wasm_bindgen(js_class = Swatch)]
impl SwatchWrapper {
    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// A reference of color of this swatch.
    #[must_use]
    pub fn color(&self) -> ColorWrapper {
        ColorWrapper(self.0.color().clone())
    }

    /// Returns the (x, y) position of this swatch.
    ///
    /// # Returns
    /// The (x, y) position of this swatch.
    #[must_use]
    pub fn position(&self) -> Position {
        let (x, y) = self.0.position();
        Position::new(x, y)
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The population of this swatch.
    #[must_use]
    pub fn population(&self) -> usize {
        self.0.population()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto_palette::color_struct::Color;
    use auto_palette::rgb::RGB;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_swatch_wrapper() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let swatch = Swatch::new(color, (90, 120), 384);
        let swatch_wrapper = SwatchWrapper(swatch);
        assert_eq!(
            swatch_wrapper.color(),
            ColorWrapper(Color::from(&RGB::new(255, 0, 64)))
        );
        assert_eq!(swatch_wrapper.position(), Position::new(90, 120));
        assert_eq!(swatch_wrapper.population(), 384);
    }
}
