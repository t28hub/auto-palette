use wasm_bindgen::prelude::wasm_bindgen;

/// Struct representing a CIE L*a*b* color.
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Lab {
    /// The lightness component of the color.
    pub l: f32,
    /// The a* component of the color.
    pub a: f32,
    /// The b* component of the color.
    pub b: f32,
}
