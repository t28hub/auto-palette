use wasm_bindgen::prelude::wasm_bindgen;
#[allow(clippy::extra_unused_type_parameters)]

/// Struct representing a position.
#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct Position {
    x: u32,
    y: u32,
}

#[wasm_bindgen]
impl Position {
    /// Creates a new `Position` instance.
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the position.
    /// * `y` - The y coordinate of the position.
    ///
    /// # Returns
    /// A `Position` instance.
    #[must_use]
    #[wasm_bindgen(constructor)]
    pub fn new(x: u32, y: u32) -> Position {
        Self { x, y }
    }

    /// Returns the x coordinate of this position.
    ///
    /// # Returns
    /// The x coordinate of this position.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Returns the y coordinate of this position.
    ///
    /// # Returns
    /// The y coordinate of this position.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_position_wrapper() {
        let position = Position::new(90, 120);
        assert_eq!(position.x(), 90);
        assert_eq!(position.y(), 120);
    }
}
