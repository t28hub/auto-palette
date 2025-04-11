use std::str::FromStr;

use auto_palette::color::{
    Ansi16,
    Ansi256,
    Color,
    Hue,
    LCHab,
    LCHuv,
    Lab,
    Luv,
    Oklab,
    Oklch,
    CMYK,
    HSL,
    HSV,
    RGB,
    XYZ,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

/// `JsColor` is a struct for wrapping `Color<f64>` to expose it to JavaScript.
/// This struct is used to wrap the `Color` type from the auto-palette crate so that it can be used in JavaScript.
#[derive(Debug, PartialEq, Clone)]
#[wasm_bindgen(js_name = Color)]
pub struct JsColor(pub(crate) Color<f64>);

#[wasm_bindgen(js_class = Color)]
impl JsColor {
    /// Returns whether the color is light.
    ///
    /// # Returns
    /// `true` if the color is light, otherwise `false`.
    #[wasm_bindgen(js_name = isLight)]
    pub fn is_light(&self) -> bool {
        self.0.is_light()
    }

    /// Returns whether the color is dark.
    ///
    /// # Returns
    /// `true` if the color is dark, otherwise `false`.
    #[wasm_bindgen(js_name = isDark)]
    pub fn is_dark(&self) -> bool {
        self.0.is_dark()
    }

    /// Returns the lightness of the color.
    ///
    /// # Returns
    /// The lightness of the color.
    #[wasm_bindgen]
    pub fn lightness(&self) -> f64 {
        self.0.lightness()
    }

    /// Returns the chroma of the color.
    ///
    /// # Returns
    /// The chroma of the color.
    #[wasm_bindgen]
    pub fn chroma(&self) -> f64 {
        self.0.chroma()
    }

    /// Returns the hue of the color.
    ///
    /// # Returns
    /// The hue of the color.
    #[wasm_bindgen]
    pub fn hue(&self) -> Hue<f64> {
        self.0.hue()
    }

    /// Converts the color to RGB format.
    ///
    /// # Returns
    /// An object containing the RGB values.
    #[wasm_bindgen(js_name = "toRGB")]
    pub fn to_rgb(&self) -> RGB {
        self.0.to_rgb()
    }

    /// Converts the color to CMYK format.
    ///
    /// # Returns
    /// An object containing the CMYK values.
    #[wasm_bindgen(js_name = "toCMYK")]
    pub fn to_cmyk(&self) -> CMYK<f64> {
        self.0.to_cmyk()
    }

    /// Converts the color to HSL format.
    ///
    /// # Returns
    /// An object containing the HSL values.
    #[wasm_bindgen(js_name = "toHSL")]
    pub fn to_hsl(&self) -> HSL<f64> {
        self.0.to_hsl()
    }

    /// Converts the color to HSV format.
    ///
    /// # Returns
    /// An object containing the HSV values.
    #[wasm_bindgen(js_name = "toHSV")]
    pub fn to_hsv(&self) -> HSV<f64> {
        self.0.to_hsv()
    }

    /// Converts the color to XYZ format.
    ///
    /// # Returns
    /// An object containing the XYZ values.
    #[wasm_bindgen(js_name = "toXYZ")]
    pub fn to_xyz(&self) -> XYZ<f64> {
        self.0.to_xyz()
    }

    /// Converts the color to CIE L*a*b* format.
    ///
    /// # Returns
    /// An object containing the CIE L*a*b* values.
    #[wasm_bindgen(js_name = "toLab")]
    pub fn to_lab(&self) -> Lab<f64> {
        self.0.to_lab()
    }

    /// Converts the color to Oklab format.
    ///
    /// # Returns
    /// An object containing the Oklab values.
    #[wasm_bindgen(js_name = "toOklab")]
    pub fn to_oklab(&self) -> Oklab<f64> {
        self.0.to_oklab()
    }

    /// Converts the color to CIE LCHab format.
    ///
    /// # Returns
    /// An object containing the CIE LCHab values.
    #[wasm_bindgen(js_name = "toLCHab")]
    pub fn to_lchab(&self) -> LCHab<f64> {
        self.0.to_lchab()
    }

    /// Converts the color to CIE L*u*v* format.
    ///
    /// # Returns
    /// An object containing the CIE L*u*v* values.
    #[wasm_bindgen(js_name = "toLuv")]
    pub fn to_luv(&self) -> Luv<f64> {
        self.0.to_luv()
    }

    /// Converts the color to Oklch format.
    ///
    /// # Returns
    /// An object containing the Oklch values.
    #[wasm_bindgen(js_name = "toOklch")]
    pub fn to_oklch(&self) -> Oklch<f64> {
        self.0.to_oklch()
    }

    /// Converts the color to CIE LCHuv format.
    ///
    /// # Returns
    /// An object containing the CIE LCHuv values.
    #[wasm_bindgen(js_name = "toLCHuv")]
    pub fn to_lchuv(&self) -> LCHuv<f64> {
        self.0.to_lchuv()
    }

    /// Converts the color to ANSI 16 format.
    ///
    /// # Returns
    /// An object containing the ANSI 16 values.
    #[wasm_bindgen(js_name = "toAnsi16")]
    pub fn to_ansi16(&self) -> Ansi16 {
        self.0.to_ansi16()
    }

    /// Converts the color to ANSI 256 format.
    ///
    /// # Returns
    /// An object containing the ANSI 256 values.
    #[wasm_bindgen(js_name = "toAnsi256")]
    pub fn to_ansi256(&self) -> Ansi256 {
        self.0.to_ansi256()
    }

    /// Converts the color to RGB integer format.
    ///
    /// # Returns
    /// An integer representing the RGB value.
    #[wasm_bindgen(js_name = "toInt")]
    pub fn to_int(&self) -> u32 {
        self.0.to_rgb_int()
    }

    /// Converts the color to hex string format.
    ///
    /// # Returns
    /// A string representing the color in hex format.
    #[wasm_bindgen(js_name = toHexString)]
    pub fn to_hex_string(&self) -> String {
        self.0.to_hex_string()
    }

    /// Creates a new `Color` instance with the given RGB integer value.
    ///
    /// # Arguments
    /// * `value` - The RGB integer value representing the color.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[wasm_bindgen(js_name = fromInt)]
    pub fn from_int(value: u32) -> Self {
        let color = Color::from(value);
        Self(color)
    }

    /// Creates a new `Color` instance with the given hex string.
    ///
    /// Acceptable formats are:
    /// - `#RGB` (e.g., `#f80`)
    /// - `#RGBA` (e.g., `#f80f`)
    /// - `#RRGGBB` (e.g., `#ff8000`)
    /// - `#RRGGBBAA` (e.g., `#ff8000ff`)
    ///
    /// # Arguments
    /// * `string` - The hex string representing the color.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[wasm_bindgen(js_name = fromHexString)]
    pub fn from_hex_string(string: &str) -> Result<JsColor, JsError> {
        Color::from_str(string)
            .map(Self)
            .map_err(|cause| JsError::new(&cause.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use auto_palette::assert_approx_eq;
    use wasm_bindgen_test::*;

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_is_light() {
        // Arrange
        let white = JsColor::from_int(0xffffff);
        let black = JsColor::from_int(0x000000);

        // Act & Assert
        assert_eq!(white.is_light(), true);
        assert_eq!(black.is_light(), false);
    }

    #[wasm_bindgen_test]
    fn test_is_dark() {
        // Arrange
        let white = JsColor::from_int(0xffffff);
        let black = JsColor::from_int(0x000000);

        // Act & Assert
        assert_eq!(white.is_dark(), false);
        assert_eq!(black.is_dark(), true);
    }

    #[wasm_bindgen_test]
    fn test_lightness() {
        // Arrange
        let color = JsColor::from_int(0xff8000);
        let actual = color.lightness();

        // Assert
        assert_approx_eq!(actual, 67.052536);
    }

    #[wasm_bindgen_test]
    fn test_chroma() {
        // Arrange
        let color = JsColor::from_int(0xff8000);
        let actual = color.chroma();

        // Assert
        assert_approx_eq!(actual, 85.508024);
    }

    #[wasm_bindgen_test]
    fn test_hue() {
        // Arrange
        let color = JsColor::from_int(0xff8000);
        let actual = color.hue();

        // Assert
        assert_approx_eq!(actual.to_degrees(), 59.950111);
    }

    #[wasm_bindgen_test]
    fn test_to_rgb() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_rgb();

        // Assert
        assert_eq!(
            actual,
            RGB {
                r: 255,
                g: 128,
                b: 0
            }
        );
    }

    #[wasm_bindgen_test]
    fn test_to_cmyk() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_cmyk();

        // Assert
        assert_approx_eq!(actual.c, 0.0, 0.1);
        assert_approx_eq!(actual.m, 0.5, 0.1);
        assert_approx_eq!(actual.y, 1.0, 0.1);
        assert_approx_eq!(actual.k, 0.0, 0.1);
    }

    #[wasm_bindgen_test]
    fn test_to_hsl() {
        // Act
        let color = JsColor::from_int(0xff0080);
        let actual = color.to_hsl();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 329.882352);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.l, 0.5);
    }

    #[wasm_bindgen_test]
    fn test_to_hsv() {
        // Act
        let color = JsColor::from_int(0xff0080);
        let actual = color.to_hsv();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 329.882352);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.v, 1.0);
    }

    #[wasm_bindgen_test]
    fn test_to_xyz() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_xyz();

        // Assert
        assert_approx_eq!(actual.x, 0.489579);
        assert_approx_eq!(actual.y, 0.367015);
        assert_approx_eq!(actual.z, 0.045060);
    }

    #[wasm_bindgen_test]
    fn test_to_lab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_lab();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.a, 42.818474);
        assert_approx_eq!(actual.b, 74.014866);
    }

    #[wasm_bindgen_test]
    fn test_to_oklab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_oklab();

        // Assert
        assert_approx_eq!(actual.l, 0.731893);
        assert_approx_eq!(actual.a, 0.111823);
        assert_approx_eq!(actual.b, 0.148294);
    }

    #[wasm_bindgen_test]
    fn test_to_lchab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_lchab();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.c, 85.508024);
        assert_approx_eq!(actual.h.to_degrees(), 59.950111);
    }

    #[wasm_bindgen_test]
    fn test_to_luv() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_luv();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.u, 106.018257);
        assert_approx_eq!(actual.v, 61.464573);
    }

    #[wasm_bindgen_test]
    fn test_to_oklch() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_oklch();

        // Assert
        assert_approx_eq!(actual.l, 0.731893);
        assert_approx_eq!(actual.c, 0.185730);
        assert_approx_eq!(actual.h.to_degrees(), 52.981430);
    }

    #[wasm_bindgen_test]
    fn test_to_lchuv() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_lchuv();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.c, 122.546989);
        assert_approx_eq!(actual.h.to_degrees(), 30.103211);
    }

    #[wasm_bindgen_test]
    fn test_to_ansi16() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_ansi16();

        // Assert
        assert_eq!(actual, Ansi16::bright_yellow());
    }

    #[wasm_bindgen_test]
    fn test_to_ansi256() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_ansi256();

        // Assert
        assert_eq!(actual, Ansi256::new(208));
    }

    #[wasm_bindgen_test]
    fn test_to_int() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_int();

        // Assert
        assert_eq!(actual, 0xff8000);
    }

    #[wasm_bindgen_test]
    fn test_to_hex_string() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let actual = color.to_hex_string();

        // Assert
        assert_eq!(actual, "#FF8000");
    }

    #[wasm_bindgen_test]
    fn test_from_int() {
        // Act
        let actual = JsColor::from_int(0xff8000);

        // Assert
        assert_eq!(actual.to_int(), 0xff8000);
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_from_hex_string() {
        // Act
        let actual = JsColor::from_hex_string("#ff8000");

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().to_hex_string(), "#FF8000");
    }

    #[test]
    #[wasm_bindgen_test]
    fn test_invalid_hex_string() {
        // Act
        let actual = JsColor::from_hex_string("#ZZZZZZ");

        // Assert
        assert!(actual.is_err());
    }
}
