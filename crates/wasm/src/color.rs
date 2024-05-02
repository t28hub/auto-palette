use auto_palette::Color;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{lab::Lab, rgb::RGB, xyz::XYZ};

/// Struct for wrapping Color<f32> in auto-palette
///
/// This struct is used to wrap the Color<f32> type from the auto-palette crate so that it can be used in JavaScript.
#[wasm_bindgen]
pub struct ColorWrapper(pub(super) Color<f32>);

#[wasm_bindgen]
impl ColorWrapper {
    /// Checks whether this color is light.
    ///
    /// # Returns
    /// `true` if this color is light, `false` otherwise.
    #[wasm_bindgen(js_name = isLight)]
    pub fn is_light(&self) -> bool {
        self.0.is_light()
    }

    /// Checks whether this color is dark.
    ///
    /// # Returns
    /// `true` if this color is dark, `false` otherwise.
    #[wasm_bindgen(js_name = isDark)]
    pub fn is_dark(&self) -> bool {
        self.0.is_dark()
    }

    /// Returns the lightness of this color.
    ///
    /// # Returns
    /// The lightness of this color.
    pub fn lightness(&self) -> f32 {
        self.0.lightness()
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    pub fn chroma(&self) -> f32 {
        self.0.chroma()
    }

    /// Returns the hue of this color.
    ///
    /// The hue is the angle of the vector in the a*b* plane.
    ///
    /// # Returns
    /// The hue of this color.
    pub fn hue(&self) -> f32 {
        self.0.hue()
    }

    /// Returns the RGB representation of this color.
    ///
    /// # Returns
    /// The RGB representation of this color.
    #[wasm_bindgen(js_name = toRGB)]
    pub fn to_rgb(&self) -> Result<JsValue, JsValue> {
        let rgb = self.0.to_rgb();
        let value = RGB {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        };
        serde_wasm_bindgen::to_value(&value).map_err(JsValue::from)
    }

    /// Returns the CIE XYZ representation of this color.
    ///
    /// # Returns
    /// The CIE XYZ representation of this color.
    #[wasm_bindgen(js_name = toXYZ)]
    pub fn to_xyz(&self) -> Result<JsValue, JsValue> {
        let xyz = self.0.to_xyz();
        let value = XYZ {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
        };
        serde_wasm_bindgen::to_value(&value).map_err(JsValue::from)
    }

    /// Returns the CIE L*a*b* representation of this color.
    ///
    /// # Returns
    /// The CIE L*a*b* representation of this color.
    #[wasm_bindgen(js_name = toLab)]
    pub fn to_lab(&self) -> Result<JsValue, JsValue> {
        let lab = self.0.to_lab();
        let value = Lab {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        };
        serde_wasm_bindgen::to_value(&value).map_err(JsValue::from)
    }

    /// Returns the hex string representation of this color.
    ///
    /// # Returns
    /// The hex string representation of this color.
    #[wasm_bindgen(js_name = toHexString)]
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_is_light() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color);

        // Assert
        assert_eq!(wrapper.is_light(), true);
    }

    #[test]
    fn test_is_dark() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color);

        // Assert
        assert_eq!(wrapper.is_dark(), false);
    }

    #[test]
    fn test_lightness() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color.clone());
        let actual = wrapper.lightness();

        // Assert
        assert_eq!(actual, color.lightness());
    }

    #[test]
    fn test_chroma() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color.clone());
        let actual = wrapper.chroma();

        // Assert
        assert_eq!(actual, color.chroma());
    }

    #[test]
    fn test_hue() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color.clone());
        let actual = wrapper.hue();

        // Assert
        assert_eq!(actual, color.hue());
    }

    #[wasm_bindgen_test]
    fn test_to_rgb() {
        // Arrange
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color);

        // Act
        let rgb = wrapper.to_rgb().unwrap();
        let actual: RGB = serde_wasm_bindgen::from_value(rgb).unwrap();

        // Assert
        assert_eq!(
            actual,
            RGB {
                r: 20,
                g: 153,
                b: 114,
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_to_xyz() {
        // Arrange
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color);

        // Act
        let xyz = wrapper.to_xyz().unwrap();
        let actual: XYZ = serde_wasm_bindgen::from_value(xyz).unwrap();

        // Assert
        assert_eq!(
            actual,
            XYZ {
                x: 0.147_161_54,
                y: 0.241_450_06,
                z: 0.198_049_84,
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_to_lab() {
        // Arrange
        let color = Color::from_str("#149972").unwrap();
        let wrapper = ColorWrapper(color);

        // Act
        let lab = wrapper.to_lab().unwrap();
        let actual: Lab = serde_wasm_bindgen::from_value(lab).unwrap();

        // Assert
        assert_eq!(
            actual,
            Lab {
                l: 56.232_69,
                a: -42.861_58,
                b: 11.220_444,
            }
        );
    }

    #[test]
    fn test_to_hex_string() {
        // Act
        let color = Color::from_str("#149972").unwrap();
        let actual = ColorWrapper(color);

        // Assert
        assert_eq!(actual.to_hex_string(), "#149972");
    }
}
