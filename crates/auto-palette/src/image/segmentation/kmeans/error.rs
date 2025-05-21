use thiserror::Error;

use crate::FloatNumber;

/// K-means segmentation algorithm error types.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum KmeansError<T>
where
    T: FloatNumber,
{
    /// Error when the number of segments is invalid.
    #[error("The number of segments must be greater than zero")]
    InvalidSegments,

    /// Error when the number of iterations is invalid.
    #[error("The number of iterations must be greater than zero")]
    InvalidIterations,

    /// Error when the tolerance is invalid.
    #[error("Tolerance must be greater than zero and not NaN: {0}")]
    InvalidTolerance(T),

    /// Error when the number of pixels do not match the expected length.
    #[error("Unexpected pixels length: {actual}, expected: {expected}")]
    UnexpectedLength { actual: usize, expected: usize },
}
