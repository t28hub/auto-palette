use std::{collections::BTreeSet, fmt::Display};

use crate::color::ColorType;

/// The style of the text in the terminal.
///
/// See the following for more details:
/// [SGR (Select Graphic Rendition) parameters - Wikipedia](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
}

impl Style {
    /// Returns the ANSI escape code for this style.
    ///
    /// # Returns
    /// The ANSI escape code for this style.
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

/// The styled string in the terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledString {
    value: String,
    foreground: Option<ColorType>,
    background: Option<ColorType>,
    styles: BTreeSet<Style>,
}

#[allow(dead_code)]
impl StyledString {
    /// Creates a new `StyledString` instance with the given value.
    ///
    /// # Arguments
    /// * `value` - The value of this string.
    ///
    /// # Returns
    /// A new `StyledString` instance.
    #[inline]
    #[must_use]
    fn new(value: String) -> Self {
        Self {
            value,
            foreground: None,
            background: None,
            styles: BTreeSet::new(),
        }
    }

    /// Applies the color to this string.
    ///
    /// # Arguments
    /// * `color` - The color of this string.
    ///
    /// # Returns
    /// The styled string with the color.
    #[inline]
    #[must_use]
    pub fn color(mut self, color: ColorType) -> Self {
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
    pub fn background(mut self, color: ColorType) -> Self {
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
        self.styles.insert(Style::Bold);
        self
    }

    /// Applies the dim style to this string.
    ///
    /// # Returns
    /// The styled string with the dim style.
    #[inline]
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.styles.insert(Style::Dim);
        self
    }

    /// Applies the italic style to this string.
    ///
    /// # Returns
    /// The styled string with the italic style.
    #[inline]
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.styles.insert(Style::Italic);
        self
    }

    /// Applies the underline style to this string.
    ///
    /// # Returns
    /// The styled string with the underline style.
    #[inline]
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.styles.insert(Style::Underline);
        self
    }

    /// Applies the blink style to this string.
    ///
    /// # Returns
    /// The styled string with the blink style.
    #[inline]
    #[must_use]
    pub fn blink(mut self) -> Self {
        self.styles.insert(Style::Blink);
        self
    }

    /// Applies the reverse style to this string.
    ///
    /// # Returns
    /// The styled string with the reverse style.
    #[inline]
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.styles.insert(Style::Reverse);
        self
    }

    /// Applies the hidden style to this string.
    ///
    /// # Returns
    /// The styled string with the hidden style.
    #[inline]
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.styles.insert(Style::Hidden);
        self
    }
}

impl Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut applied = false;
        if let Some(foreground) = &self.foreground {
            write!(f, "\x1b[{}m", foreground.fg_code())?;
            applied = true;
        }

        if let Some(background) = &self.background {
            write!(f, "\x1b[{}m", background.bg_code())?;
            applied = true;
        }

        for style in &self.styles {
            write!(f, "\x1b[{}m", style.code())?;
            applied = true;
        }

        write!(f, "{}", self.value)?;

        if applied {
            write!(f, "\x1b[0m")?;
        }
        Ok(())
    }
}

/// Creates a new `StyledString` instance with the given value.
///
/// # Arguments
/// * `value` - The value of this string.
///
/// # Returns
/// A new `StyledString` instance.
#[inline]
#[must_use]
pub fn styled<T>(value: T) -> StyledString
where
    T: Into<String>,
{
    StyledString::new(value.into())
}

#[cfg(test)]
mod tests {
    use auto_palette::color::{Ansi16, Ansi256, RGB};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::bold(Style::Bold, 1)]
    #[case::dim(Style::Dim, 2)]
    #[case::italic(Style::Italic, 3)]
    #[case::underline(Style::Underline, 4)]
    #[case::blink(Style::Blink, 5)]
    #[case::reverse(Style::Reverse, 7)]
    #[case::hidden(Style::Hidden, 8)]
    fn test_style_code(#[case] style: Style, #[case] expected: u8) {
        // Act
        let actual = style.code();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_new() {
        // Act
        let actual = StyledString::new("Hello, world!".into());

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: None,
                background: None,
                styles: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn test_color_ansi16() {
        // Arrange
        let color = ColorType::Ansi16(Ansi16::black());

        // Act
        let actual = styled("Hello, world!").color(color.clone());

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: Some(color),
                background: None,
                styles: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn test_color_ansi256() {
        // Arrange
        let color = ColorType::Ansi256(Ansi256::new(86));

        // Act
        let actual = styled("Hello, world!").color(color.clone());

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: Some(color),
                background: None,
                styles: BTreeSet::new(),
            }
        );
        assert_eq!(format!("{}", actual), "\x1b[38;5;86mHello, world!\x1b[0m");
    }

    #[test]
    fn test_color_true_color() {
        // Arrange
        let color = ColorType::TrueColor(RGB::new(30, 215, 96));

        // Act
        let actual = styled("Hello, world!").color(color.clone());

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: Some(color),
                background: None,
                styles: BTreeSet::new(),
            }
        );
        assert_eq!(
            format!("{}", actual),
            "\x1b[38;2;30;215;96mHello, world!\x1b[0m"
        );
    }

    #[test]
    fn test_background() {
        // Arrange
        let color = ColorType::TrueColor(RGB::new(30, 215, 96));

        // Act
        let actual = styled("Hello, world!").background(color.clone());

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: None,
                background: Some(color),
                styles: BTreeSet::new(),
            }
        );
        assert_eq!(
            format!("{}", actual),
            "\x1b[48;2;30;215;96mHello, world!\x1b[0m"
        );
    }

    #[rstest]
    #[case::bold(styled("Hello, world!").bold(), vec![Style::Bold], "\x1b[1mHello, world!\x1b[0m")]
    #[case::dim(styled("Hello, world!").dim(), vec![Style::Dim], "\x1b[2mHello, world!\x1b[0m")]
    #[case::italic(styled("Hello, world!").italic(), vec![Style::Italic], "\x1b[3mHello, world!\x1b[0m")]
    #[case::underline(styled("Hello, world!").underline(), vec![Style::Underline], "\x1b[4mHello, world!\x1b[0m")]
    #[case::blink(styled("Hello, world!").blink(), vec![Style::Blink], "\x1b[5mHello, world!\x1b[0m")]
    #[case::reverse(styled("Hello, world!").reverse(), vec![Style::Reverse], "\x1b[7mHello, world!\x1b[0m")]
    #[case::hidden(styled("Hello, world!").hidden(), vec![Style::Hidden], "\x1b[8mHello, world!\x1b[0m")]
    fn test_styled(
        #[case] actual: StyledString,
        #[case] styles: Vec<Style>,
        #[case] expected: &str,
    ) {
        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: None,
                background: None,
                styles: styles.iter().cloned().collect(),
            }
        );
        assert_eq!(format!("{}", actual), expected);
    }

    #[test]
    fn test_styled_no_style() {
        // Act
        let actual = styled("Hello, world!");

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: None,
                background: None,
                styles: BTreeSet::new(),
            }
        );
        assert_eq!(format!("{}", actual), "Hello, world!");
    }

    #[test]
    fn test_styled_multiple_styles() {
        // Act
        let actual = styled("Hello, world!")
            .color(ColorType::TrueColor(RGB::new(255, 255, 255)))
            .background(ColorType::TrueColor(RGB::new(30, 215, 96)))
            .bold()
            .italic()
            .underline()
            .reverse();

        // Assert
        assert_eq!(
            actual,
            StyledString {
                value: String::from("Hello, world!"),
                foreground: Some(ColorType::TrueColor(RGB::new(255, 255, 255))),
                background: Some(ColorType::TrueColor(RGB::new(30, 215, 96))),
                styles: [Style::Bold, Style::Italic, Style::Underline, Style::Reverse]
                    .iter()
                    .cloned()
                    .collect(),
            }
        );
        assert_eq!(
            format!("{}", actual),
            "\x1b[38;2;255;255;255m\x1b[48;2;30;215;96m\x1b[1m\x1b[3m\x1b[4m\x1b[7mHello, world!\x1b[0m"
        );
    }
}
