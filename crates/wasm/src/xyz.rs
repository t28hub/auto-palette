use serde::{Deserialize, Serialize};

/// Struct representing a CIE XYZ color.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct XYZ {
    /// The x component of the color.
    pub x: f32,
    /// The y component of the color.
    pub y: f32,
    /// The z component of the color.
    pub z: f32,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_tokens() {
        // Act
        let actual = XYZ {
            x: 0.147_161,
            y: 0.241_450,
            z: 0.198_049,
        };

        // Assert
        assert_tokens(
            &actual,
            &[
                Token::Struct {
                    name: "XYZ",
                    len: 3,
                },
                Token::Str("x"),
                Token::F32(0.147_161),
                Token::Str("y"),
                Token::F32(0.241_450),
                Token::Str("z"),
                Token::F32(0.198_049),
                Token::StructEnd,
            ],
        );
    }
}
