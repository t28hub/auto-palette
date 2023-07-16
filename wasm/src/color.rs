use auto_palette::color_struct::Color;
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for wrapping Color<f64> in wasm
#[derive(Debug, PartialEq)]
#[wasm_bindgen(js_name = Color)]
pub struct ColorWrapper(pub(crate) Color<f64>);

#[wasm_bindgen(js_class = Color)]
impl ColorWrapper {
    /// Checks if this color is light.
    ///
    /// # Returns
    /// `true` if this color is light, `false` otherwise.
    #[must_use]
    #[wasm_bindgen(js_name = isLight)]
    pub fn is_light(&self) -> bool {
        self.0.is_light()
    }

    /// Checks if this color is dark.
    ///
    /// # Returns
    /// `true` if this color is dark, `false` otherwise.
    #[must_use]
    #[wasm_bindgen(js_name = isDark)]
    pub fn is_dark(&self) -> bool {
        self.0.is_dark()
    }

    /// Returns the lightness of this color.
    ///
    /// # Returns
    /// The lightness of this color.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn lightness(&self) -> f64 {
        self.0.lightness()
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn chroma(&self) -> f64 {
        self.0.chroma()
    }

    /// Returns the hue of this color.
    ///
    /// # Returns
    /// The hue of this color.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn hue(&self) -> f64 {
        self.0.hue()
    }

    /// Returns the hex string representation of this color.
    ///
    /// # Returns
    /// The hex string representation of this color.
    #[must_use]
    #[wasm_bindgen(js_name = toHexString)]
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use auto_palette::rgb::RGB;

    #[test]
    fn test_color_wrapper() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.0, color);
    }

    #[test]
    fn test_is_light() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.is_light(), color.is_light());
    }

    #[test]
    fn test_is_dark() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.is_dark(), color.is_dark());
    }

    #[test]
    fn test_lightness() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.lightness(), color.lightness());
    }

    #[test]
    fn test_chroma() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.chroma(), color.chroma());
    }

    #[test]
    fn test_hue() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.hue(), color.hue());
    }

    #[test]
    fn test_to_hex_string() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.to_hex_string(), color.to_hex_string());
    }
}
