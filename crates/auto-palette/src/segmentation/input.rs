use crate::{image::Pixel, math::FloatNumber, segmentation::error::DimensionMismatchError};

/// A validated, read-only view of image pixel data with a mask.
///
/// This type bundles width, height, pixels, and mask into a single struct,
/// guaranteeing the invariant `width * height == pixels.len() == mask.len()`
/// at construction time.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct SegmentationInput<'a, T>
where
    T: FloatNumber,
{
    width: usize,
    height: usize,
    pixels: &'a [Pixel<T>],
    mask: &'a [bool],
}

impl<'a, T> SegmentationInput<'a, T>
where
    T: FloatNumber,
{
    /// Creates a new `SegmentationInput` after validating that dimensions match.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels of the image.
    /// * `mask` - The mask to apply to the pixels.
    ///
    /// # Returns
    /// A validated `SegmentationInput`, or a `DimensionMismatchError` if
    /// `width * height` does not equal the length of `pixels` or `mask`.
    pub fn new(
        width: usize,
        height: usize,
        pixels: &'a [Pixel<T>],
        mask: &'a [bool],
    ) -> Result<Self, DimensionMismatchError> {
        let expected = width * height;
        if pixels.len() != expected {
            return Err(DimensionMismatchError {
                width,
                height,
                expected,
                actual: pixels.len(),
            });
        }
        if mask.len() != expected {
            return Err(DimensionMismatchError {
                width,
                height,
                expected,
                actual: mask.len(),
            });
        }
        Ok(Self {
            width,
            height,
            pixels,
            mask,
        })
    }

    /// Returns the width of the image.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the image.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the pixels of the image.
    #[inline]
    #[must_use]
    pub fn pixels(&self) -> &[Pixel<T>] {
        self.pixels
    }

    /// Returns the mask of the image.
    #[inline]
    #[must_use]
    pub fn mask(&self) -> &[bool] {
        self.mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pixels: Vec<[f64; 5]> = vec![[0.0; 5]; 6];
        let mask = vec![true; 6];
        let input = SegmentationInput::new(3, 2, &pixels, &mask);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.width(), 3);
        assert_eq!(input.height(), 2);
        assert_eq!(input.pixels().len(), 6);
        assert_eq!(input.mask().len(), 6);
    }

    #[test]
    fn test_new_empty() {
        let pixels: Vec<[f64; 5]> = Vec::new();
        let mask: Vec<bool> = Vec::new();
        let input = SegmentationInput::new(0, 0, &pixels, &mask);
        assert!(input.is_ok());

        let input = input.unwrap();
        assert_eq!(input.width(), 0);
        assert_eq!(input.height(), 0);
    }

    #[test]
    fn test_new_pixel_length_mismatch() {
        let pixels: Vec<[f64; 5]> = vec![[0.0; 5]; 4];
        let mask = vec![true; 6];
        let result = SegmentationInput::new(3, 2, &pixels, &mask);
        assert_eq!(
            result,
            Err(DimensionMismatchError {
                width: 3,
                height: 2,
                expected: 6,
                actual: 4,
            })
        );
    }

    #[test]
    fn test_new_mask_length_mismatch() {
        let pixels: Vec<[f64; 5]> = vec![[0.0; 5]; 6];
        let mask = vec![true; 4];
        let result = SegmentationInput::new(3, 2, &pixels, &mask);
        assert_eq!(
            result,
            Err(DimensionMismatchError {
                width: 3,
                height: 2,
                expected: 6,
                actual: 4,
            })
        );
    }
}
