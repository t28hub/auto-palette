use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TYPE_DEFINITION: &'static str = r#"
/**
 * The position representation in an image.
 */
export interface Position {
    /**
     * The x-coordinate of the position.
     */
    readonly x: number;

    /**
     * The y-coordinate of the position.
     */
    readonly y: number;
}
"#;

/// The position of a color in the image.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "Position")]
#[wasm_bindgen(js_name = Position, skip_typescript)]
pub struct JsPosition {
    /// The x-coordinate of the position.
    pub x: u32,
    /// The y-coordinate of the position.
    pub y: u32,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
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

    #[test]
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
}
