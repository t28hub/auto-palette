use wasm_bindgen::prelude::wasm_bindgen;

/// Struct representing an RGB color.
#[wasm_bindgen(js_name = RGB)]
#[derive(Debug, PartialEq)]
pub struct Rgb {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
}
