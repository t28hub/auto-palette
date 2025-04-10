use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::color::{Ansi16, RGB};

/// The 8-bit ANSI 256 color.
///
/// See the following for more details:
/// [ANSI escape code - Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit)
///
/// # Examples
/// ```
/// use auto_palette::color::{Ansi256, RGB};
///
/// let rgb = RGB::new(30, 215, 96);
/// let ansi256 = Ansi256::from(&rgb);
/// assert_eq!(ansi256.code(), 41);
/// assert_eq!(format!("{}", ansi256), "ANSI256(41)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Ansi256 {
    code: u8,
}

impl Ansi256 {
    /// Creates a new `Ansi256` instance.
    ///
    /// # Arguments
    /// * `code` - The ANSI 256 color code.
    ///
    /// # Returns
    /// A new `Ansi256` instance.
    #[must_use]
    pub fn new(code: u8) -> Self {
        Self { code }
    }

    /// Returns the ANSI 256 color code.
    ///
    /// # Returns
    /// The ANSI 256 color code.
    #[must_use]
    pub fn code(&self) -> u8 {
        self.code
    }
}

impl Display for Ansi256 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ANSI256({})", self.code)
    }
}

impl From<&RGB> for Ansi256 {
    fn from(rgb: &RGB) -> Self {
        let code = from_rgb(rgb.r, rgb.g, rgb.b);
        Self { code }
    }
}

impl From<&Ansi16> for Ansi256 {
    fn from(ansi16: &Ansi16) -> Self {
        let code = match ansi16.code {
            30..=37 => ansi16.code - 30,
            _ => ansi16.code - 82,
        };
        Self { code }
    }
}

/// Converts RGB values to ANSI 256 color code.
///
/// This function calculates the ANSI 256 color code based on the RGB values.
///
/// # Arguments
/// * `r` - The red component (0-255).
/// * `g` - The green component (0-255).
/// * `b` - The blue component (0-255).
///
/// # Returns
/// The ANSI 256 color code.
#[inline]
fn from_rgb(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            16
        } else if r > 248 {
            231
        } else {
            232 + ((r - 8) as u16 * 24 / 247) as u8
        }
    } else {
        16 + 36 * (r / 51) + 6 * (g / 51) + (b / 51)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;

    #[rstest]
    #[case(0)]
    #[case(128)]
    #[case(255)]
    fn test_new(#[case] code: u8) {
        // Act
        let actual = Ansi256::new(code);

        // Assert
        assert_eq!(actual, Ansi256 { code });
        assert_eq!(actual.code(), code);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let ansi256 = Ansi256::new(120);

        // Assert
        assert_ser_tokens(
            &ansi256,
            &[
                Token::Struct {
                    name: "Ansi256",
                    len: 1,
                },
                Token::Str("code"),
                Token::U8(120),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let ansi256 = Ansi256::new(120);

        // Assert
        assert_de_tokens(
            &ansi256,
            &[
                Token::Struct {
                    name: "Ansi256",
                    len: 1,
                },
                Token::Str("code"),
                Token::U8(120),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let ansi256 = Ansi256::new(120);
        let actual = format!("{}", ansi256);

        // Assert
        assert_eq!(actual, "ANSI256(120)");
    }

    #[rstest]
    #[case::black((0, 0, 0), 16)]
    #[case::white((255, 255, 255), 231)]
    #[case::gray((192, 192, 192), 249)]
    #[case::red((192, 0, 0), 124)]
    #[case::green((0, 192, 0), 34)]
    #[case::blue((0, 0, 192), 19)]
    #[case::yellow((255, 255, 0), 226)]
    #[case::cyan((0, 255, 215), 50)]
    #[case::magenta((192, 0, 192), 127)]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] expected: u8) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let actual = Ansi256::from(&rgb);

        // Assert
        assert_eq!(actual.code(), expected);
    }

    #[rstest]
    #[case::black(Ansi16::black(), 0)]
    #[case::red(Ansi16::red(), 1)]
    #[case::green(Ansi16::green(), 2)]
    #[case::yellow(Ansi16::yellow(), 3)]
    #[case::blue(Ansi16::blue(), 4)]
    #[case::magenta(Ansi16::magenta(), 5)]
    #[case::cyan(Ansi16::cyan(), 6)]
    #[case::white(Ansi16::white(), 7)]
    #[case::bright_black(Ansi16::bright_black(), 8)]
    #[case::bright_red(Ansi16::bright_red(), 9)]
    #[case::bright_green(Ansi16::bright_green(), 10)]
    #[case::bright_yellow(Ansi16::bright_yellow(), 11)]
    #[case::bright_blue(Ansi16::bright_blue(), 12)]
    #[case::bright_magenta(Ansi16::bright_magenta(), 13)]
    #[case::bright_cyan(Ansi16::bright_cyan(), 14)]
    #[case::bright_white(Ansi16::bright_white(), 15)]
    fn test_from_ansi16(#[case] ansi16: Ansi16, #[case] expected: u8) {
        // Act
        let actual = Ansi256::from(&ansi16);

        // Assert
        assert_eq!(actual.code(), expected);
    }
}
