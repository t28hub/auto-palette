use std::str::FromStr;

use auto_palette::{Algorithm, Theme};
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
use web_sys::js_sys::JsString;

#[wasm_bindgen(typescript_custom_section)]
// language=TypeScript
const TYPE_DEFINITION: &'static str = r#"
/**
 * The algorithm of the palette.
 */
export type Algorithm = "dbscan" | "dbscan++" | "kmeans" | "slic" | "snic";

/**
 * The theme of the palette.
 */
export type Theme = "vivid" | "muted" | "light" | "dark" | "colorful";
"#;

#[wasm_bindgen]
extern "C" {
    /// The algorithm of the palette extraction.
    #[wasm_bindgen(typescript_type = "Algorithm")]
    pub type JsAlgorithm;

    /// The theme of the swatch selection.
    #[wasm_bindgen(typescript_type = "Theme")]
    pub type JsTheme;
}

impl TryFrom<JsAlgorithm> for Algorithm {
    type Error = JsError;

    fn try_from(name: JsAlgorithm) -> Result<Self, Self::Error> {
        let value: JsValue = name.into();
        let string: JsString = value.into();
        Algorithm::from_str(
            &string
                .as_string()
                .ok_or_else(|| JsError::new("Failed to retrieve algorithm name"))?,
        )
        .map_err(|_| JsError::new(&format!("Unknown algorithm name: {string}")))
    }
}

impl TryFrom<JsTheme> for Theme {
    type Error = JsError;

    fn try_from(name: JsTheme) -> Result<Self, Self::Error> {
        let value: JsValue = name.into();
        let string: JsString = value.into();
        string
            .as_string()
            .ok_or_else(|| JsError::new("Failed to retrieve theme name"))
            .and_then(|name| {
                Theme::from_str(&name)
                    .map_err(|_| JsError::new(&format!("Unknown theme name: {name}")))
            })
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    use super::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_algorithm_try_from() {
        // Act
        let algorithm: JsAlgorithm = JsValue::from_str("dbscan").into();
        let result: Result<Algorithm, JsError> = Algorithm::try_from(algorithm);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Algorithm::DBSCAN);
    }

    #[wasm_bindgen_test]
    fn test_algorithm_try_from_error() {
        // Act
        let algorithm: JsAlgorithm = JsValue::from_str("unknown").into();
        let result: Result<Algorithm, JsError> = Algorithm::try_from(algorithm);

        // Assert
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_theme_try_from() {
        // Act
        let theme: JsTheme = JsValue::from_str("vivid").into();
        let result: Result<Theme, JsError> = Theme::try_from(theme);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Theme::Vivid);
    }

    #[wasm_bindgen_test]
    fn test_theme_try_from_error() {
        // Act
        let theme: JsTheme = JsValue::from_str("unknown").into();
        let result: Result<Theme, JsError> = Theme::try_from(theme);

        // Assert
        assert!(result.is_err());
    }
}
