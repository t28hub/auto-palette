use std::fmt::Display;

use thiserror::Error;

use crate::FloatNumber;

/// K-means clustering algorithm error types.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum KmeansError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the number of clusters is invalid.
    #[error("The number of clusters must be greater than zero: {0}")]
    InvalidClusterCount(usize),

    /// Error when the number of iterations is invalid.
    #[error("The number of iterations must be greater than zero: {0}")]
    InvalidIterations(usize),

    /// Error when the tolerance is invalid.
    #[error("Tolerance must be greater than zero: {0}")]
    InvalidTolerance(T),

    /// Error when the points slice is empty.
    #[error("The points slice is empty")]
    EmptyPoints,
}
