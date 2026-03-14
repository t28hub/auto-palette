use crate::{
    math::FloatNumber,
    segmentation::{
        error::SegmentationError,
        input::SegmentationInput,
        result::SegmentationResult,
    },
};

/// Trait for segmentation algorithms.
///
/// Implementations receive a validated `SegmentationInput` and split it into
/// coherent segments.
///
/// # Type Parameters
/// * `T` - The floating point type.
pub trait Segmentation<T>
where
    T: FloatNumber,
{
    /// Splits the given image into segments.
    ///
    /// # Arguments
    /// * `input` - A validated segmentation input containing pixels, mask, and dimensions.
    ///
    /// # Returns
    /// A `SegmentationResult` representing the segments, or an error if segmentation fails.
    fn segment(
        &self,
        input: &SegmentationInput<'_, T>,
    ) -> Result<SegmentationResult<T>, SegmentationError>;
}
