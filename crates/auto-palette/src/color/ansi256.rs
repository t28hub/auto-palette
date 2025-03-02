use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[cfg(feature = "wasm")]
use serde::{Serialize, Serializer};

use crate::{
    color::{Ansi16, RGB},
    FloatNumber,
};

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
/// assert_eq!(ansi256.code(), 78);
/// assert_eq!(format!("{}", ansi256), "ANSI256(78)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(feature = "wasm")]
impl Serialize for Ansi256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.code.serialize(serializer)
    }
}

impl Display for Ansi256 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ANSI256({})", self.code)
    }
}

impl From<&RGB> for Ansi256 {
    fn from(rgb: &RGB) -> Self {
        let code = from_rgb::<f32>(rgb.r, rgb.g, rgb.b);
        Self::new(code)
    }
}

impl From<&Ansi16> for Ansi256 {
    fn from(ansi16: &Ansi16) -> Self {
        let code = ansi16.code;
        if (30..=37).contains(&code) {
            Self::new(code - 30)
        } else {
            Self::new(code - 82)
        }
    }
}

#[inline]
#[must_use]
fn from_rgb<T>(r: u8, g: u8, b: u8) -> u8
where
    T: FloatNumber,
{
    let r = T::from_u8(r);
    let g = T::from_u8(g);
    let b = T::from_u8(b);
    if (r == g) && (g == b) {
        // Grayscale colors
        if r < T::from_u8(8) {
            16 // Black
        } else if r > T::from_u8(248) {
            231 // White
        } else {
            232 + ((r - T::from_u8(8)) / T::from_u8(247) * T::from_u8(24))
                .round()
                .to_u8_unsafe()
        }
    } else {
        let denominator = T::from_u8(51);
        let r = (r / denominator).round().to_u8_unsafe();
        let g = (g / denominator).round().to_u8_unsafe();
        let b = (b / denominator).round().to_u8_unsafe();
        16 + 36 * r + 6 * g + b
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_ser_tokens, Token};

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
        assert_ser_tokens(&ansi256, &[Token::U8(120)]);
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
    #[case::gray((192, 192, 192), 250)]
    #[case::red((192, 0, 0), 160)]
    #[case::green((0, 192, 0), 40)]
    #[case::blue((0, 0, 192), 20)]
    #[case::yellow((192, 192, 0), 184)]
    #[case::cyan((0, 192, 192), 44)]
    #[case::magenta((192, 0, 192), 164)]
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
