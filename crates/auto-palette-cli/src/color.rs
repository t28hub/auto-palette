use auto_palette::color;

/// The color type for the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorType {
    /// The 4-bit ANSI color.
    #[allow(dead_code)]
    Ansi16(color::Ansi16),
    /// The 8-bit ANSI color.
    Ansi256(color::Ansi256),
    /// The 24-bit true color.
    TrueColor(color::RGB),
    /// No color.
    NoColor,
}

impl ColorType {
    /// Returns the ANSI color code for background.
    ///
    /// # Returns
    /// The ANSI color code for background.
    #[inline]
    #[must_use]
    pub fn bg_code(&self) -> String {
        match self {
            Self::Ansi16(color) => format!("{}", color.background()),
            Self::Ansi256(color) => format!("48;5;{}", color.code()),
            Self::TrueColor(color) => format!("48;2;{};{};{}", color.r, color.g, color.b),
            Self::NoColor => String::from("49"),
        }
    }

    /// Returns the ANSI color code for foreground.
    ///
    /// # Returns
    /// The ANSI color code for foreground.
    #[inline]
    #[must_use]
    pub fn fg_code(&self) -> String {
        match self {
            Self::Ansi16(color) => format!("{}", color.foreground()),
            Self::Ansi256(color) => format!("38;5;{}", color.code()),
            Self::TrueColor(color) => format!("38;2;{};{};{}", color.r, color.g, color.b),
            Self::NoColor => String::from("39"),
        }
    }
}

#[cfg(test)]
mod tests {
    use auto_palette::color::{Ansi16, Ansi256, RGB};

    use super::*;

    #[test]
    fn test_bg_code() {
        let color = ColorType::Ansi16(Ansi16::bright_blue());
        assert_eq!(color.bg_code(), "104");

        let color = ColorType::Ansi256(Ansi256::new(33));
        assert_eq!(color.bg_code(), "48;5;33");

        let color = ColorType::TrueColor(RGB::new(0, 102, 255));
        assert_eq!(color.bg_code(), "48;2;0;102;255");

        let color = ColorType::NoColor;
        assert_eq!(color.bg_code(), "49");
    }

    #[test]
    fn test_fg_code() {
        let color = ColorType::Ansi16(Ansi16::bright_blue());
        assert_eq!(color.fg_code(), "94");

        let color = ColorType::Ansi256(Ansi256::new(33));
        assert_eq!(color.fg_code(), "38;5;33");

        let color = ColorType::TrueColor(RGB::new(0, 102, 255));
        assert_eq!(color.fg_code(), "38;2;0;102;255");

        let color = ColorType::NoColor;
        assert_eq!(color.fg_code(), "39");
    }
}
