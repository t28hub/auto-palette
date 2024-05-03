use std::str::FromStr;

use auto_palette::Algorithm;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

/// Struct for wrapping `Algorithm` in auto-palette
#[wasm_bindgen]
#[derive(Debug)]
pub struct AlgorithmWrapper(pub(super) Algorithm);

#[wasm_bindgen]
impl AlgorithmWrapper {
    /// Creates an `AlgorithmWrapper` from the given string.
    ///
    /// # Arguments
    /// * `s` - The string representation of the algorithm.
    ///
    /// # Returns
    /// The `AlgorithmWrapper` if successful, otherwise an error.
    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(s: &str) -> Result<AlgorithmWrapper, JsValue> {
        Algorithm::from_str(s)
            .map(AlgorithmWrapper)
            .map_err(|_| JsValue::from_str(format!("Unknown algorithm name: {}", s).as_str()))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[rstest]
    #[case::kmeans("kmeans", Algorithm::KMeans)]
    #[case::dbscan("dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp("dbscan++", Algorithm::DBSCANpp)]
    fn test_from_string(#[case] s: &str, #[case] expected: Algorithm) {
        // Act
        let actual = AlgorithmWrapper::from_string(s).unwrap();

        // Assert
        assert_eq!(actual.0, expected);
    }

    #[wasm_bindgen_test]
    fn test_from_string_unknown() {
        // Act
        let actual = AlgorithmWrapper::from_string("unknown");

        // Assert
        assert!(actual.is_err());
    }

    #[wasm_bindgen_test]
    fn test_from_string_empty() {
        // Act
        let actual = AlgorithmWrapper::from_string("");

        // Assert
        assert!(actual.is_err());
    }
}
