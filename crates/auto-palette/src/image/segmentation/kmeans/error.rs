use thiserror::Error;

use crate::{math::clustering::KmeansError as KmeansClusteringError, FloatNumber};

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

    /// Error when the parameters for K-means clustering are invalid.
    #[error("Parameters for K-means clustering are invalid: {0}")]
    InvalidParameters(#[from] KmeansClusteringError<T>),
}
