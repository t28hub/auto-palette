use std::fmt::{Display, Formatter};

#[cfg(feature = "wasm")]
use serde::{Serialize, Serializer};

use crate::color::{error::ColorError, RGB};

/// The 4-bit ANSI 16 color representation.
///
/// See the following for more details:
/// [ANSI escape code - Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit)
///
/// # Examples
/// ```
/// use auto_palette::color::{Ansi16, RGB};
///
/// let rgb = RGB::new(30, 215, 96);
/// let ansi16 = Ansi16::from(&rgb);
/// assert_eq!(ansi16.foreground(), 92);
/// assert_eq!(ansi16.background(), 102);
/// assert_eq!(ansi16, Ansi16::bright_green());
/// assert_eq!(format!("{}", ansi16), "ANSI16(92)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ansi16 {
    /// The ANSI 16 color code.
    pub(crate) code: u8,
}

impl Ansi16 {
    /// Creates a new `Ansi16` instance.
    ///
    /// # Arguments
    /// * `code` - The ANSI 16 color code for foreground text.
    ///
    /// # Returns
    /// A new `Ansi16` instance.
    pub fn new(code: u8) -> Result<Self, ColorError> {
        match code {
            30..=37 | 90..=97 => Ok(Self { code }),
            _ => Err(ColorError::InvalidColorCode(code)),
        }
    }

    /// Returns the ANSI 16 color code for foreground text.
    ///
    /// # Returns
    /// The ANSI 16 color code for foreground text.
    #[inline]
    #[must_use]
    pub fn foreground(&self) -> u8 {
        self.code
    }

    /// Returns the ANSI 16 color code for background text.
    ///
    /// # Returns
    /// The ANSI 16 color code for background text.
    #[inline]
    #[must_use]
    pub fn background(&self) -> u8 {
        self.code + 10
    }

    /// Creates a new `Ansi16` instance with the black color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the black color.
    #[inline]
    #[must_use]
    pub fn black() -> Self {
        Self { code: 30 }
    }

    /// Creates a new `Ansi16` instance with the red color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the red color.
    #[inline]
    #[must_use]
    pub fn red() -> Self {
        Self { code: 31 }
    }

    /// Creates a new `Ansi16` instance with the green color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the green color.
    #[inline]
    #[must_use]
    pub fn green() -> Self {
        Self { code: 32 }
    }

    /// Creates a new `Ansi16` instance with the yellow color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the yellow color.
    #[inline]
    #[must_use]
    pub fn yellow() -> Self {
        Self { code: 33 }
    }

    /// Creates a new `Ansi16` instance with the blue color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the blue color.
    #[inline]
    #[must_use]
    pub fn blue() -> Self {
        Self { code: 34 }
    }

    /// Creates a new `Ansi16` instance with the magenta color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the magenta color.
    #[inline]
    #[must_use]
    pub fn magenta() -> Self {
        Self { code: 35 }
    }

    /// Creates a new `Ansi16` instance with the cyan color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the cyan color.
    #[inline]
    #[must_use]
    pub fn cyan() -> Self {
        Self { code: 36 }
    }

    /// Creates a new `Ansi16` instance with the white color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the white color.
    #[inline]
    #[must_use]
    pub fn white() -> Self {
        Self { code: 37 }
    }

    /// Creates a new `Ansi16` instance with the bright black color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright black color.
    #[inline]
    #[must_use]
    pub fn bright_black() -> Self {
        Self { code: 90 }
    }

    /// Creates a new `Ansi16` instance with the bright red color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright red color.
    #[inline]
    #[must_use]
    pub fn bright_red() -> Self {
        Self { code: 91 }
    }

    /// Creates a new `Ansi16` instance with the bright green color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright green color.
    #[inline]
    #[must_use]
    pub fn bright_green() -> Self {
        Self { code: 92 }
    }

    /// Creates a new `Ansi16` instance with the bright yellow color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright yellow color.
    #[inline]
    #[must_use]
    pub fn bright_yellow() -> Self {
        Self { code: 93 }
    }

    /// Creates a new `Ansi16` instance with the bright blue color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright blue color.
    #[inline]
    #[must_use]
    pub fn bright_blue() -> Self {
        Self { code: 94 }
    }

    /// Creates a new `Ansi16` instance with the bright magenta color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright magenta color.
    #[inline]
    #[must_use]
    pub fn bright_magenta() -> Self {
        Self { code: 95 }
    }

    /// Creates a new `Ansi16` instance with the bright cyan color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright cyan color.
    #[inline]
    #[must_use]
    pub fn bright_cyan() -> Self {
        Self { code: 96 }
    }

    /// Creates a new `Ansi16` instance with the bright white color.
    ///
    /// # Returns
    /// A new `Ansi16` instance with the bright white color.
    #[inline]
    #[must_use]
    pub fn bright_white() -> Self {
        Self { code: 97 }
    }
}

#[cfg(feature = "wasm")]
impl Serialize for Ansi16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.code.serialize(serializer)
    }
}

impl Display for Ansi16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ANSI16({})", self.code)
    }
}

impl From<&RGB> for Ansi16 {
    fn from(rgb: &RGB) -> Self {
        let code = from_rgb(rgb.r, rgb.g, rgb.b);
        Self { code }
    }
}

/// Converts RGB values to an ANSI 16 color code.
///
/// This function is used internally to convert RGB values to the corresponding ANSI 16 color code.
///
/// # Arguments
/// * `r` - The red component (0-255).
/// * `g` - The green component (0-255).
/// * `b` - The blue component (0-255).
///
/// # Returns
/// The ANSI 16 color code.
#[inline]
fn from_rgb(r: u8, g: u8, b: u8) -> u8 {
    let max = r.max(g).max(b);
    let brightness = if max > 128 { 60 } else { 0 };
    let color = (((b > 127) as u8) << 2) | (((g > 127) as u8) << 1) | ((r > 127) as u8);
    color + brightness + 30
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = Ansi16::new(30).unwrap();

        // Assert
        assert_eq!(actual, Ansi16 { code: 30 });
        assert_eq!(actual.foreground(), 30);
        assert_eq!(actual.background(), 40);
    }

    #[test]
    fn test_new_bright() {
        // Act
        let actual = Ansi16::new(90).unwrap();

        // Assert
        assert_eq!(actual, Ansi16 { code: 90 });
        assert_eq!(actual.foreground(), 90);
        assert_eq!(actual.background(), 100);
    }

    #[test]
    fn test_new_invalid() {
        // Act
        let actual = Ansi16::new(29);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), ColorError::InvalidColorCode(29));
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let ansi16 = Ansi16::new(30).unwrap();

        // Act
        assert_ser_tokens(&ansi16, &[Token::U8(30)]);
    }

    #[test]
    fn test_fmt() {
        // Act
        let ansi16 = Ansi16::bright_black();
        let actual = format!("{}", ansi16);

        // Assert
        assert_eq!(actual, "ANSI16(90)");
    }

    #[rstest]
    #[case::black((0, 0, 0), Ansi16::black())]
    #[case::red((128, 0, 0), Ansi16::red())]
    #[case::green((0, 128, 0), Ansi16::green())]
    #[case::yellow((128, 128, 0), Ansi16::yellow())]
    #[case::blue((0, 0, 128), Ansi16::blue())]
    #[case::magenta((128, 0, 128), Ansi16::magenta())]
    #[case::cyan((0, 128, 128), Ansi16::cyan())]
    #[case::white((128, 128, 128), Ansi16::white())]
    #[case::bright_red((255, 0, 0), Ansi16::bright_red())]
    #[case::bright_green((0, 255, 0), Ansi16::bright_green())]
    #[case::bright_yellow((255, 255, 0), Ansi16::bright_yellow())]
    #[case::bright_blue((0, 0, 255), Ansi16::bright_blue())]
    #[case::bright_magenta((255, 0, 255), Ansi16::bright_magenta())]
    #[case::bright_cyan((0, 255, 255), Ansi16::bright_cyan())]
    #[case::bright_white((255, 255, 255), Ansi16::bright_white())]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] expected: Ansi16) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let actual = Ansi16::from(&rgb);

        // Assert
        assert_eq!(actual, expected);
    }
}
