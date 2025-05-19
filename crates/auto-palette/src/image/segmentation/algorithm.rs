use crate::{
    image::{segmentation::segment::Segment, Pixel},
    FloatNumber,
};

/// A collection of segments.
pub type Segments<T> = Vec<Segment<T>>;

/// Trait for segmentation algorithms.
///
/// Implementations receive image `pixels` and split them into a requested
/// number of coherent segments.
///
/// # Type Parameters
/// * `T` - The floating point type.
pub trait Segmentation<T>
where
    T: FloatNumber,
{
    /// Error type for the segmentation algorithm.
    type Err;

    /// Splits the given image into segments.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels of the image.
    ///
    /// # Returns
    /// A collection of segments.
    fn segment(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
    ) -> Result<Segments<T>, Self::Err> {
        let mask = vec![true; width * height];
        self.segment_with_mask(width, height, pixels, &mask)
    }

    /// Splits the given image into segments with a mask.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels of the image.
    /// * `mask` - The mask to apply to the pixels.
    ///
    /// # Returns
    /// A collection of segments.
    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<Segments<T>, Self::Err>;
}
