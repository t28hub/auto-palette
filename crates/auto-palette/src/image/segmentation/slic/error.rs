use std::fmt::Display;

use thiserror::Error;

use crate::{math::matrix::MatrixError, FloatNumber};

/// SLIC algorithm error type.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum SlicError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the number of segments is invalid.
    #[error("The number of segments must be greater than zero: {0}")]
    InvalidSegments(usize),

    /// Error when the compactness is invalid.
    #[error("Compactness must be greater than zero: {0}")]
    InvalidCompactness(T),

    /// Error when the number of iterations is invalid.
    #[error("Iterations must be greater than zero: {0}")]
    InvalidIterations(usize),

    /// Error when the tolerance is invalid.
    #[error("Tolerance must be greater than zero: {0}")]
    InvalidTolerance(T),

    /// Error when the pixels slice is empty.
    #[error("The pixels slic is empty")]
    EmptyPixels,

    /// Error when the number of pixels is invalid.
    #[error("Unexpected pixels length: {0}")]
    UnexpectedLength(#[from] MatrixError),
}
