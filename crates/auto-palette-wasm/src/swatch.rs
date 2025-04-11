use wasm_bindgen::{prelude::wasm_bindgen, JsError};

use crate::{color::JsColor, position::JsPosition};

/// The swatch representation.
#[derive(Debug)]
#[wasm_bindgen(js_name = Swatch)]
pub struct JsSwatch {
    color: JsColor,
    position: JsPosition,
    population: usize,
    ratio: f64,
}

#[wasm_bindgen(js_class = Swatch)]
impl JsSwatch {
    /// Creates a new `Swatch` instance.
    ///
    /// @param color The color of the swatch.
    /// @param position The position of the swatch.
    /// @param population The population of the swatch.
    /// @param ratio The ratio of the swatch to the total population.
    /// @returns A new `Swatch` instance.
    /// @throws { JsError } if population is zero or ratio is not between 0 and 1.
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: JsColor,
        position: JsPosition,
        population: usize,
        ratio: f64,
    ) -> Result<JsSwatch, JsError> {
        if population == 0 {
            return Err(JsError::new("Population cannot be zero"));
        }
        if !(0.0..=1.0).contains(&ratio) {
            return Err(JsError::new("Ratio must be between 0 and 1"));
        }

        Ok(JsSwatch {
            color,
            position,
            population,
            ratio,
        })
    }

    /// Returns the color of this swatch.
    ///
    /// @returns The color of this swatch.
    #[wasm_bindgen(getter)]
    pub fn color(&self) -> JsColor {
        self.color.clone()
    }

    /// Returns the position of this swatch.
    ///
    /// @returns The position of this swatch.
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> JsPosition {
        self.position.clone()
    }

    /// Returns the population of this swatch.
    ///
    /// @returns The population of this swatch.
    #[wasm_bindgen(getter)]
    pub fn population(&self) -> usize {
        self.population
    }

    /// Returns the ratio of this swatch.
    ///
    /// @returns The ratio of this swatch.
    #[wasm_bindgen(getter)]
    pub fn ratio(&self) -> f64 {
        self.ratio
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_new_swatch() {
        // Act
        let color = JsColor::from_hex_string("#ff8000").unwrap();
        let position = JsPosition { x: 10, y: 20 };
        let population = 256;
        let ratio = 0.25;
        let actual = JsSwatch::new(color.clone(), position.clone(), population, ratio).unwrap();

        // Assert
        assert_eq!(actual.color(), color);
        assert_eq!(actual.position(), position);
        assert_eq!(actual.population(), population);
        assert_eq!(actual.ratio(), ratio);
    }
}
