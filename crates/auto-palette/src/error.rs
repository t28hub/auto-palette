#[cfg(feature = "image")]
use image::ImageError;
use thiserror::Error;

/// The `Error` enum represents the errors that can occur in the palette extraction process.
#[derive(Debug, Error)]
pub enum Error {
    /// The image data is empty and cannot be processed.
    #[error("The image data is empty and cannot be processed.")]
    EmptyImageData,

    /// The image data contains invalid pixel data.
    #[error("The image data contains invalid pixel data.")]
    InvalidImageData,

    /// The palette extraction process failed.
    /// The details provide more information about the error.
    #[error("The palette extraction process failed with error: {0}")]
    ExtractionFailure(String),

    /// The algorithm is not supported.
    /// The name provides more information about the unsupported algorithm.
    #[error("The algorithm '{0}' is not supported.")]
    UnsupportedAlgorithm(String),

    /// The theme is not supported.
    /// The name provides more information about the unsupported theme.
    #[error("The theme '{0}' is not supported.")]
    UnsupportedTheme(String),

    /// The image loading process failed.
    /// The cause provides more information about the error.
    #[cfg(feature = "image")]
    #[error("The image loading process failed with error: {0}")]
    ImageLoadError(#[from] ImageError),

    /// The color type of the image is not supported.
    #[cfg(feature = "image")]
    #[error("The image format is not supported.")]
    UnsupportedImage,
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
            "The image data is empty and cannot be processed."
        );
    }

    #[test]
    fn test_invalid_image_data() {
        // Act
        let actual = Error::InvalidImageData;

        // Assert
        assert_eq!(
            actual.to_string(),
            "The image data contains invalid pixel data."
        );
    }

    #[test]
    fn test_extraction_failure() {
        // Act
        let actual = Error::ExtractionFailure("Failed to extract palette.".to_string());

        // Assert
        assert_eq!(
            actual.to_string(),
            "The palette extraction process failed with error: Failed to extract palette."
        );
    }

    #[test]
    fn test_unsupported_algorithm() {
        // Act
        let actual = Error::UnsupportedAlgorithm("unknown_algorithm".to_string());

        // Assert
        assert_eq!(
            actual.to_string(),
            "The algorithm 'unknown_algorithm' is not supported."
        );
    }

    #[test]
    fn test_unsupported_theme() {
        // Act
        let actual = Error::UnsupportedTheme("unknown_theme".to_string());

        // Assert
        assert_eq!(
            actual.to_string(),
            "The theme 'unknown_theme' is not supported."
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_image_load_error() {
        // Arrange
        let cause = ImageError::IoError(std::io::Error::from(std::io::ErrorKind::NotFound));
        let error = Error::ImageLoadError(cause);

        // Act
        let actual = error.to_string();

        // Assert
        assert_eq!(
            actual,
            "The image loading process failed with error: entity not found"
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_unsupported_image() {
        // Act
        let actual = Error::UnsupportedImage;

        // Assert
        assert_eq!(actual.to_string(), "The image format is not supported.");
    }
}
