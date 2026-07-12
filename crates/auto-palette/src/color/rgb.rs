use std::{fmt, fmt::Display};

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{hsl::Hsl, hsv::Hsv, xyz::Xyz};

/// The RGB color representation.
///
/// See the following for more details:
/// [RGB color model - Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model)
///
/// # Fields
/// * `r` - The red component.
/// * `g` - The green component.
/// * `b` - The blue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Hsl, Hsv, Rgb, Xyz};
///
/// let rgb = Rgb::new(255, 0, 64);
/// assert_eq!(format!("{}", rgb), "RGB(255, 0, 64)");
///
/// let hsl: Hsl = (&rgb).into();
/// assert_eq!(format!("{}", hsl), "HSL(344.94, 1.00, 0.50)");
///
/// let hsv: Hsv = (&rgb).into();
/// assert_eq!(format!("{}", hsv), "HSV(344.94, 1.00, 1.00)");
///
/// let xyz: Xyz = (&rgb).into();
/// assert_eq!(format!("{}", xyz), "XYZ(0.42, 0.22, 0.07)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// The maximum value of the RGB color space.
    const MAX: u8 = 255;

    /// Creates a new `Rgb` instance.
    ///
    /// # Arguments
    /// * `r` - The red component.
    /// * `g` - The green component.
    /// * `b` - The blue component.
    ///
    /// # Returns
    /// A new `Rgb` instance.
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the maximum value of the RGB color space.
    ///
    /// # Returns
    /// The maximum value of the RGB color space.
    #[inline]
    #[must_use]
    pub(crate) fn max_value() -> f64 {
        f64::from(Self::MAX)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl From<&Hsl> for Rgb {
    fn from(hsl: &Hsl) -> Self {
        let hue = hsl.h.to_degrees();

        let c = (1.0 - (2.0 * hsl.l - 1.0).abs()) * hsl.s;
        let x = (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs()) * c;
        let m = hsl.l - (c / 2.0);

        let (r, g, b) = hue_to_rgb(hue, c, x);

        Self {
            r: clamp_to_u8(r + m),
            g: clamp_to_u8(g + m),
            b: clamp_to_u8(b + m),
        }
    }
}

impl From<&Hsv> for Rgb {
    fn from(hsv: &Hsv) -> Self {
        let hue = hsv.h.to_degrees();

        let c = hsv.v * hsv.s;
        let x = (1.0 - (((hue / 60.0) % 2.0) - 1.0).abs()) * c;
        let m = hsv.v - c;

        let (r, g, b) = hue_to_rgb(hue, c, x);

        Self {
            r: clamp_to_u8(r + m),
            g: clamp_to_u8(g + m),
            b: clamp_to_u8(b + m),
        }
    }
}

impl From<&Xyz> for Rgb {
    fn from(xyz: &Xyz) -> Self {
        let (r, g, b) = xyz_to_rgb(xyz.x, xyz.y, xyz.z);
        Self { r, g, b }
    }
}

/// Clamps a floating point value to the range [0, 255] and converts it to u8.
///
/// # Arguments
/// * `value` - The floating point value to clamp.
///
/// # Returns
/// The clamped value as u8.
#[inline]
#[must_use]
fn clamp_to_u8(value: f64) -> u8 {
    let max = f64::from(Rgb::MAX);
    (value * max).round() as u8
}

/// Converts a hue value to RGB components.
///
/// # Arguments
/// * `hue` - The hue value.
/// * `c` - The chroma value.
/// * `x` - The intermediate value.
///
/// # Returns
/// The RGB components as a tuple (r, g, b).
#[inline]
#[must_use]
fn hue_to_rgb(hue: f64, c: f64, x: f64) -> (f64, f64, f64) {
    if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    }
}

/// Converts the CIE XYZ color space to the RGB color space.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The RGB color space representation of the XYZ color. The tuple contains the red, green, and blue components.
#[inline]
#[must_use]
pub(crate) fn xyz_to_rgb(x: f64, y: f64, z: f64) -> (u8, u8, u8) {
    let f = |t: f64| -> f64 {
        if t > f64::from(0.003_130_8_f32) {
            f64::from(1.055_f32) * t.powf(f64::from(1.0_f32 / 2.4_f32)) - f64::from(0.055_f32)
        } else {
            f64::from(12.92_f32) * t
        }
    };

    let r = f(f64::from(3.240_97_f32) * x
        - f64::from(1.537_383_f32) * y
        - f64::from(0.498_611_f32) * z);
    let g = f(-f64::from(0.969_244_f32) * x
        + f64::from(1.875_968_f32) * y
        + f64::from(0.041_555_f32) * z);
    let b =
        f(f64::from(0.055_630_f32) * x - f64::from(0.203_977_f32) * y
            + f64::from(1.056_972_f32) * z);
    (clamp_to_u8(r), clamp_to_u8(g), clamp_to_u8(b))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = Rgb::new(255, 0, 64);

        // Assert
        assert_eq!(actual.r, 255);
        assert_eq!(actual.g, 0);
        assert_eq!(actual.b, 64);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let rgb = Rgb::new(255, 0, 64);

        // Act
        assert_ser_tokens(
            &rgb,
            &[
                Token::Struct {
                    name: "Rgb",
                    len: 3,
                },
                Token::Str("r"),
                Token::U8(255),
                Token::Str("g"),
                Token::U8(0),
                Token::Str("b"),
                Token::U8(64),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let rgb = Rgb::new(64, 255, 0);

        // Act
        assert_de_tokens(
            &rgb,
            &[
                Token::Struct {
                    name: "Rgb",
                    len: 3,
                },
                Token::Str("r"),
                Token::U8(64),
                Token::Str("g"),
                Token::U8(255),
                Token::Str("b"),
                Token::U8(0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let rgb = Rgb::new(255, 0, 64);
        let actual = format!("{}", rgb);

        // Assert
        assert_eq!(actual, "RGB(255, 0, 64)");
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.0, 0.0, 1.0), (255, 255, 255))]
    #[case::red((0.0, 1.0, 0.5), (255, 0, 0))]
    #[case::green((120.0, 1.0, 0.5), (0, 255, 0))]
    #[case::blue((240.0, 1.0, 0.5), (0, 0, 255))]
    #[case::yellow((60.0, 1.0, 0.5), (255, 255, 0))]
    #[case::cyan((180.0, 1.0, 0.5), (0, 255, 255))]
    #[case::magenta((300.0, 1.0, 0.5), (255, 0, 255))]
    fn test_from_hsl(#[case] hsl: (f64, f64, f64), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (h, s, l) = hsl;
        let actual = Rgb::from(&Hsl::new(h, s, l));

        // Assert
        assert_eq!(actual.r, rgb.0);
        assert_eq!(actual.g, rgb.1);
        assert_eq!(actual.b, rgb.2);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.0, 0.0, 1.0), (255, 255, 255))]
    #[case::red((0.0, 1.0, 1.0), (255, 0, 0))]
    #[case::green((120.0, 1.0, 1.0), (0, 255, 0))]
    #[case::blue((240.0, 1.0, 1.0), (0, 0, 255))]
    #[case::yellow((60.0, 1.0, 1.0), (255, 255, 0))]
    #[case::cyan((180.0, 1.0, 1.0), (0, 255, 255))]
    #[case::magenta((300.0, 1.0, 1.0), (255, 0, 255))]
    fn test_from_hsv(#[case] hsv: (f64, f64, f64), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (h, s, v) = hsv;
        let actual = Rgb::from(&Hsv::new(h, s, v));

        // Assert
        assert_eq!(actual.r, rgb.0);
        assert_eq!(actual.g, rgb.1);
        assert_eq!(actual.b, rgb.2);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = Xyz::new(0.3576, 0.7152, 0.119);
        let actual = Rgb::from(&xyz);

        // Assert
        assert_eq!(actual.r, 0);
        assert_eq!(actual.g, 255);
        assert_eq!(actual.b, 0);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.9505, 1.0000, 1.0886), (255, 255, 255))]
    #[case::red((0.4125, 0.2127, 0.0193), (255, 0, 0))]
    #[case::green((0.3576, 0.7152, 0.1192), (0, 255, 0))]
    #[case::blue((0.1804, 0.0722, 0.9502), (0, 0, 255))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (0, 255, 255))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (255, 0, 255))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (255, 255, 0))]
    fn test_xyz_to_rgb(#[case] xyz: (f64, f64, f64), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (r, g, b) = xyz_to_rgb(xyz.0, xyz.1, xyz.2);

        // Assert
        assert_eq!(r, rgb.0);
        assert_eq!(g, rgb.1);
        assert_eq!(b, rgb.2);
    }
}
