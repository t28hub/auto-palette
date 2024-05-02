use auto_palette::Swatch;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{position::Position, ColorWrapper};

/// Struct for wrapping Swatch<f32> in auto-palette
///
/// This struct is used to wrap the Swatch<f32> type from the auto-palette crate so that it can be used in JavaScript.
#[wasm_bindgen]
pub struct SwatchWrapper(pub(super) Swatch<f32>);

#[wasm_bindgen]
impl SwatchWrapper {
    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// The color of this swatch.
    #[wasm_bindgen]
    pub fn color(&self) -> ColorWrapper {
        ColorWrapper(*self.0.color())
    }

    /// Returns the position of this swatch.
    ///
    /// # Returns
    /// The position of this swatch.
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Result<JsValue, JsValue> {
        let (x, y) = self.0.position();
        let position = Position { x, y };
        serde_wasm_bindgen::to_value(&position).map_err(JsValue::from)
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The population of this swatch.
    #[wasm_bindgen(getter)]
    pub fn population(&self) -> usize {
        self.0.population()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use auto_palette::Color;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_color() {
        // Arrange
        let color = Color::from_str("#149972").unwrap();
        let swatch = Swatch::new(color.clone(), (128, 32), 384);
        let wrapper = SwatchWrapper(swatch);

        // Act
        let actual = wrapper.color();

        // Assert
        assert_eq!(actual.0, color);
    }

    #[wasm_bindgen_test]
    fn test_position() {
        let color = Color::from_str("#149972").unwrap();
        let swatch = Swatch::new(color.clone(), (128, 32), 384);
        let wrapper = SwatchWrapper(swatch);

        // Act
        let position = wrapper.position().unwrap();
        let actual: Position = serde_wasm_bindgen::from_value(position).unwrap();

        // Assert
        assert_eq!(actual, Position { x: 128, y: 32 });
    }

    #[test]
    fn test_population() {
        // Arrange
        let color = Color::from_str("#149972").unwrap();
        let swatch = Swatch::new(color, (128, 32), 384);
        let wrapper = SwatchWrapper(swatch);

        // Act
        let actual = wrapper.population();

        // Assert
        assert_eq!(actual, 384);
    }
}
