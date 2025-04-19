use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// The position representation of a swatch.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Tsify)]
#[serde(rename = "Position")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JsPosition {
    /// The x coordinate of the swatch.
    pub x: u32,
    /// The y coordinate of the swatch.
    pub y: u32,
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_serialize() {
        // Act
        let position = JsPosition { x: 16, y: 32 };

        // Assert
        assert_ser_tokens(
            &position,
            &[
                Token::Struct {
                    name: "Position",
                    len: 2,
                },
                Token::Str("x"),
                Token::U32(16),
                Token::Str("y"),
                Token::U32(32),
                Token::StructEnd,
            ],
        );
    }

    #[wasm_bindgen_test]
    fn test_deserialize() {
        // Act
        let position = JsPosition { x: 32, y: 16 };

        // Assert
        assert_de_tokens(
            &position,
            &[
                Token::Struct {
                    name: "Position",
                    len: 2,
                },
                Token::Str("x"),
                Token::U32(32),
                Token::Str("y"),
                Token::U32(16),
                Token::StructEnd,
            ],
        );
    }

    #[wasm_bindgen_test]
    fn test_tsify() {
        // Assert
        let expected = indoc! {
            // language=TypeScript
            "/**
              * The position representation of a swatch.
              */
             export interface JsPosition {
                 /**
                  * The x coordinate of the swatch.
                  */
                 x: number;
                 /**
                  * The y coordinate of the swatch.
                  */
                 y: number;
             }"
        };
        assert_eq!(JsPosition::DECL, expected);
    }
}
