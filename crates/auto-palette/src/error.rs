use thiserror::Error;

use crate::math::sampling::SamplingError;

/// Represents specific errors encountered during the palette extraction process.
#[derive(Debug, Error)]
pub enum Error {
    /// Error when the palette extraction process fails, providing the underlying details.
    #[error("Palette extraction process failed with error: {details}")]
    PaletteExtractionError {
        /// The underlying cause of the palette extraction failure.
        details: String,
    },

    /// Error when the swatch selection process fails, providing the underlying details.
    #[error("Swatch selection process failed with error: {cause}")]
    SwatchSelectionError {
        #[from]
        cause: SamplingError,
    },

    /// Error when an unsupported algorithm is specified.
    #[error("Unsupported algorithm specified: '{name}'")]
    UnsupportedAlgorithm {
        /// The name of the unsupported algorithm.
        name: String,
    },

    /// Error when an unsupported theme is specified.
    #[error("Unsupported theme specified: '{name}'")]
    UnsupportedTheme {
        /// The name of the unsupported theme.
        name: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_extraction_error() {
        // Act
        let actual = Error::PaletteExtractionError {
            details: "Details about the failure.".to_string(),
        };

        // Assert
        assert_eq!(
            actual.to_string(),
            "Palette extraction process failed with error: Details about the failure."
        );
    }

    #[test]
    fn test_swatches_selection_error() {
        // Act
        let cause = SamplingError::InvalidDiversity;
        let actual = Error::SwatchSelectionError { cause };

        // Assert
        assert_eq!(
            actual.to_string(),
            "Swatch selection process failed with error: Invalid diversity: Diversity score must be between 0.0 and 1.0."
        );
    }

    #[test]
    fn test_unsupported_algorithm() {
        // Act
        let actual = Error::UnsupportedAlgorithm {
            name: "xmeans".to_string(),
        };

        // Assert
        assert_eq!(
            actual.to_string(),
            "Unsupported algorithm specified: 'xmeans'"
        );
    }

    #[test]
    fn test_unsupported_theme() {
        // Act
        let actual = Error::UnsupportedTheme {
            name: "pastel".to_string(),
        };

        // Assert
        assert_eq!(actual.to_string(), "Unsupported theme specified: 'pastel'");
    }
}
