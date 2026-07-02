use crate::{
    image::{Pixel, LABXY_CHANNELS},
    math::FloatNumber,
};

/// Metadata for a segment in a labeled image.
///
/// The segment tracks the number of assigned pixels and the component-wise
/// sum of their values; the center is derived as `sum / count`. Compared to
/// a running mean this avoids divisions on every insertion and accumulates
/// less floating point error.
///
/// # Type Parameters
/// * `T` - The floating point type used for pixel values.
#[derive(Debug, PartialEq)]
pub struct SegmentMetadata<T>
where
    T: FloatNumber,
{
    /// The label of this segment.
    pub(super) label: usize,

    /// The component-wise sum of the pixels assigned to this segment.
    pub(super) sum: Pixel<T>,

    /// The number of pixels assigned to this segment.
    pub(super) count: usize,
}

impl<T> SegmentMetadata<T>
where
    T: FloatNumber,
{
    /// Creates a new `SegmentMetadata` instance with the given label.
    ///
    /// # Arguments
    /// * `label` - The label of this segment.
    ///
    /// # Returns
    /// A new `SegmentMetadata` instance with the specified label.
    #[must_use]
    pub fn new(label: usize) -> Self {
        Self {
            label,
            sum: [T::zero(); LABXY_CHANNELS],
            count: 0,
        }
    }

    /// Returns the label of this segment.
    ///
    /// # Returns
    /// The label of this segment.
    #[inline]
    #[must_use]
    pub fn label(&self) -> usize {
        self.label
    }

    /// Returns the center pixel of this segment.
    ///
    /// # Returns
    /// The center pixel of this segment, or all zeros if the segment is empty.
    #[inline]
    #[must_use]
    pub fn center(&self) -> Pixel<T> {
        if self.count == 0 {
            return [T::zero(); LABXY_CHANNELS];
        }

        let count = T::from_usize(self.count);
        let mut center = self.sum;
        center.iter_mut().for_each(|c| *c = *c / count);
        center
    }

    /// Checks whether this segment is empty.
    ///
    /// # Returns
    /// `true` if this segment has no pixels; `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the number of pixels in this segment.
    ///
    /// # Returns
    /// The number of pixels in this segment.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Inserts a pixel into this segment.
    ///
    /// # Arguments
    /// * `pixel` - A reference to the pixel to insert.
    #[inline(always)]
    pub(super) fn insert(&mut self, pixel: &Pixel<T>) {
        self.count += 1;
        self.sum.iter_mut().zip(pixel).for_each(|(s, p)| {
            *s = *s + *p;
        });
    }

    /// Absorbs the metadata from another segment into this one.
    ///
    /// # Arguments
    /// * `other` - The segment metadata to absorb.
    ///
    /// # Note
    /// This method resets the given segment state, merging its sum and count into this segment.
    pub(super) fn absorb(&mut self, other: &mut Self) {
        if other.is_empty() {
            return;
        }

        self.count += other.count;
        self.sum.iter_mut().zip(&other.sum).for_each(|(s, o)| {
            *s = *s + *o;
        });
        other.clear();
    }

    /// Clears the segment metadata, resetting its state.
    pub(super) fn clear(&mut self) {
        self.sum.fill(T::zero());
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual = SegmentMetadata::<f64>::new(0);

        // Assert
        assert_eq!(
            actual,
            SegmentMetadata {
                label: 0,
                sum: [0.0; LABXY_CHANNELS],
                count: 0,
            }
        );
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
        assert_eq!(actual.label(), 0);
        assert_eq!(actual.center(), [0.0; LABXY_CHANNELS]);
    }

    #[test]
    fn test_insert() {
        // Act
        let mut segment = SegmentMetadata::new(1);
        segment.insert(&[0.2, 0.3, 0.4, 0.5, 0.6]);

        // Assert
        assert_eq!(segment.len(), 1);
        assert_eq!(segment.center(), [0.2, 0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_insert_multiple() {
        // Act
        let mut segment = SegmentMetadata::new(1);
        segment.insert(&[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment.insert(&[0.4, 0.5, 0.6, 0.7, 0.8]);

        // Assert
        assert_eq!(segment.len(), 2);

        let center = segment.center();
        assert_approx_eq!(center[0], 0.3);
        assert_approx_eq!(center[1], 0.4);
        assert_approx_eq!(center[2], 0.5);
        assert_approx_eq!(center[3], 0.6);
        assert_approx_eq!(center[4], 0.7);
    }

    #[test]
    fn test_absorb() {
        // Arrange
        let mut segment1 = SegmentMetadata::new(1);
        segment1.insert(&[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment1.insert(&[0.3, 0.4, 0.5, 0.6, 0.7]);
        segment1.insert(&[0.5, 0.6, 0.7, 0.8, 0.9]);

        let mut segment2 = SegmentMetadata::new(2);
        segment2.insert(&[0.4, 0.5, 0.6, 0.7, 0.8]);

        // Act
        segment1.absorb(&mut segment2);

        // Assert
        assert_eq!(segment1.len(), 4);
        assert!(segment2.is_empty());

        let center = segment1.center();
        assert_approx_eq!(center[0], 0.35);
        assert_approx_eq!(center[1], 0.45);
        assert_approx_eq!(center[2], 0.55);
        assert_approx_eq!(center[3], 0.65);
        assert_approx_eq!(center[4], 0.75);
    }

    #[test]
    fn test_absorb_empty_segment() {
        // Arrange
        let mut segment1 = SegmentMetadata::new(1);
        segment1.insert(&[0.2, 0.3, 0.4, 0.5, 0.6]);

        let mut segment2 = SegmentMetadata::<f64>::new(2); // Empty segment

        // Act
        segment1.absorb(&mut segment2);

        // Assert
        assert_eq!(segment1.len(), 1);
        assert_eq!(segment1.center(), [0.2, 0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_clear() {
        // Arrange
        let mut segment = SegmentMetadata::new(1);
        segment.insert(&[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment.insert(&[0.3, 0.4, 0.5, 0.6, 0.7]);

        // Act
        segment.clear();

        // Assert
        assert!(segment.is_empty());
        assert_eq!(segment.len(), 0);
        assert_eq!(segment.center(), [0.0; LABXY_CHANNELS]);
    }
}
