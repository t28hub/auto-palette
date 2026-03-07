use rustc_hash::FxHashMap;

use crate::{math::FloatNumber, segmentation::segment::SegmentMetadata};

/// The result of segmenting an image into regions.
///
/// # Type Parameters
/// * `T` - The floating point type used for pixel values.
#[derive(Debug, PartialEq)]
pub struct SegmentationResult<T>
where
    T: FloatNumber,
{
    /// The width of the segmented image.
    width: usize,

    /// The height of the segmented image.
    height: usize,

    /// The segments extracted from the image.
    segments: Vec<SegmentMetadata<T>>,
}

impl<T> SegmentationResult<T>
where
    T: FloatNumber,
{
    /// Creates a new `Builder` instance for constructing a `SegmentationResult`.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    ///
    /// # Returns
    /// A new `Builder` instance initialized with the specified dimensions.
    #[must_use]
    pub(super) fn builder(width: usize, height: usize) -> Builder<T> {
        Builder::new(width, height)
    }

    /// Returns the width of the segmented image.
    ///
    /// # Returns
    /// The width of the segmented image.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the segmented image.
    ///
    /// # Returns
    /// The height of the segmented image.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a slice of the segments in the result.
    ///
    /// # Returns
    /// A slice of the segments in the result.
    #[inline]
    #[must_use]
    pub fn segments(&self) -> &[SegmentMetadata<T>] {
        &self.segments
    }

    /// Returns whether the result contains no segments.
    ///
    /// # Returns
    /// `true` if the result contains no segments.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the number of segments in the result.
    ///
    /// # Returns
    /// The number of segments in the result.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }
}

/// A builder for constructing a `SegmentationResult`.
///
/// # Type Parameters
/// * `T` - The floating point type used for pixel values.
#[derive(Debug, PartialEq)]
pub(super) struct Builder<T>
where
    T: FloatNumber,
{
    /// The width of the image.
    width: usize,

    /// The height of the image.
    height: usize,

    /// The segments indexed by their labels.
    segments: FxHashMap<usize, SegmentMetadata<T>>,
}

impl<T> Builder<T>
where
    T: FloatNumber,
{
    /// Creates a new `Builder` instance for constructing a `SegmentationResult`.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    ///
    /// # Returns
    /// A new `Builder` instance initialized with the specified dimensions.
    #[must_use]
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            segments: FxHashMap::default(),
        }
    }

    /// Returns the segment metadata for a given label, if it exists.
    ///
    /// # Arguments
    /// * `label` - The label of the segment to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `SegmentMetadata` if it exists, or `None` if it does not.
    #[allow(dead_code)]
    #[inline]
    pub fn get(&self, label: &usize) -> Option<&SegmentMetadata<T>> {
        self.segments.get(label)
    }

    /// Returns a mutable reference to the segment metadata for a given label, creating it if it does not exist.
    ///
    /// # Arguments
    /// * `label` - The label of the segment to retrieve or create.
    ///
    /// # Returns
    /// A mutable reference to the `SegmentMetadata` associated with the label.
    pub fn get_mut(&mut self, label: &usize) -> &mut SegmentMetadata<T> {
        self.segments
            .entry(*label)
            .or_insert_with(|| SegmentMetadata::new(*label))
    }

    /// Returns an iterator over the segments in the builder.
    ///
    /// # Returns
    /// An iterator over the segments in the builder.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &SegmentMetadata<T>> {
        self.segments.values()
    }

    /// Returns a mutable iterator over the segments in the builder.
    ///
    /// # Returns
    /// A mutable iterator over the segments in the builder.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SegmentMetadata<T>> {
        self.segments.values_mut()
    }

    /// Merges two segments in the builder.
    ///
    /// # Arguments
    /// * `src_label` - The label of the source segment to merge.
    /// * `dst_label` - The label of the destination segment to merge into.
    ///
    /// # Returns
    /// `true` if the merge was successful, `false` if the segments were the same or one of them did not exist.
    pub fn merge(&mut self, src_label: &usize, dst_label: &usize) -> bool {
        if src_label == dst_label {
            // No need to merge the same segment.
            return false;
        }

        let (mut src_segment, mut dst_segment) = match (
            self.segments.remove(src_label),
            self.segments.remove(dst_label),
        ) {
            (Some(src), Some(dst)) => (src, dst),
            (Some(src), None) => {
                // If the destination segment does not exist, just reinsert the source segment.
                self.segments.insert(*src_label, src);
                return false;
            }
            (None, Some(dst)) => {
                // If the source segment does not exist, just reinsert the destination segment.
                self.segments.insert(*dst_label, dst);
                return false;
            }
            _ => return false,
        };

        // Absorb the source segment into the destination segment.
        dst_segment.absorb(&mut src_segment);

        // Insert the updated destination segment back into the map.
        self.segments.insert(*dst_label, dst_segment);
        true
    }

    /// Removes a segment corresponding to the given label from the builder.
    ///
    /// # Arguments
    /// * `label` - The label of the segment to remove.
    ///
    /// # Returns
    /// An `Option` containing the removed `SegmentMetadata` if it existed, or `None` if it did not.
    pub fn remove(&mut self, label: &usize) -> Option<SegmentMetadata<T>> {
        self.segments.remove(label)
    }

    /// Builds the `SegmentationResult` from the current state of the builder.
    ///
    /// Empty segments are excluded from the result.
    ///
    /// # Returns
    /// A new `SegmentationResult` instance.
    #[must_use]
    pub fn build(self) -> SegmentationResult<T> {
        let segments = self
            .segments
            .into_values()
            .filter(|s| !s.is_empty())
            .collect();
        SegmentationResult {
            width: self.width,
            height: self.height,
            segments,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rustc_hash::FxHashSet;

    use super::*;
    use crate::image::LABXY_CHANNELS;

    #[test]
    fn test_builder() {
        // Act
        let builder = SegmentationResult::<f64>::builder(480, 320);

        // Assert
        assert_eq!(
            builder,
            Builder {
                width: 480,
                height: 320,
                segments: FxHashMap::default(),
            }
        );
    }

    #[test]
    fn test_builder_get() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label = 0usize;
        builder.get_mut(&label);

        // Act
        let actual = builder.get(&label);

        // Assert
        assert!(actual.is_some());

        let segment = actual.unwrap();
        assert!(segment.is_empty());
        assert_eq!(segment.len(), 0);
        assert_eq!(segment.label, label);
        assert_eq!(segment.center, [0.0; LABXY_CHANNELS]);
    }

    #[test]
    fn test_builder_get_non_existent() {
        // Arrange
        let builder = SegmentationResult::<f64>::builder(480, 320);
        let label = 1usize;

        // Act
        let actual = builder.get(&label);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_builder_iter() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label1 = 1usize;
        builder.get_mut(&label1);

        let label2 = 2usize;
        builder.get_mut(&label2);

        // Act
        let actual: Vec<_> = builder.iter().collect();

        // Assert
        assert_eq!(actual.len(), 2);
        assert!(actual.iter().any(|s| s.label == label1));
        assert!(actual.iter().any(|s| s.label == label2));
    }

    #[test]
    fn test_builder_iter_mut() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label1 = 1usize;
        builder.get_mut(&label1).insert(0, &[1.0; LABXY_CHANNELS]);

        let label2 = 2usize;
        builder.get_mut(&label2).insert(1, &[2.0; LABXY_CHANNELS]);

        // Act
        for segment in builder.iter_mut() {
            segment.clear();
        }

        // Assert
        assert_eq!(builder.segments.len(), 2);

        let segment1 = builder.get(&label1).unwrap();
        assert!(segment1.is_empty());
        assert_eq!(segment1.len(), 0);
        assert_eq!(segment1.label, label1);
        assert_eq!(segment1.center, [0.0; LABXY_CHANNELS]);

        let segment2 = builder.get(&label2).unwrap();
        assert!(segment2.is_empty());
        assert_eq!(segment2.len(), 0);
        assert_eq!(segment2.label, label2);
        assert_eq!(segment2.center, [0.0; LABXY_CHANNELS]);
    }

    #[test]
    fn test_builder_merge() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let src_label = 1usize;
        let src = builder.get_mut(&src_label);
        src.insert(0, &[1.0; LABXY_CHANNELS]);
        src.insert(1, &[2.0; LABXY_CHANNELS]);

        let dst_label = 2usize;
        let dst = builder.get_mut(&dst_label);
        dst.insert(2, &[3.0; LABXY_CHANNELS]);
        dst.insert(3, &[4.0; LABXY_CHANNELS]);
        dst.insert(4, &[5.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.merge(&src_label, &dst_label);

        // Assert
        assert!(actual);

        let src = builder.get(&src_label);
        assert!(src.is_none());

        let dst = builder.get(&dst_label);
        assert!(dst.is_some());

        let dst_segment = dst.unwrap();
        assert_eq!(dst_segment.len(), 5);
        assert_eq!(dst_segment.label, dst_label);
        assert_eq!(dst_segment.center, [3.0; LABXY_CHANNELS]);

        let dst_indices: HashSet<_> = dst_segment.members().cloned().collect();
        assert_eq!(dst_indices, HashSet::from([0, 1, 2, 3, 4]));
    }

    #[test]
    fn test_builder_merge_same_label() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label = 1usize;
        builder.get_mut(&label).insert(0, &[1.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.merge(&label, &label);

        // Assert
        assert!(!actual);
        assert!(builder.get(&label).is_some());
    }

    #[test]
    fn test_builder_merge_non_existent() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let src_label = 1usize;
        let dst_label = 2usize;

        // Act
        let actual = builder.merge(&src_label, &dst_label);

        // Assert
        assert!(!actual);
        assert!(builder.get(&src_label).is_none());
        assert!(builder.get(&dst_label).is_none());
    }

    #[test]
    fn test_builder_merge_src_non_existent() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let src_label = 1usize;

        let dst_label = 2usize;
        builder
            .get_mut(&dst_label)
            .insert(0, &[1.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.merge(&src_label, &dst_label);

        // Assert
        assert!(!actual);
        assert!(builder.get(&src_label).is_none());
        assert!(builder.get(&dst_label).is_some());
    }

    #[test]
    fn test_builder_merge_dst_non_existent() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let src_label = 1usize;
        builder
            .get_mut(&src_label)
            .insert(0, &[1.0; LABXY_CHANNELS]);

        let dst_label = 2usize;

        // Act
        let actual = builder.merge(&src_label, &dst_label);

        // Assert
        assert!(!actual);
        assert!(builder.get(&src_label).is_some());
        assert!(builder.get(&dst_label).is_none());
    }

    #[test]
    fn test_builder_remove() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label = 1usize;
        builder.get_mut(&label).insert(0, &[1.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.remove(&label);

        // Assert
        assert!(actual.is_some());
        assert!(builder.get(&label).is_none());

        let segment = actual.unwrap();
        assert_eq!(
            segment,
            SegmentMetadata {
                label,
                center: [1.0; LABXY_CHANNELS],
                indices: FxHashSet::from_iter([0]),
            }
        );
    }

    #[test]
    fn test_builder_remove_non_existent() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label = 1usize;

        // Act
        let actual = builder.remove(&label);

        // Assert
        assert!(actual.is_none());
        assert!(builder.get(&label).is_none());
    }

    #[test]
    fn test_builder_build() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label1 = 0usize;
        let segment1 = builder.get_mut(&label1);
        segment1.insert(0, &[1.0; LABXY_CHANNELS]);
        segment1.insert(1, &[2.0; LABXY_CHANNELS]);

        let label2 = 1usize;
        let segment2 = builder.get_mut(&label2);
        segment2.insert(2, &[3.0; LABXY_CHANNELS]);
        segment2.insert(4, &[4.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.build();

        // Assert
        assert_eq!(actual.width(), 480);
        assert_eq!(actual.height(), 320);
        assert_eq!(actual.len(), 2);
        assert!(actual
            .segments()
            .iter()
            .any(|s| s.label() == label1 && s.len() == 2));
        assert!(actual
            .segments()
            .iter()
            .any(|s| s.label() == label2 && s.len() == 2));
    }

    #[test]
    fn test_builder_build_filters_empty() {
        // Arrange
        let mut builder = SegmentationResult::<f64>::builder(480, 320);

        let label1 = 0usize;
        builder.get_mut(&label1); // Create empty segment

        let label2 = 1usize;
        let segment2 = builder.get_mut(&label2);
        segment2.insert(0, &[1.0; LABXY_CHANNELS]);

        // Act
        let actual = builder.build();

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual.segments()[0].label(), label2);
    }
}
