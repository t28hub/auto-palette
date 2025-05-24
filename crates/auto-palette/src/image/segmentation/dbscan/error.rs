use thiserror::Error;

use crate::FloatNumber;

/// Error type for the DBSCAN segmentation algorithm.
#[derive(Debug, PartialEq, Error)]
pub enum DbscanError<T>
where
    T: FloatNumber,
{
    /// Error when the number of segments is invalid.
    #[error("The number of segments must be greater than zero")]
    InvalidSegments,

    /// Error when the minimum number of pixels is invalid.
    #[error("The minimum number of pixels must be greater than zero")]
    InvalidMinPixels,

    /// Error when the epsilon value is invalid.
    #[error("Epsilon must be greater than zero and not NaN: {0}")]
    InvalidEpsilon(T),

    /// Error when the number of pixels do not match the expected length.
    #[error("Expected pixels length {expected}, but got {actual}")]
    UnexpectedLength { actual: usize, expected: usize },
}
