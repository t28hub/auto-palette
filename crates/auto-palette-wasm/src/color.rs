use crate::json::RGBJson;
use auto_palette::color_struct::Color;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Struct for wrapping Color<f64> in auto-palette
#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub struct ColorWrapper(pub(crate) Color<f64>);

#[wasm_bindgen]
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

    #[wasm_bindgen(js_name = toRGB)]
    pub fn to_rgb(&self) -> Result<JsValue, JsValue> {
        let rgb = self.0.to_rgb();
        let json = RGBJson {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        };
        Ok(serde_wasm_bindgen::to_value(&json)?)
    }

    #[wasm_bindgen(js_name = toLab)]
    pub fn to_lab(&self) -> Result<JsValue, JsValue> {
        let lab = self.0.to_lab();
        let json = crate::json::LabJson {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        };
        Ok(serde_wasm_bindgen::to_value(&json)?)
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
    use crate::json::LabJson;
    use auto_palette::lab::Lab;
    use auto_palette::rgb::RGB;
    use auto_palette::white_point::D65;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_color_wrapper() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.0, color);
    }

    #[wasm_bindgen_test]
    fn test_is_light() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.is_light(), color.is_light());
    }

    #[wasm_bindgen_test]
    fn test_is_dark() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.is_dark(), color.is_dark());
    }

    #[wasm_bindgen_test]
    fn test_lightness() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.lightness(), color.lightness());
    }

    #[wasm_bindgen_test]
    fn test_chroma() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.chroma(), color.chroma());
    }

    #[wasm_bindgen_test]
    fn test_hue() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.hue(), color.hue());
    }

    #[wasm_bindgen_test]
    fn test_to_rgb() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color);
        let rgb: RGBJson = serde_wasm_bindgen::from_value(wrapper.to_rgb().unwrap()).unwrap();
        assert_eq!(
            rgb,
            RGBJson {
                r: 255,
                g: 0,
                b: 64
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_to_lab() {
        let color: Color<f64> = Color::from(&Lab::<_, D65>::new(
            53.24079414140596,
            93.530808985697,
            40.899171645982,
        ));
        let wrapper = ColorWrapper(color);
        let lab: LabJson = serde_wasm_bindgen::from_value(wrapper.to_lab().unwrap()).unwrap();
        assert_eq!(
            lab,
            LabJson {
                l: 53.24079414140596,
                a: 93.530808985697,
                b: 40.899171645982
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_to_hex_string() {
        let color: Color<f64> = Color::from(&RGB::new(255, 0, 64));
        let wrapper = ColorWrapper(color.clone());
        assert_eq!(wrapper.to_hex_string(), color.to_hex_string());
    }
}
