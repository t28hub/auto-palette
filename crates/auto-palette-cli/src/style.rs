use std::{collections::BTreeSet, fmt::Display};

use crate::color::ColorMode;

/// The display attribute.
///
/// See the following for more details:
/// [SGR (Select Graphic Rendition) parameters - Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Attribute {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
}

impl Attribute {
    /// Returns the ANSI escape code for this attribute.
    ///
    /// # Returns
    /// The ANSI escape code for this attribute.
    #[inline]
    #[must_use]
    fn code(&self) -> u8 {
        match self {
            Self::Bold => 1,
            Self::Dim => 2,
            Self::Italic => 3,
            Self::Underline => 4,
            Self::Blink => 5,
            Self::Reverse => 7,
            Self::Hidden => 8,
        }
    }
}

/// The display style in the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    foreground: Option<ColorMode>,
    background: Option<ColorMode>,
    attributes: BTreeSet<Attribute>,
}

#[allow(dead_code)]
impl Style {
    /// Applies the color to this string.
    ///
    /// # Arguments
    /// * `color` - The color of this string.
    ///
    /// # Returns
    /// The styled string with the color.
    #[inline]
    #[must_use]
    pub fn color(mut self, color: ColorMode) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Applies the background color to this string.
    ///
    /// # Arguments
    /// * `color` - The background color of this string.
    ///
    /// # Returns
    /// The styled string with the background color.
    #[inline]
    #[must_use]
    pub fn background(mut self, color: ColorMode) -> Self {
        self.background = Some(color);
        self
    }

    /// Applies the bold style to this string.
    ///
    /// # Returns
    /// The styled string with the bold style.
    #[inline]
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.attributes.insert(Attribute::Bold);
        self
    }

    /// Applies the dim style to this string.
    ///
    /// # Returns
    /// The styled string with the dim style.
    #[inline]
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.attributes.insert(Attribute::Dim);
        self
    }

    /// Applies the italic style to this string.
    ///
    /// # Returns
    /// The styled string with the italic style.
    #[inline]
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.attributes.insert(Attribute::Italic);
        self
    }

    /// Applies the underline style to this string.
    ///
    /// # Returns
    /// The styled string with the underline style.
    #[inline]
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.attributes.insert(Attribute::Underline);
        self
    }

    /// Applies the blink style to this string.
    ///
    /// # Returns
    /// The styled string with the blink style.
    #[inline]
    #[must_use]
    pub fn blink(mut self) -> Self {
        self.attributes.insert(Attribute::Blink);
        self
    }

    /// Applies the reverse style to this string.
    ///
    /// # Returns
    /// The styled string with the reverse style.
    #[inline]
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.attributes.insert(Attribute::Reverse);
        self
    }

    /// Applies the hidden style to this string.
    ///
    /// # Returns
    /// The styled string with the hidden style.
    #[inline]
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.attributes.insert(Attribute::Hidden);
        self
    }

    /// Applies the style to this string.
    ///
    /// # Arguments
    /// * `value` - The value to apply the style.
    ///
    /// # Returns
    /// The styled string.
    #[inline]
    #[must_use]
    pub fn apply<T>(&self, value: T) -> StyledString
    where
        T: Into<String>,
    {
        StyledString {
            style: self.clone(),
            value: value.into(),
        }
    }
}

impl Default for Style {
    /// Creates a new default `Style` instance.
    ///
    /// # Returns
    /// A new default `Style` instance.
    fn default() -> Self {
        Self {
            foreground: None,
            background: None,
            attributes: BTreeSet::new(),
        }
    }
}

/// The styled string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledString {
    style: Style,
    value: String,
}

impl Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut applied = false;
        if let Some(foreground) = &self.style.foreground {
            write!(f, "\x1b[{}m", foreground.fg_code())?;
            applied = true;
        }

        if let Some(background) = &self.style.background {
            write!(f, "\x1b[{}m", background.bg_code())?;
            applied = true;
        }

        for attribute in &self.style.attributes {
            write!(f, "\x1b[{}m", attribute.code())?;
            applied = true;
        }

        write!(f, "{}", self.value)?;

        if applied {
            write!(f, "\x1b[0m")?;
        }
        Ok(())
    }
}

/// Creates a new `StyledString` instance.
///
/// # Returns
/// A new `StyledString` instance.
#[inline]
#[must_use]
pub fn style() -> Style {
    Style::default()
}

#[cfg(test)]
mod tests {
    use auto_palette::color::{Ansi16, Ansi256, RGB};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::bold(Attribute::Bold, 1)]
    #[case::dim(Attribute::Dim, 2)]
    #[case::italic(Attribute::Italic, 3)]
    #[case::underline(Attribute::Underline, 4)]
    #[case::blink(Attribute::Blink, 5)]
    #[case::reverse(Attribute::Reverse, 7)]
    #[case::hidden(Attribute::Hidden, 8)]
    fn test_attribute_code(#[case] attribute: Attribute, #[case] expected: u8) {
        // Act
        let actual = attribute.code();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_default() {
        // Act
        let actual = Style::default();

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: None,
                background: None,
                attributes: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn test_color_ansi16() {
        // Arrange
        let color = ColorMode::Ansi16(Ansi16::black());

        // Act
        let actual = style().color(color.clone());

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: Some(color),
                background: None,
                attributes: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn test_color_ansi256() {
        // Arrange
        let color = ColorMode::Ansi256(Ansi256::new(86));

        // Act
        let actual = style().color(color.clone());

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: Some(color),
                background: None,
                attributes: BTreeSet::new(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(format!("{}", actual), "\x1b[38;5;86mHello, world!\x1b[0m");
    }

    #[test]
    fn test_color_true_color() {
        // Arrange
        let color = ColorMode::TrueColor(RGB::new(30, 215, 96));

        // Act
        let actual = style().color(color.clone());

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: Some(color),
                background: None,
                attributes: BTreeSet::new(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(
            format!("{}", actual),
            "\x1b[38;2;30;215;96mHello, world!\x1b[0m"
        );
    }

    #[test]
    fn test_background() {
        // Arrange
        let color = ColorMode::TrueColor(RGB::new(30, 215, 96));

        // Act
        let actual = style().background(color.clone());

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: None,
                background: Some(color),
                attributes: BTreeSet::new(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(
            format!("{}", actual),
            "\x1b[48;2;30;215;96mHello, world!\x1b[0m"
        );
    }

    #[rstest]
    #[case::bold(style().bold(), vec![Attribute::Bold], "\x1b[1mHello, world!\x1b[0m")]
    #[case::dim(style().dim(), vec![Attribute::Dim], "\x1b[2mHello, world!\x1b[0m")]
    #[case::italic(style().italic(), vec![Attribute::Italic], "\x1b[3mHello, world!\x1b[0m")]
    #[case::underline(style().underline(), vec![Attribute::Underline], "\x1b[4mHello, world!\x1b[0m")]
    #[case::blink(style().blink(), vec![Attribute::Blink], "\x1b[5mHello, world!\x1b[0m")]
    #[case::reverse(style().reverse(), vec![Attribute::Reverse], "\x1b[7mHello, world!\x1b[0m")]
    #[case::hidden(style().hidden(), vec![Attribute::Hidden], "\x1b[8mHello, world!\x1b[0m")]
    fn test_styled(
        #[case] actual: Style,
        #[case] attributes: Vec<Attribute>,
        #[case] expected: &str,
    ) {
        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: None,
                background: None,
                attributes: attributes.iter().cloned().collect(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn test_styled_no_style() {
        // Act
        let actual = style();

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: None,
                background: None,
                attributes: BTreeSet::new(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(format!("{}", actual), "Hello, world!");
    }

    #[test]
    fn test_styled_multiple_styles() {
        // Act
        let actual = style()
            .color(ColorMode::TrueColor(RGB::new(255, 255, 255)))
            .background(ColorMode::TrueColor(RGB::new(30, 215, 96)))
            .bold()
            .italic()
            .underline()
            .reverse();

        // Assert
        assert_eq!(
            actual,
            Style {
                foreground: Some(ColorMode::TrueColor(RGB::new(255, 255, 255))),
                background: Some(ColorMode::TrueColor(RGB::new(30, 215, 96))),
                attributes: [
                    Attribute::Bold,
                    Attribute::Italic,
                    Attribute::Underline,
                    Attribute::Reverse
                ]
                .iter()
                .cloned()
                .collect(),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(
            format!("{}", actual),
            "\x1b[38;2;255;255;255m\x1b[48;2;30;215;96m\x1b[1m\x1b[3m\x1b[4m\x1b[7mHello, world!\x1b[0m"
        );
    }
}
