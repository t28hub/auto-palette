use std::fmt::Display;

use crate::color::ColorMode;

/// The display style in the terminal.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Style {
    background: Option<ColorMode>,
}

impl Style {
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

/// The styled string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyledString {
    style: Style,
    value: String,
}

impl Display for StyledString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(background) = &self.style.background {
            write!(f, "\x1b[{}m{}\x1b[0m", background.bg_code(), self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// Creates a new `Style` instance.
///
/// # Returns
/// A new `Style` instance.
#[inline]
#[must_use]
pub fn style() -> Style {
    Style::default()
}

#[cfg(test)]
mod tests {
    use auto_palette::color::RGB;

    use super::*;

    #[test]
    fn test_default() {
        // Act
        let actual = Style::default();

        // Assert
        assert_eq!(actual, Style { background: None });
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
                background: Some(color),
            }
        );

        let actual = actual.apply("Hello, world!");
        assert_eq!(
            format!("{}", actual),
            "\x1b[48;2;30;215;96mHello, world!\x1b[0m"
        );
    }

    #[test]
    fn test_styled_no_style() {
        // Act
        let actual = style().apply("Hello, world!");

        // Assert
        assert_eq!(format!("{}", actual), "Hello, world!");
    }
}
