use wasm_bindgen::prelude::wasm_bindgen;

/// Struct representing a CIE XYZ color.
#[wasm_bindgen(js_name = XYZ)]
#[derive(Debug, PartialEq)]
pub struct Xyz {
    /// The x component of the color.
    pub x: f32,
    /// The y component of the color.
    pub y: f32,
    /// The z component of the color.
    pub z: f32,
}
