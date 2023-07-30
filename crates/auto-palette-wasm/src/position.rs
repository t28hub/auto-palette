use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for wrapping (x, y) positions in auto-palette
#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct Position(pub(crate) u32, pub(crate) u32);

#[wasm_bindgen]
impl Position {
    /// Returns the x position.
    ///
    /// # Returns
    /// The x position.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u32 {
        self.0
    }

    /// Returns the y position.
    ///
    /// # Returns
    /// The y position.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u32 {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let position = Position(90, 120);
        assert_eq!(position.x(), 90);
        assert_eq!(position.y(), 120);
    }
}
