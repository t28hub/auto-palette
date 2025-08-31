use thiserror::Error;

/// Errors that can occur during point sampling operations.
///
/// This enum represents various failure conditions that may arise when
/// using sampling algorithms to select points from a dataset. Each variant
/// provides specific information about what went wrong to help with debugging
/// and error handling.
#[derive(Debug, Error, PartialEq)]
pub enum SamplingError {
    /// The input dataset contains no points to sample from.
    ///
    /// This error occurs when attempting to sample from an empty point array.
    /// Most sampling algorithms require at least one point to function correctly.
    #[error("empty points: cannot sample from an empty dataset")]
    EmptyPoints,

    /// The weights vector is empty.
    ///
    /// This error occurs when weighted sampling algorithms receive an empty
    /// weights vector. Weighted algorithms require at least one weight value
    /// to determine point selection probabilities.
    #[error("empty weights: weight vector must contain at least one element")]
    EmptyWeights,

    /// The diversity factor is outside the valid range [0, 1].
    ///
    /// This error occurs when the diversity factor parameter is not within
    /// the expected range. The diversity factor controls the trade-off between
    /// selecting high-scoring points and maintaining diversity in the selection.
    ///
    /// - A value of 0 means pure score-based selection
    /// - A value of 1 means pure diversity-based selection
    /// - Values between 0 and 1 balance both criteria
    #[error("invalid diversity factor: value must be in range [0.0, 1.0]")]
    DiversityOutOfRange,

    /// The number of points and weights do not match.
    ///
    /// This error occurs when the length of the points array differs from
    /// the length of the weights array in weighted sampling algorithms.
    /// Each point must have exactly one corresponding weight.
    ///
    /// # Fields
    /// * `points_len` - The number of points in the dataset
    /// * `weights_len` - The number of weights provided
    #[error("length mismatch: points array has {points_len} elements but weights array has {weights_len}")]
    LengthMismatch {
        /// The number of points in the dataset.
        points_len: usize,
        /// The number of weights provided.
        weights_len: usize,
    },
}
