use serde::{Deserialize, Serialize};

/// Struct for representing a position.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// The x coordinate.
    pub x: u32,
    /// The y coordinate.
    pub y: u32,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_tokens() {
        // Act
        let actual = Position { x: 64, y: 32 };

        // Assert
        assert_tokens(
            &actual,
            &[
                Token::Struct {
                    name: "Position",
                    len: 2,
                },
                Token::Str("x"),
                Token::U32(64),
                Token::Str("y"),
                Token::U32(32),
                Token::StructEnd,
            ],
        );
    }
}
