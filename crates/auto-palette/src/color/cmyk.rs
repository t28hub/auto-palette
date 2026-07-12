use std::fmt::{Display, Formatter};

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::Rgb;

/// The CMYK color representation.
///
/// See the following for more details:
/// [CMYK color model - Wikipedia](https://en.wikipedia.org/wiki/CMYK_color_model)
///
/// # Fields
/// * `c` - The cyan component.
/// * `m` - The magenta component.
/// * `y` - The yellow component.
/// * `k` - The key (black) component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Cmyk, Rgb};
///
/// let rgb = Rgb::new(255, 255, 0);
/// let cmyk = Cmyk::from(&rgb);
/// assert_eq!(format!("{}", cmyk), "CMYK(0.00, 0.00, 1.00, 0.00)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl Cmyk {
    /// Creates a new `Cmyk` instance.
    ///
    /// # Arguments
    /// * `c` - The cyan component.
    /// * `m` - The magenta component.
    /// * `y` - The yellow component.
    /// * `k` - The key (black) component.
    ///
    /// # Returns
    /// A new `Cmyk` instance.
    #[must_use]
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self {
            c: clamp(c, 0.0, 1.0),
            m: clamp(m, 0.0, 1.0),
            y: clamp(y, 0.0, 1.0),
            k: clamp(k, 0.0, 1.0),
        }
    }
}

impl Display for Cmyk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CMYK({:.2}, {:.2}, {:.2}, {:.2})",
            self.c, self.m, self.y, self.k
        )
    }
}

impl From<&Rgb> for Cmyk {
    fn from(rgb: &Rgb) -> Self {
        let max = Rgb::max_value();
        let r = f64::from(rgb.r) / max;
        let g = f64::from(rgb.g) / max;
        let b = f64::from(rgb.b) / max;

        let k = 1.0 - r.max(g).max(b);
        if k == 1.0 {
            Cmyk::new(0.0, 0.0, 0.0, k)
        } else {
            let denominator = 1.0 - k;
            let c = (1.0 - r - k) / denominator;
            let m = (1.0 - g - k) / denominator;
            let y = (1.0 - b - k) / denominator;
            Cmyk::new(c, m, y, k)
        }
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
        let actual = Cmyk::new(1.00, 0.75, 0.50, 0.25);

        // Assert
        assert_eq!(
            actual,
            Cmyk {
                c: 1.00,
                m: 0.75,
                y: 0.50,
                k: 0.25,
            }
        );
    }

    #[rstest]
    #[case((-0.01, -0.02, -0.03, -0.04), (0.00, 0.00, 0.00, 0.00))]
    #[case((1.01, 1.02, 1.03, 1.04), (1.00, 1.00, 1.00, 1.00))]
    fn test_new_clamp(#[case] input: (f64, f64, f64, f64), #[case] expected: (f64, f64, f64, f64)) {
        // Act
        let (c, m, y, k) = input;
        let actual = Cmyk::new(c, m, y, k);

        // Assert
        assert_eq!(
            actual,
            Cmyk {
                c: expected.0,
                m: expected.1,
                y: expected.2,
                k: expected.3,
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let cmyk = Cmyk::new(1.00, 0.75, 0.50, 0.25);

        // Assert
        assert_ser_tokens(
            &cmyk,
            &[
                Token::Struct {
                    name: "Cmyk",
                    len: 4,
                },
                Token::Str("c"),
                Token::F64(1.00),
                Token::Str("m"),
                Token::F64(0.75),
                Token::Str("y"),
                Token::F64(0.50),
                Token::Str("k"),
                Token::F64(0.25),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let cmyk = Cmyk::new(0.50, 0.25, 1.00, 0.50);

        // Assert
        assert_de_tokens(
            &cmyk,
            &[
                Token::Struct {
                    name: "Cmyk",
                    len: 4,
                },
                Token::Str("c"),
                Token::F64(0.50),
                Token::Str("m"),
                Token::F64(0.25),
                Token::Str("y"),
                Token::F64(1.00),
                Token::Str("k"),
                Token::F64(0.50),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let cmyk = Cmyk::new(0.00, 0.00, 1.00, 0.00);
        let actual = format!("{}", cmyk);

        // Assert
        assert_eq!("CMYK(0.00, 0.00, 1.00, 0.00)", actual);
    }

    #[rstest]
    #[case::black(Rgb::new(0, 0, 0), Cmyk::new(0.00, 0.00, 0.00, 1.00))]
    #[case::white(Rgb::new(255, 255, 255), Cmyk::new(0.00, 0.00, 0.00, 0.00))]
    #[case::red(Rgb::new(255, 0, 0), Cmyk::new(0.00, 1.00, 1.00, 0.00))]
    #[case::green(Rgb::new(0, 255, 0), Cmyk::new(1.00, 0.00, 1.00, 0.00))]
    #[case::blue(Rgb::new(0, 0, 255), Cmyk::new(1.00, 1.00, 0.00, 0.00))]
    #[case::yellow(Rgb::new(255, 255, 0), Cmyk::new(0.00, 0.00, 1.00, 0.00))]
    #[case::cyan(Rgb::new(0, 255, 255), Cmyk::new(1.00, 0.00, 0.00, 0.00))]
    #[case::magenta(Rgb::new(255, 0, 255), Cmyk::new(0.00, 1.00, 0.00, 0.00))]
    fn test_from_rgb(#[case] rgb: Rgb, #[case] expected: Cmyk) {
        // Act
        let actual = Cmyk::from(&rgb);

        // Assert
        assert_eq!(actual, expected);
    }
}
