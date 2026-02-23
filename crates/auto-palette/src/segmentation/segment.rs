use rustc_hash::FxHashSet;

use crate::{
    image::{Pixel, LABXY_CHANNELS},
    math::FloatNumber,
};

/// Metadata for a segment in a labeled image.
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

    /// The center pixel of this segment, represented as a normalized LABXY pixel.
    pub(super) center: Pixel<T>,

    /// The indices of the pixels that belong to this segment.
    pub(super) indices: FxHashSet<usize>,
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
            center: [T::zero(); LABXY_CHANNELS],
            indices: FxHashSet::default(),
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
    /// The center pixel of this segment.
    #[inline]
    #[must_use]
    pub fn center(&self) -> &Pixel<T> {
        &self.center
    }

    /// Checks whether this segment is empty.
    ///
    /// # Returns
    /// `true` if this segment has no indices; `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Returns the number of pixel indices in this segment.
    ///
    /// # Returns
    /// The number of pixel indices in this segment.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Returns the indices of the pixels that belong to this segment.
    ///
    /// # Returns
    /// A slice containing the indices of the pixels in this segment.
    pub fn members(&self) -> impl Iterator<Item = &usize> {
        self.indices.iter()
    }

    /// Inserts a pixel into this segment.
    ///
    /// # Arguments
    /// * `index` - The index of the pixel to insert.
    /// * `pixel` - A reference to the pixel to insert.
    ///
    /// # Returns
    /// `true` if the pixel was successfully inserted, `false` if it was already assigned to this segment.
    #[inline(always)]
    pub(super) fn insert(&mut self, index: usize, pixel: &Pixel<T>) -> bool {
        if !self.indices.insert(index) {
            // The pixel is already assigned to this segment.
            return false;
        }

        let count = T::from_usize(self.indices.len());
        self.center.iter_mut().zip(pixel).for_each(|(c, p)| {
            *c = (*c * (count - T::one()) + *p) / count;
        });
        true
    }

    /// Absorbs the metadata from another segment into this one.
    ///
    /// # Arguments
    /// * `other` - The segment metadata to absorb.
    ///
    /// # Note
    /// This method resets the given segment state, merging its center and indices into this segment.
    pub(super) fn absorb(&mut self, other: &mut Self) {
        if other.is_empty() {
            return;
        }

        let self_count = T::from_usize(self.indices.len());
        let other_count = T::from_usize(other.indices.len());
        let total_count = self_count + other_count;
        self.center
            .iter_mut()
            .zip(other.center())
            .for_each(|(c, o)| {
                *c = (*c * self_count + *o * other_count) / total_count;
            });
        self.indices.extend(other.members());
    }

    /// Clears the segment metadata, resetting its state.
    ///
    /// # Note
    /// This method clears the indices, resets the total pixel values, and marks the segment as dirty.
    pub(super) fn clear(&mut self) {
        self.center.fill(T::zero());
        self.indices.clear();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
                center: [0.0; LABXY_CHANNELS],
                indices: FxHashSet::default(),
            }
        );
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
        assert_eq!(actual.label(), 0);
        assert_eq!(actual.center(), &[0.0; LABXY_CHANNELS]);
    }

    #[test]
    fn test_members() {
        // Arrange
        let mut segment = SegmentMetadata::new(1);
        segment.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment.insert(1, &[0.3, 0.4, 0.5, 0.6, 0.7]);

        // Act
        let members: HashSet<_> = segment.members().cloned().collect();

        // Assert
        assert_eq!(members, HashSet::from([0, 1]));
    }

    #[test]
    fn test_members_empty() {
        // Arrange
        let segment = SegmentMetadata::<f64>::new(1);

        // Act
        let members: Vec<_> = segment.members().cloned().collect();

        // Assert
        assert!(members.is_empty());
    }

    #[test]
    fn test_insert() {
        // Act
        let mut segment = SegmentMetadata::new(1);
        let actual = segment.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);

        // Assert
        assert!(actual);
        assert_eq!(segment.len(), 1);
        assert_eq!(segment.center(), &[0.2, 0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_insert_duplicate() {
        // Arrange
        let mut segment = SegmentMetadata::new(1);
        segment.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);

        // Act
        let actual = segment.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);

        // Assert
        assert!(!actual);
        assert_eq!(segment.len(), 1);
        assert_eq!(segment.center(), &[0.2, 0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_absorb() {
        // Arrange
        let mut segment1 = SegmentMetadata::new(1);
        segment1.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment1.insert(2, &[0.3, 0.4, 0.5, 0.6, 0.7]);
        segment1.insert(3, &[0.5, 0.6, 0.7, 0.8, 0.9]);

        let mut segment2 = SegmentMetadata::new(2);
        segment2.insert(1, &[0.4, 0.5, 0.6, 0.7, 0.8]);

        // Act
        segment1.absorb(&mut segment2);

        // Assert
        assert_eq!(segment1.len(), 4);

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
        segment1.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);

        let mut segment2 = SegmentMetadata::new(2); // Empty segment

        // Act
        segment1.absorb(&mut segment2);

        // Assert
        assert_eq!(segment1.len(), 1);
        assert_eq!(segment1.center(), &[0.2, 0.3, 0.4, 0.5, 0.6]);
    }

    #[test]
    fn test_clear() {
        // Arrange
        let mut segment = SegmentMetadata::new(1);
        segment.insert(0, &[0.2, 0.3, 0.4, 0.5, 0.6]);
        segment.insert(1, &[0.3, 0.4, 0.5, 0.6, 0.7]);

        // Act
        segment.clear();

        // Assert
        assert!(segment.is_empty());
        assert_eq!(segment.len(), 0);
        assert_eq!(segment.center(), &[0.0; LABXY_CHANNELS]);
    }
}
