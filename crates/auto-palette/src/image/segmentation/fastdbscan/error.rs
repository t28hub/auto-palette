use thiserror::Error;

/// Error type for the FastDBSCAN (DBSCAN++) segmentation algorithm.
#[derive(Debug, PartialEq, Error)]
pub enum FastDbscanError<T> {
    /// Error when the minimum number of pixels is invalid.
    #[error("The minimum number of pixels must be greater than zero")]
    InvalidMinPixels,

    /// Error when the epsilon value is invalid.
    #[error("The epsilon value must be greater than zero and not NaN: {0}")]
    InvalidEpsilon(T),

    /// Error when the probability value is out of range.
    #[error("The probability value must be in the range (0, 1]: {0}")]
    OutOfRangeProbability(T),

    /// Error when the number of pixels do not match the expected length.
    #[error("Expected pixels length {expected}, but got {actual}")]
    UnexpectedLength { actual: usize, expected: usize },
}
