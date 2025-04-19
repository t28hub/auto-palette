use auto_palette::Swatch;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{color::JsColor, position::JsPosition};

#[wasm_bindgen(typescript_custom_section)]
const TYPE_DEFINITION: &'static str = r#"
/**
 * The swatch representation in a palette.
 */
export class Swatch {
    /**
     * The color of the swatch.
     */
    readonly color: Color;

    /**
     * The position of the swatch.
     */
    readonly position: Position;

    /**
     * The population of the swatch.
     */
    readonly population: number;

    /**
     * The ratio of the swatch to the total population.
     */
    readonly ratio: number;

    /**
     * Creates a new `Swatch` instance.
     *
     * @param color The color of the swatch.
     * @param position The position of the swatch.
     * @param population The population of the swatch.
     * @param ratio The ratio of the swatch to the total population.
     * @returns A new `Swatch` instance.
     * @throws { Error } if population is zero or ratio is not between 0 and 1.
     */
    constructor(
        color: Color,
        position: Position,
        population: number,
        ratio: number
    );
}
"#;

/// The swatch representation.
#[derive(Debug)]
#[wasm_bindgen(js_name = Swatch, skip_typescript)]
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
        position: JsValue,
        population: usize,
        ratio: f64,
    ) -> Result<JsSwatch, JsError> {
        if population == 0 {
            return Err(JsError::new("Population cannot be zero"));
        }
        if !(0.0..=1.0).contains(&ratio) {
            return Err(JsError::new("Ratio must be between 0 and 1"));
        }
        let position: JsPosition = from_value(position)
            .map_err(|cause| JsError::new(&format!("Invalid position: {}", cause)))?;
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
    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Result<JsValue, JsError> {
        to_value(&self.position).map_err(|cause| JsError::new(&cause.to_string()))
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

impl From<Swatch<f64>> for JsSwatch {
    fn from(swatch: Swatch<f64>) -> Self {
        let color = JsColor(*swatch.color());
        let (x, y) = swatch.position();
        let position = JsPosition { x, y };
        let population = swatch.population();
        let ratio = swatch.ratio();

        JsSwatch {
            color,
            position,
            population,
            ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use auto_palette::color::Color;
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
        let actual = JsSwatch::new(
            color.clone(),
            to_value(&position).unwrap(),
            population,
            ratio,
        )
        .unwrap();

        // Assert
        assert_eq!(actual.color(), color);
        assert_eq!(
            from_value::<JsPosition>(actual.position().unwrap()).unwrap(),
            position
        );
        assert_eq!(actual.population(), population);
        assert_eq!(actual.ratio(), ratio);
    }

    #[wasm_bindgen_test]
    fn test_from_swatch() {
        // Act
        let color = Color::from_str("#ff8000").unwrap();
        let position = JsPosition { x: 10, y: 20 };
        let population = 256;
        let ratio = 0.25;
        let swatch = Swatch::new(color.clone(), (position.x, position.y), population, ratio);
        let actual = JsSwatch::from(swatch);

        // Assert
        assert_eq!(actual.color(), JsColor::from_hex_string("#ff8000").unwrap());
        assert_eq!(
            from_value::<JsPosition>(actual.position().unwrap()).unwrap(),
            position
        );
        assert_eq!(actual.population(), population);
        assert_eq!(actual.ratio(), ratio);
    }
}
