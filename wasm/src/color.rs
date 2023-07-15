use auto_palette::color_struct::Color;
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for wrapping Color<f64> in wasm
#[derive(Debug, PartialEq)]
#[wasm_bindgen(js_name = Color)]
pub struct ColorWrapper(pub(crate) Color<f64>);

#[wasm_bindgen(js_class = Color)]
impl ColorWrapper {
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto_palette::rgb::RGB;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_color_wrapper() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let color_wrapper = ColorWrapper(color);
        assert_eq!(color_wrapper.to_hex_string(), "#ff0040");
    }
}
