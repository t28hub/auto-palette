use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for representing a position.
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Position {
    /// The x coordinate.
    pub x: u32,
    /// The y coordinate.
    pub y: u32,
}
