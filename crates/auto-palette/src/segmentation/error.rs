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

/// Error returned when constructing a `SegmentationInput` with mismatched dimensions.
#[derive(Debug, PartialEq, Error)]
#[error("dimension mismatch: expected {expected} pixels ({width}x{height}), but got {actual}")]
pub struct DimensionMismatchError {
    /// The declared width.
    pub width: usize,
    /// The declared height.
    pub height: usize,
    /// The expected number of pixels (`width * height`).
    pub expected: usize,
    /// The actual number of pixels provided.
    pub actual: usize,
}
