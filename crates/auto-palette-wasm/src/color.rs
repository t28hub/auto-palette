use std::str::FromStr;

use auto_palette::color::Color;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

/// This section contains the TypeScript definition for the `Color` class.
#[wasm_bindgen(typescript_custom_section)]
const TYPE_DEFINITION: &'static str = r#"
/**
 * The hue representation.
 *
 * Hue is represented as a number in the range [0, 360).
 */
export type Hue = number;

/**
 * The RGB representation.
 */
export interface RGB {
    readonly r: number;
    readonly g: number;
    readonly b: number;
}

/**
 * The CMYK representation.
 */
export interface CMYK {
    readonly c: number;
    readonly m: number;
    readonly y: number;
    readonly k: number;
}

/**
 * The HSL representation.
 */
export interface HSL {
    readonly h: Hue;
    readonly s: number;
    readonly l: number;
}

/**
 * The HSV representation.
 */
export interface HSV {
    readonly h: Hue;
    readonly s: number;
    readonly v: number;
}

/**
 * The XYZ representation.
 */
export interface XYZ {
    readonly x: number;
    readonly y: number;
    readonly z: number;
}

/**
 * The CIE L*a*b* representation.
 */
export interface Lab {
    readonly l: number;
    readonly a: number;
    readonly b: number;
}

/**
 * The Oklab representation.
 */
export interface Oklab {
    readonly l: number;
    readonly a: number;
    readonly b: number;
}

/**
 * The CIE LCHab representation.
 */
export interface LCHab {
    readonly l: number;
    readonly c: number;
    readonly h: Hue;
}

/**
 * The CIE L*u*v* representation.
 */
export interface Luv {
    readonly l: number;
    readonly u: number;
    readonly v: number;
}

/**
 * The Oklch representation.
 */
export interface Oklch {
    readonly l: number;
    readonly c: number;
    readonly h: Hue;
}

/**
 * The CIE LCHuv representation.
 */
export interface LCHuv {
    readonly l: number;
    readonly c: number;
    readonly h: Hue;
}

/**
 * The ANSI 16 representation.
 */
export interface Ansi16 {
    readonly code: number;
}

/**
 * The ANSI 256 representation.
 */
export interface Ansi256 {
    readonly code: number;
}

/**
 * The color representation.
 */
export class Color {
    /**
     * Returns whether the color is light.
     *
     * @returns `true` if the color is light, otherwise `false`.
     */
    isLight(): boolean;

    /**
     * Returns whether the color is dark.
     *
     * @returns `true` if the color is dark, otherwise `false`.
     */
    isDark(): boolean;

    /**
     * Returns the lightness of the color.
     *
     * @returns The lightness of the color.
     */
    lightness(): number;

    /**
     * Returns the chroma of the color.
     *
     * @returns The chroma of the color.
     */
    chroma(): number;

    /**
     * Returns the hue of the color.
     *
     * @returns The hue of the color.
     */
    hue(): Hue;

    /**
     * Converts the color to RGB format.
     *
     * @returns An object containing the RGB values.
     */
    toRGB(): RGB;

    /**
     * Converts the color to CMYK format.
     *
     * @returns An object containing the CMYK values.
     */
    toCMYK(): CMYK;

    /**
     * Converts the color to HSL format.
     *
     * @returns An object containing the HSL values.
     */
    toHSL(): HSL;

    /**
     * Converts the color to HSV format.
     *
     * @returns An object containing the HSV values.
     */
    toHSV(): HSV;

    /**
     * Converts the color to XYZ format.
     *
     * @returns An object containing the XYZ values.
     */
    toXYZ(): XYZ;

    /**
     * Converts the color to CIE L*a*b* format.
     *
     * @returns An object containing the CIE L*a*b* values.
     */
    toLab(): Lab;

    /**
     * Converts the color to Oklab format.
     *
     * @returns An object containing the Oklab values.
     */
    toOklab(): Oklab;

    /**
     * Converts the color to CIE LCHab format.
     *
     * @returns An object containing the CIE LCHab values.
     */
    toLCHab(): LCHab;

    /**
     * Converts the color to CIE L*u*v* format.
     *
     * @returns An object containing the CIE L*u*v* values.
     */
    toLuv(): Luv;

    /**
     * Converts the color to Oklch format.
     *
     * @returns An object containing the Oklch values.
     */
    toOklch(): Oklch;

    /**
     * Converts the color to CIE LCHuv format.
     *
     * @returns An object containing the CIE LCHuv values.
     */
    toLCHuv(): LCHuv;

    /**
     * Converts the color to ANSI 16 format.
     *
     * @returns An object containing the ANSI 16 values.
     */
    toAnsi16(): Ansi16;

    /**
     * Converts the color to ANSI 256 format.
     *
     * @returns An object containing the ANSI 256 values.
     */
    toAnsi256(): Ansi256;

    /**
     * Converts the color to RGB integer format.
     *
     * @returns An integer representing the RGB value.
     */
    toInt(): number;

    /**
     * Converts the color to hex string format.
     *
     * @returns A string representing the color in hex format.
     */
    toHexString(): string;

    /**
     * Creates a new `Color` instance with the given RGB integer value.
     *
     * @param value The RGB integer value representing the color.
     * @returns A new `Color` instance.
     */
    static fromInt(value: number): Color;

    /**
     * Creates a new `Color` instance with the given hex string.
     *
     * Acceptable formats are:
     * - `#RGB` (e.g., `#f80`)
     * - `#RGBA` (e.g., `#f80f`)
     * - `#RRGGBB` (e.g., `#ff8000`)
     * - `#RRGGBBAA` (e.g., `#ff8000ff`)
     *
     * @param string The hex string representing the color.
     * @returns A new `Color` instance.
     * @throws Error if the hex string is invalid.
     */
    static fromHexString(string: string): Color;
}
"#;

/// `JsColor` is a struct for wrapping `Color<f64>` to expose it to JavaScript.
/// This struct is used to wrap the `Color` type from the auto-palette crate so that it can be used in JavaScript.
#[derive(Debug, PartialEq, Clone)]
#[wasm_bindgen(js_name = Color, skip_typescript)]
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
    pub fn hue(&self) -> Result<JsValue, JsError> {
        let hue = self.0.hue();
        to_value(&hue).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to RGB format.
    ///
    /// # Returns
    /// An object containing the RGB values.
    #[wasm_bindgen(js_name = "toRGB")]
    pub fn to_rgb(&self) -> Result<JsValue, JsError> {
        let rgb = self.0.to_rgb();
        to_value(&rgb).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to CMYK format.
    ///
    /// # Returns
    /// An object containing the CMYK values.
    #[wasm_bindgen(js_name = "toCMYK")]
    pub fn to_cmyk(&self) -> Result<JsValue, JsError> {
        let cmyk = self.0.to_cmyk();
        to_value(&cmyk).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to HSL format.
    ///
    /// # Returns
    /// An object containing the HSL values.
    #[wasm_bindgen(js_name = "toHSL")]
    pub fn to_hsl(&self) -> Result<JsValue, JsError> {
        let hsl = self.0.to_hsl();
        to_value(&hsl).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to HSV format.
    ///
    /// # Returns
    /// An object containing the HSV values.
    #[wasm_bindgen(js_name = "toHSV")]
    pub fn to_hsv(&self) -> Result<JsValue, JsError> {
        let hsv = self.0.to_hsv();
        to_value(&hsv).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to XYZ format.
    ///
    /// # Returns
    /// An object containing the XYZ values.
    #[wasm_bindgen(js_name = "toXYZ")]
    pub fn to_xyz(&self) -> Result<JsValue, JsError> {
        let xyz = self.0.to_xyz();
        to_value(&xyz).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to CIE L*a*b* format.
    ///
    /// # Returns
    /// An object containing the CIE L*a*b* values.
    #[wasm_bindgen(js_name = "toLab")]
    pub fn to_lab(&self) -> Result<JsValue, JsError> {
        let lab = self.0.to_lab();
        to_value(&lab).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to Oklab format.
    ///
    /// # Returns
    /// An object containing the Oklab values.
    #[wasm_bindgen(js_name = "toOklab")]
    pub fn to_oklab(&self) -> Result<JsValue, JsError> {
        let oklab = self.0.to_oklab();
        to_value(&oklab).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to CIE LCHab format.
    ///
    /// # Returns
    /// An object containing the CIE LCHab values.
    #[wasm_bindgen(js_name = "toLCHab")]
    pub fn to_lchab(&self) -> Result<JsValue, JsError> {
        let lchab = self.0.to_lchab();
        to_value(&lchab).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to CIE L*u*v* format.
    ///
    /// # Returns
    /// An object containing the CIE L*u*v* values.
    #[wasm_bindgen(js_name = "toLuv")]
    pub fn to_luv(&self) -> Result<JsValue, JsError> {
        let luv = self.0.to_luv();
        to_value(&luv).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to Oklch format.
    ///
    /// # Returns
    /// An object containing the Oklch values.
    #[wasm_bindgen(js_name = "toOklch")]
    pub fn to_oklch(&self) -> Result<JsValue, JsError> {
        let oklch = self.0.to_oklch();
        to_value(&oklch).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to CIE LCHuv format.
    ///
    /// # Returns
    /// An object containing the CIE LCHuv values.
    #[wasm_bindgen(js_name = "toLCHuv")]
    pub fn to_lchuv(&self) -> Result<JsValue, JsError> {
        let lchuv = self.0.to_lchuv();
        to_value(&lchuv).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to ANSI 16 format.
    ///
    /// # Returns
    /// An object containing the ANSI 16 values.
    #[wasm_bindgen(js_name = "toAnsi16")]
    pub fn to_ansi16(&self) -> Result<JsValue, JsError> {
        let ansi16 = self.0.to_ansi16();
        to_value(&ansi16).map_err(|cause| JsError::new(&cause.to_string()))
    }

    /// Converts the color to ANSI 256 format.
    ///
    /// # Returns
    /// An object containing the ANSI 256 values.
    #[wasm_bindgen(js_name = "toAnsi256")]
    pub fn to_ansi256(&self) -> Result<JsValue, JsError> {
        let ansi256 = self.0.to_ansi256();
        to_value(&ansi256).map_err(|cause| JsError::new(&cause.to_string()))
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

    /// Creates a new `JsColor` instance with the given hex string.
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
    use auto_palette::{
        assert_approx_eq,
        color::{
            Ansi16,
            Ansi256,
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
        },
    };
    use serde_wasm_bindgen::from_value;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

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
        let value = color.hue().unwrap();
        let actual: Hue = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.to_degrees(), 59.950111);
    }

    #[wasm_bindgen_test]
    fn test_to_rgb() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_rgb().unwrap();
        let actual: RGB = from_value(value).unwrap();

        // Assert
        assert_eq!(actual.r, 255);
        assert_eq!(actual.g, 128);
        assert_eq!(actual.b, 0);
    }

    #[wasm_bindgen_test]
    fn test_to_cmyk() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_cmyk().unwrap();
        let actual: CMYK = from_value(value).unwrap();

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
        let value = color.to_hsl().unwrap();
        let actual: HSL = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 329.882352);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.l, 0.5);
    }

    #[wasm_bindgen_test]
    fn test_to_hsv() {
        // Act
        let color = JsColor::from_int(0xff0080);
        let value = color.to_hsv().unwrap();
        let actual: HSV = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 329.882352);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.v, 1.0);
    }

    #[wasm_bindgen_test]
    fn test_to_xyz() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_xyz().unwrap();
        let actual: XYZ = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.x, 0.489579);
        assert_approx_eq!(actual.y, 0.367015);
        assert_approx_eq!(actual.z, 0.045060);
    }

    #[wasm_bindgen_test]
    fn test_to_lab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_lab().unwrap();
        let actual: Lab = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.a, 42.818474);
        assert_approx_eq!(actual.b, 74.014866);
    }

    #[wasm_bindgen_test]
    fn test_to_oklab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_oklab().unwrap();
        let actual: Oklab = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 0.731893);
        assert_approx_eq!(actual.a, 0.111823);
        assert_approx_eq!(actual.b, 0.148294);
    }

    #[wasm_bindgen_test]
    fn test_to_lchab() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_lchab().unwrap();
        let actual: LCHab = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.c, 85.508024);
        assert_approx_eq!(actual.h.to_degrees(), 59.950111);
    }

    #[wasm_bindgen_test]
    fn test_to_luv() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_luv().unwrap();
        let actual: Luv = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.u, 106.018257);
        assert_approx_eq!(actual.v, 61.464573);
    }

    #[wasm_bindgen_test]
    fn test_to_oklch() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_oklch().unwrap();
        let actual: Oklch = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 0.731893);
        assert_approx_eq!(actual.c, 0.185730);
        assert_approx_eq!(actual.h.to_degrees(), 52.981430);
    }

    #[wasm_bindgen_test]
    fn test_to_lchuv() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_lchuv().unwrap();
        let actual: LCHuv = from_value(value).unwrap();

        // Assert
        assert_approx_eq!(actual.l, 67.052536);
        assert_approx_eq!(actual.c, 122.546989);
        assert_approx_eq!(actual.h.to_degrees(), 30.103211);
    }

    #[wasm_bindgen_test]
    fn test_to_ansi16() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_ansi16().unwrap();
        let actual: Ansi16 = from_value(value).unwrap();

        // Assert
        assert_eq!(actual, Ansi16::bright_yellow());
    }

    #[wasm_bindgen_test]
    fn test_to_ansi256() {
        // Act
        let color = JsColor::from_int(0xff8000);
        let value = color.to_ansi256().unwrap();
        let actual: Ansi256 = from_value(value).unwrap();

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
