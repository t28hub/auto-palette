use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{hue::Hue, rgb::Rgb};

/// The HSL color representation.
///
/// See the following for more details:
/// [HSL and HSV - Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV)
///
/// # Fields
/// * `h` - The hue component.
/// * `s` - The saturation component.
/// * `l` - The lightness component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Hsl, Rgb};
///
/// let rgb = Rgb::new(255, 255, 0);
/// let hsl = Hsl::from(&rgb);
/// assert_eq!(format!("{}", hsl), "HSL(60.00, 1.00, 0.50)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Hsl {
    pub h: Hue,
    pub s: f64,
    pub l: f64,
}

impl Hsl {
    /// Creates a new `Hsl` instance.
    ///
    /// # Arguments
    /// * `h` - The hue component.
    /// * `s` - The saturation component.
    /// * `l` - The lightness component.
    ///
    /// # Returns
    /// A new `Hsl` instance.
    #[must_use]
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self {
            // Hue::from_degrees normalizes the hue value to the range [0, 360)
            h: Hue::from_degrees(h),
            s: clamp(s, 0.0, 1.0),
            l: clamp(l, 0.0, 1.0),
        }
    }
}

impl Display for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HSL({:.2}, {:.2}, {:.2})", self.h, self.s, self.l)
    }
}

impl From<&Rgb> for Hsl {
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

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };
        Hsl::new(h, s, l)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual = Hsl::new(60.0, 1.0, 0.5);

        // Assert
        assert_eq!(
            actual,
            Hsl {
                h: Hue::from_degrees(60.0),
                s: 1.0,
                l: 0.5,
            }
        );
    }

    #[rstest]
    #[case((-400.0, -1.0, -1.0), (320.0, 0.0, 0.0))]
    #[case((400.0, 2.0, 2.0), (40.0, 1.0, 1.0))]
    fn test_new_with_out_of_range(
        #[case] input: (f64, f64, f64),
        #[case] expected: (f64, f64, f64),
    ) {
        // Act
        let (h, s, l) = input;
        let actual = Hsl::new(h, s, l);

        // Assert
        let (h, s, l) = expected;
        assert_eq!(actual, Hsl::new(h, s, l));
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let hsl = Hsl::new(150.0, 0.3, 0.6);

        // Assert
        assert_ser_tokens(
            &hsl,
            &[
                Token::Struct {
                    name: "Hsl",
                    len: 3,
                },
                Token::Str("h"),
                Token::F64(150.0),
                Token::Str("s"),
                Token::F64(0.3),
                Token::Str("l"),
                Token::F64(0.6),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let hsl = Hsl::new(120.0, 0.5, 0.8);

        // Assert
        assert_de_tokens(
            &hsl,
            &[
                Token::Struct {
                    name: "Hsl",
                    len: 3,
                },
                Token::Str("h"),
                Token::F64(120.0),
                Token::Str("s"),
                Token::F64(0.5),
                Token::Str("l"),
                Token::F64(0.8),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn test_fmt() {
        // Act
        let hsl = Hsl::new(60.0, 1.0, 0.5);
        let actual = format!("{}", hsl);

        // Assert
        assert_eq!("HSL(60.00, 1.00, 0.50)", actual);
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.0, 0.0, 1.0))]
    #[case::red((255, 0, 0), (0.0, 1.0, 0.5))]
    #[case::green((0, 255, 0), (120.0, 1.0, 0.5))]
    #[case::blue((0, 0, 255), (240.0, 1.0, 0.5))]
    #[case::yellow((255, 255, 0), (60.0, 1.0, 0.5))]
    #[case::cyan((0, 255, 255), (180.0, 1.0, 0.5))]
    #[case::magenta((255, 0, 255), (300.0, 1.0, 0.5))]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] expected: (f64, f64, f64)) {
        // Act
        let rgb = Rgb::new(rgb.0, rgb.1, rgb.2);
        let actual = Hsl::from(&rgb);

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), expected.0, 1e-3);
        assert_approx_eq!(actual.s, expected.1, 1e-3);
        assert_approx_eq!(actual.l, expected.2, 1e-3);
    }
}
