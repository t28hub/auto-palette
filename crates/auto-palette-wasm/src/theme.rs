use std::str::FromStr;

use auto_palette::Theme;
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct for wrapping `Theme` in auto-palette
#[wasm_bindgen]
#[derive(Debug)]
pub struct ThemeWrapper(pub(super) Theme);

#[wasm_bindgen]
impl ThemeWrapper {
    /// Creates a new `ThemeWrapper` from the given string.
    ///
    /// # Arguments
    /// * `s` - The string representation of the theme.
    ///
    /// # Returns
    /// The `ThemeWrapper` if successful, otherwise an error.
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(s: &str) -> Result<ThemeWrapper, wasm_bindgen::JsValue> {
        Theme::from_str(s).map(ThemeWrapper).map_err(|_| {
            wasm_bindgen::JsValue::from_str(format!("Unknown theme name: {}", s).as_str())
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[rstest]
    #[case::basic("basic", Theme::Basic)]
    #[case::colorful("colorful", Theme::Colorful)]
    #[case::vivid("vivid", Theme::Vivid)]
    #[case::muted("muted", Theme::Muted)]
    #[case::light("light", Theme::Light)]
    #[case::dark("dark", Theme::Dark)]
    fn test_from_string(#[case] s: &str, #[case] expected: Theme) {
        // Act
        let actual = ThemeWrapper::from_string(s).unwrap();

        // Assert
        assert_eq!(actual.0, expected);
    }

    #[wasm_bindgen_test]
    fn test_from_string_unknown() {
        // Act
        let actual = ThemeWrapper::from_string("unknown");

        // Assert
        assert!(actual.is_err());
    }

    #[wasm_bindgen_test]
    fn test_from_string_empty() {
        // Act
        let actual = ThemeWrapper::from_string("");

        // Assert
        assert!(actual.is_err());
    }
}
