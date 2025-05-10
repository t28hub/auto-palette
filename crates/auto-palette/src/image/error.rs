use thiserror::Error;

/// Represents the result type for image processing operations.
///
/// # Type Parameters
/// * `T` - The type of the result.
pub type ImageResult<T> = Result<T, ImageError>;

/// Represents specific errors encountered during image processing.
#[derive(Debug, Error)]
pub enum ImageError {
    /// Error when the length of the image data is unexpected.
    #[error("Unexpected data length - expected {expected}, got {actual}")]
    UnexpectedLength { expected: usize, actual: usize },

    /// Error when the image fails to load, providing the underlying cause.
    #[cfg(feature = "image")]
    #[error("Failed to load image from file: {source}")]
    LoadError {
        #[from]
        source: image::ImageError,
    },

    /// Error when the image format or color type is not supported.
    #[cfg(feature = "image")]
    #[error("Unsupported image format or color type")]
    UnsupportedFormat,
}
