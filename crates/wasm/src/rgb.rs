use serde::{Deserialize, Serialize};

/// Struct representing an RGB color.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RGB {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_tokens() {
        // Act
        let actual = RGB {
            r: 20,
            g: 153,
            b: 114,
        };

        // Assert
        assert_tokens(
            &actual,
            &[
                Token::Struct {
                    name: "RGB",
                    len: 3,
                },
                Token::Str("r"),
                Token::U8(20),
                Token::Str("g"),
                Token::U8(153),
                Token::Str("b"),
                Token::U8(114),
                Token::StructEnd,
            ],
        );
    }
}
