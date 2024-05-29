use std::env;

/// The environment variables.
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub colorterm: Option<String>,
    pub no_color: Option<String>,
}

impl Env {
    /// Initializes the environment variables.
    ///
    /// # Returns
    /// The environment variables.
    #[must_use]
    pub fn init() -> Self {
        Self {
            colorterm: env::var("COLORTERM").ok(),
            no_color: env::var("NO_COLOR").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        // Arrange
        env::set_var("COLORTERM", "truecolor");
        env::set_var("NO_COLOR", "1");

        // Act
        let actual = Env::init();

        // Assert
        assert_eq!(
            actual,
            Env {
                colorterm: Some("truecolor".to_string()),
                no_color: Some("1".to_string()),
            }
        );
        env::remove_var("COLORTERM");
        env::remove_var("NO_COLOR");
    }
}
