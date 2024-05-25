use std::env;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    colorterm: Option<String>,
    no_color: Option<String>,
}

impl Env {
    #[must_use]
    pub fn new() -> Self {
        Self {
            colorterm: env::var("COLORTERM").ok(),
            no_color: env::var("NO_COLOR").ok(),
        }
    }

    #[must_use]
    pub fn is_truecolor_enabled(&self) -> bool {
        self.colorterm
            .as_deref()
            .map(|v| v.to_lowercase() == "truecolor" || v.to_lowercase() == "24bit")
            .unwrap_or(false)
    }

    #[must_use]
    pub fn is_color_disabled(&self) -> bool {
        self.no_color
            .as_deref()
            .map(|v| !v.is_empty() && v != "0")
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Arrange
        // env::remove_var("COLORTERM");
        // env::remove_var("NO_COLOR");
        env::set_var("COLORTERM", "truecolor");
        env::set_var("NO_COLOR", "1");

        // Act
        let actual = Env::new();

        // Assert
        assert_eq!(
            actual,
            Env {
                colorterm: Some("truecolor".to_string()),
                no_color: Some("1".to_string()),
            }
        );
    }

    #[test]
    fn test_new_with_no_env() {
        // Arrange
        env::remove_var("COLORTERM");
        env::remove_var("NO_COLOR");

        // Act
        let actual = Env::new();

        // Assert
        assert_eq!(
            actual,
            Env {
                colorterm: None,
                no_color: None,
            }
        );
    }

    #[rstest]
    #[case("truecolor", true)]
    #[case("24bit", true)]
    #[case("8bit", false)]
    #[case("", false)]
    fn test_is_truecolor_enabled(#[case] value: &str, #[case] expected: bool) {
        // Arrange
        env::set_var("COLORTERM", value);

        // Act
        let actual = Env::new().is_truecolor_enabled();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("1", true)]
    #[case("true", true)]
    #[case("false", true)]
    #[case("0", false)]
    #[case("", false)]
    fn test_is_color_disabled(#[case] value: &str, #[case] expected: bool) {
        // Arrange
        env::set_var("NO_COLOR", value);

        // Act
        let actual = Env::new().is_color_disabled();

        // Assert
        assert_eq!(actual, expected);
    }
}
