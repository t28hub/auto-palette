use thiserror::Error;

/// A specialized [`Result`](std::result::Result) type for palette operations.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type for palette operations.
///
/// This is a single flat enum so that callers — including FFI bindings for
/// WebAssembly, Android, and iOS — can match on it directly without walking
/// a nested error hierarchy.
///
/// Use [`Display`](std::fmt::Display) for user-facing messages.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// The length of the pixel data does not match the image dimensions.
    #[error("unexpected data length: expected {expected}, got {actual}")]
    InvalidImageData {
        /// The expected length (`width * height * 4`).
        expected: usize,
        /// The actual length of the provided data.
        actual: usize,
    },

    /// The image failed to load from a file.
    #[cfg(feature = "image")]
    #[error("failed to load image: {source}")]
    ImageLoad {
        /// The underlying cause.
        #[from]
        source: image::ImageError,
    },

    /// The image format or color type is not supported.
    #[cfg(feature = "image")]
    #[error("unsupported image format or color type")]
    UnsupportedImageFormat,

    /// A configuration parameter for palette extraction is invalid.
    #[error("invalid parameter for palette extraction: {reason}")]
    InvalidParameter {
        /// A human-readable description of the invalid parameter.
        reason: String,
    },

    /// The image dimensions do not match the pixel data.
    #[error("image dimensions do not match pixel data")]
    DimensionMismatch,

    /// The requested algorithm name is not recognized.
    #[error("unsupported algorithm: '{0}'")]
    UnknownAlgorithm(String),

    /// The requested theme name is not recognized.
    #[error("unsupported theme: '{0}'")]
    UnknownTheme(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_satisfies_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn test_invalid_image_data() {
        let actual = Error::InvalidImageData {
            expected: 100,
            actual: 50,
        };
        assert_eq!(
            actual.to_string(),
            "unexpected data length: expected 100, got 50"
        );
    }

    #[test]
    fn test_invalid_parameter() {
        let actual = Error::InvalidParameter {
            reason: "segments must be greater than 0".to_string(),
        };
        assert_eq!(
            actual.to_string(),
            "invalid parameter for palette extraction: segments must be greater than 0"
        );
    }

    #[test]
    fn test_dimension_mismatch() {
        let actual = Error::DimensionMismatch;
        assert_eq!(
            actual.to_string(),
            "image dimensions do not match pixel data"
        );
    }

    #[test]
    fn test_unknown_algorithm() {
        let actual = Error::UnknownAlgorithm("xmeans".to_string());
        assert_eq!(actual.to_string(), "unsupported algorithm: 'xmeans'");
    }

    #[test]
    fn test_unknown_theme() {
        let actual = Error::UnknownTheme("pastel".to_string());
        assert_eq!(actual.to_string(), "unsupported theme: 'pastel'");
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_unsupported_image_format() {
        let actual = Error::UnsupportedImageFormat;
        assert_eq!(actual.to_string(), "unsupported image format or color type");
    }
}
