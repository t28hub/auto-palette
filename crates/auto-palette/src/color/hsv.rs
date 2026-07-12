use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{hue::Hue, Rgb};

/// The HSV color representation.
///
/// See the following for more details:
/// [HSL and HSV - Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV)
///
/// # Fields
/// * `h` - The hue component.
/// * `s` - The saturation component.
/// * `v` - The value component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Hsv, Rgb};
///
/// let rgb = Rgb::new(255, 255, 0);
/// let hsv = Hsv::from(&rgb);
/// assert_eq!(format!("{}", hsv), "HSV(60.00, 1.00, 1.00)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Hsv {
    pub h: Hue,
    pub s: f64,
    pub v: f64,
}

impl Hsv {
    /// Creates a new `Hsv` instance.
    ///
    /// # Arguments
    /// * `h` - The hue component.
    /// * `s` - The saturation component.
    /// * `v` - The value component.
    ///
    /// # Returns
    /// A new `Hsv` instance.
    #[must_use]
    pub fn new(h: f64, s: f64, v: f64) -> Self {
        Self {
            // Hue::from_degrees normalizes the hue value to the range [0, 360)
            h: Hue::from_degrees(h),
            s: clamp(s, 0.0, 1.0),
            v: clamp(v, 0.0, 1.0),
        }
    }
}

impl Display for Hsv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HSV({:.2}, {:.2}, {:.2})", self.h, self.s, self.v)
    }
}

impl From<&Rgb> for Hsv {
    fn from(rgb: &Rgb) -> Self {
        let max = Rgb::max_value();
        let r = f64::from(rgb.r) / max;
        let g = f64::from(rgb.g) / max;
        let b = f64::from(rgb.b) / max;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };

        let v = max;

        Self::new(h, s, v)
    }
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
        let actual = Hsv::new(60.0, 1.0, 1.0);

        // Assert
        assert_eq!(
            actual,
            Hsv {
                h: Hue::from_degrees(60.0),
                s: 1.0,
                v: 1.0
            }
        )
    }

    #[rstest]
    #[case((400.0, 2.0, 2.0), (40.0, 1.0, 1.0))]
    #[case((-40.0, -1.0, -1.0), (320.0, 0.0, 0.0))]
    fn test_new_with_out_of_range_values(
        #[case] input: (f64, f64, f64),
        #[case] expected: (f64, f64, f64),
    ) {
        // Act
        let (h, s, v) = input;
        let actual = Hsv::new(h, s, v);

        // Assert
        let (h, s, v) = expected;
        assert_eq!(actual, Hsv::new(h, s, v));
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let hsv = Hsv::new(60.0, 0.75, 0.5);

        // Assert
        assert_ser_tokens(
            &hsv,
            &[
                Token::Struct {
                    name: "Hsv",
                    len: 3,
                },
                Token::Str("h"),
                Token::F64(60.0),
                Token::Str("s"),
                Token::F64(0.75),
                Token::Str("v"),
                Token::F64(0.5),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let hsv = Hsv::new(30.0, 0.5, 0.75);

        // Assert
        assert_de_tokens(
            &hsv,
            &[
                Token::Struct {
                    name: "Hsv",
                    len: 3,
                },
                Token::Str("h"),
                Token::F64(30.0),
                Token::Str("s"),
                Token::F64(0.5),
                Token::Str("v"),
                Token::F64(0.75),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let hsv = Hsv::new(60.0, 1.0, 1.0);
        let actual = format!("{}", hsv);

        // Assert
        assert_eq!(actual, "HSV(60.00, 1.00, 1.00)");
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.0, 0.0, 1.0))]
    #[case::red((255, 0, 0), (0.0, 1.0, 1.0))]
    #[case::green((0, 255, 0), (120.0, 1.0, 1.0))]
    #[case::blue((0, 0, 255), (240.0, 1.0, 1.0))]
    #[case::yellow((255, 255, 0), (60.0, 1.0, 1.0))]
    #[case::cyan((0, 255, 255), (180.0, 1.0, 1.0))]
    #[case::magenta((255, 0, 255), (300.0, 1.0, 1.0))]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] hsv: (f64, f64, f64)) {
        // Act
        let rgb = Rgb::new(rgb.0, rgb.1, rgb.2);
        let actual = Hsv::from(&rgb);

        // Assert
        assert_eq!(actual.h.to_degrees(), hsv.0);
        assert_eq!(actual.s, hsv.1);
        assert_eq!(actual.v, hsv.2);
    }
}
