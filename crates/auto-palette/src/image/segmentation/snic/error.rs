use thiserror::Error;

use crate::math::matrix::MatrixError;

/// SNIC algorithm error type.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum SnicError {
    /// Error when the number of segments is invalid.
    #[error("The number of segments must be greater than zero: {0}")]
    InvalidSegments(usize),

    /// Error when the pixels slice is empty.
    #[error("The pixels slice is empty")]
    EmptyPixels,

    /// Error when the number of pixels is invalid.
    #[error("Unexpected pixels length: {0}")]
    UnexpectedLength(#[from] MatrixError),
}
