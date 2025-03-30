use thiserror::Error;

use crate::FloatNumber;

/// Errors that can occur during sampling.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Error, PartialEq)]
pub enum SamplingError<T>
where
    T: FloatNumber,
{
    /// An error that occurs when the input points are empty.
    #[error("Empty points: no points to sample from.")]
    EmptyPoints,

    /// An error that occurs when the input weights are empty.
    #[error("Empty weights: no weights to sample from.")]
    EmptyWeights,

    /// An error that occurs when the diversity is out of range (0.0, 1.0).
    #[error("Invalid diversity: {diversity}. Diversity score must be between 0.0 and 1.0.")]
    InvalidDiversity { diversity: T },

    /// An error that occurs when the length of the points and weights do not match.
    #[error("Points length ({points_len}) and weights length ({weights_len}) mismatch.")]
    WeightsLengthMismatch {
        points_len: usize,
        weights_len: usize,
    },
}
