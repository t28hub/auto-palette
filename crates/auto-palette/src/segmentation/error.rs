use thiserror::Error;

use crate::math::matrix::MatrixError;

/// Error type for segmentation algorithms.
#[derive(Debug, PartialEq, Error)]
pub enum SegmentationError {
    /// Error when an argument is invalid.
    #[error("{0}")]
    InvalidArgument(String),

    /// Error when the number of pixels is invalid.
    #[error("Unexpected pixels length: {0}")]
    UnexpectedLength(#[from] MatrixError),
}

/// Error returned when constructing a `SegmentationInput` with invalid dimensions.
#[derive(Debug, PartialEq, Error)]
pub enum DimensionMismatchError {
    /// The product of width and height overflows `usize`.
    #[error("dimension overflow: {width} x {height} exceeds usize")]
    Overflow {
        /// The declared width.
        width: usize,
        /// The declared height.
        height: usize,
    },

    /// The number of pixels or mask entries does not match `width * height`.
    #[error("dimension mismatch: expected {expected} pixels ({width}x{height}), but got {actual}")]
    LengthMismatch {
        /// The declared width.
        width: usize,
        /// The declared height.
        height: usize,
        /// The expected number of pixels (`width * height`).
        expected: usize,
        /// The actual number of pixels provided.
        actual: usize,
    },
}
