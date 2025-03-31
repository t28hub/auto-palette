#[cfg(feature = "image")]
use image::ImageError;
use thiserror::Error;

use crate::math::sampling::SamplingError;

/// Represents specific errors encountered during the palette extraction process.
#[derive(Debug, Error)]
pub enum Error {
    /// Error when provided image data is empty and contains no pixel information.
    #[error("Image data is empty: no pixels to process")]
    EmptyImageData,

    /// Error when the provided image data contains invalid pixel information.
    #[error("Image data is invalid: contains invalid pixel data")]
    InvalidImageData,

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

    /// Error when the image fails to load, providing the underlying cause.
    #[cfg(feature = "image")]
    #[error("Image loading process failed with error: {cause}")]
    ImageLoadError {
        /// The underlying cause of the image loading failure.
        #[from]
        cause: ImageError,
    },

    /// Error when the image format or color type is not supported.
    #[cfg(feature = "image")]
    #[error("Image format or color type is not supported")]
    UnsupportedImageFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_image_data() {
        // Act
        let actual = Error::EmptyImageData;

        // Assert
        assert_eq!(
            actual.to_string(),
            "Image data is empty: no pixels to process"
        );
    }

    #[test]
    fn test_invalid_image_data() {
        // Act
        let actual = Error::InvalidImageData;

        // Assert
        assert_eq!(
            actual.to_string(),
            "Image data is invalid: contains invalid pixel data"
        );
    }

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

    #[test]
    #[cfg(feature = "image")]
    fn test_image_load_error() {
        // Arrange
        let cause = ImageError::IoError(std::io::Error::from(std::io::ErrorKind::NotFound));
        let error = Error::ImageLoadError { cause };

        // Act
        let actual = error.to_string();

        // Assert
        assert_eq!(
            actual,
            "Image loading process failed with error: entity not found"
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_unsupported_image() {
        // Act
        let actual = Error::UnsupportedImageFormat;

        // Assert
        assert_eq!(
            actual.to_string(),
            "Image format or color type is not supported"
        );
    }
}
