#[cfg(feature = "image")]
use image::ImageError;

/// Error might occur during the palette extraction process.
#[derive(Debug)]
pub enum Error {
    /// The image data is empty and cannot be processed.
    EmptyImageData,

    /// The image data contains invalid pixel data.
    InvalidImageData,

    /// The palette extraction process failed.
    /// The details provide more information about the error.
    ExtractionFailure { details: String },

    /// The algorithm is not supported.
    /// The name provides more information about the unsupported algorithm.
    UnsupportedAlgorithm { name: String },

    /// The theme is not supported.
    /// The name provides more information about the unsupported theme.
    UnsupportedTheme { name: String },

    /// The image loading process failed.
    /// The cause provides more information about the error.
    #[cfg(feature = "image")]
    ImageLoadError { cause: ImageError },

    /// The color type of the image is not supported.
    #[cfg(feature = "image")]
    UnsupportedImage,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidImageData => write!(f, "The image data contains invalid pixel data."),
            Error::EmptyImageData => {
                write!(f, "The image data is empty and cannot be processed.")
            }
            Error::ExtractionFailure { details } => {
                write!(
                    f,
                    "The palette extraction process failed with error: {}",
                    details
                )
            }
            Error::UnsupportedAlgorithm { name } => {
                write!(f, "The algorithm '{}' is not supported.", name)
            }
            Error::UnsupportedTheme { name } => {
                write!(f, "The theme '{}' is not supported.", name)
            }
            #[cfg(feature = "image")]
            Error::ImageLoadError { cause } => {
                write!(f, "The image loading process failed with error: {}", cause)
            }
            #[cfg(feature = "image")]
            Error::UnsupportedImage => {
                write!(f, "The image format is not supported.")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fmt_empty_image_data() {
        // Act
        let actual = Error::EmptyImageData;

        // Assert
        assert_eq!(
            actual.to_string(),
            "The image data is empty and cannot be processed."
        );
    }

    #[test]
    fn test_fmt_invalid_image_data() {
        // Act
        let actual = Error::InvalidImageData;

        // Assert
        assert_eq!(
            actual.to_string(),
            "The image data contains invalid pixel data."
        );
    }

    #[test]
    fn test_fmt_extraction_failure() {
        // Act
        let actual = Error::ExtractionFailure {
            details: "Failed to extract palette.".to_string(),
        };

        // Assert
        assert_eq!(
            actual.to_string(),
            "The palette extraction process failed with error: Failed to extract palette."
        );
    }

    #[test]
    fn test_fmt_unsupported_algorithm() {
        // Act
        let actual = Error::UnsupportedAlgorithm {
            name: "unknown_algorithm".to_string(),
        };

        // Assert
        assert_eq!(
            actual.to_string(),
            "The algorithm 'unknown_algorithm' is not supported."
        );
    }

    #[test]
    fn test_fmt_unsupported_theme() {
        // Act
        let actual = Error::UnsupportedTheme {
            name: "unknown_theme".to_string(),
        };

        // Assert
        assert_eq!(
            actual.to_string(),
            "The theme 'unknown_theme' is not supported."
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_fmt_image_load_error() {
        // Arrange
        let cause = ImageError::IoError(std::io::Error::from(std::io::ErrorKind::NotFound));
        let error = Error::ImageLoadError { cause };

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
    fn test_fmt_unsupported_image() {
        // Act
        let actual = Error::UnsupportedImage;

        // Assert
        assert_eq!(actual.to_string(), "The image format is not supported.");
    }
}
