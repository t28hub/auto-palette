use serde::{Deserialize, Serialize};

/// Struct representing a CIE L*a*b* color.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Lab {
    /// The lightness component of the color.
    pub l: f32,
    /// The a* component of the color.
    pub a: f32,
    /// The b* component of the color.
    pub b: f32,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_tokens() {
        // Act
        let actual = Lab {
            l: 56.232_69,
            a: -42.861_58,
            b: 11.220_444,
        };

        // Assert
        assert_tokens(
            &actual,
            &[
                Token::Struct {
                    name: "Lab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F32(56.232_69),
                Token::Str("a"),
                Token::F32(-42.861_58),
                Token::Str("b"),
                Token::F32(11.220_444),
                Token::StructEnd,
            ],
        );
    }
}
