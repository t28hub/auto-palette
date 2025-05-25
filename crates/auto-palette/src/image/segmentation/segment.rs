use std::collections::HashSet;

use crate::{
    image::{Pixel, LABXY_CHANNELS},
    math::clustering::Cluster,
    FloatNumber,
};

/// Represents a segment in an image.
///
/// This struct contains a center pixel and a set of pixel assignments.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment<T>
where
    T: FloatNumber,
{
    center: Pixel<T>,
    assignments: HashSet<usize>,
}

impl<T> Segment<T>
where
    T: FloatNumber,
{
    /// Returns the center pixel of this segment.
    ///
    /// # Returns
    /// The center pixel of this segment.
    #[must_use]
    pub fn center(&self) -> &Pixel<T> {
        &self.center
    }

    /// Checks whether this segment is empty.
    ///
    /// # Returns
    /// `true` if this segment is empty; `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    /// Returns the number of pixel assignments in this segment.
    ///
    /// # Returns
    /// The number of pixel assignments in this segment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    /// Returns the iterator over the pixel assignments of this segment.
    ///
    /// # Returns
    /// An iterator over the pixel assignments of this segment.
    #[allow(dead_code)]
    pub fn assignments(&self) -> impl Iterator<Item = &usize> {
        self.assignments.iter()
    }

    /// Assigns a pixel with the given index to this segment.
    ///
    /// # Arguments
    /// * `index` - The index of the pixel to assign.
    /// * `pixel` - The pixel to assign.
    ///
    /// # Returns
    /// `true` if the pixel was assigned; `false` if the pixel was already assigned.
    #[inline]
    pub(super) fn assign(&mut self, index: usize, pixel: &Pixel<T>) -> bool {
        if !self.assignments.insert(index) {
            return false;
        }

        let size = T::from_usize(self.assignments.len());
        self.center.iter_mut().zip(pixel).for_each(|(c, p)| {
            *c = (*c * (size - T::one()) + *p) / size;
        });
        true
    }

    /// Absorbs another segment into this one.
    ///
    /// # Arguments
    /// * `other` - The segment to absorb.
    pub(super) fn absorb(&mut self, other: &Segment<T>) {
        let self_weight = T::from_usize(self.assignments.len());
        let other_weight = T::from_usize(other.assignments.len());
        let total_weight = self_weight + other_weight;
        for (self_component, other_component) in self.center.iter_mut().zip(other.center()) {
            *self_component =
                (*self_component * self_weight + *other_component * other_weight) / total_weight;
        }
        self.assignments.extend(other.assignments());
    }

    /// Resets the segment to its initial state.
    #[inline]
    pub(super) fn reset(&mut self) {
        self.center.fill(T::zero());
        self.assignments.clear();
    }
}

impl<T> Default for Segment<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            center: [T::zero(); LABXY_CHANNELS],
            assignments: HashSet::new(),
        }
    }
}

impl<T> From<&Cluster<T, LABXY_CHANNELS>> for Segment<T>
where
    T: FloatNumber,
{
    fn from(cluster: &Cluster<T, LABXY_CHANNELS>) -> Self {
        Self {
            center: *cluster.centroid(),
            assignments: HashSet::from_iter(cluster.members().copied()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_assign() {
        // Arrange
        let mut segment = Segment::<f64>::default();

        // Act
        let actual = segment.assign(0, &[0.25; LABXY_CHANNELS]);

        // Assert
        assert!(actual);
        assert!(!segment.is_empty());
        assert_eq!(segment.len(), 1);
        assert_eq!(segment.center(), &[0.25; LABXY_CHANNELS]);
        assert_eq!(segment.assignments().copied().collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn test_assign_existing() {
        // Arrange
        let mut segment = Segment::<f64>::default();
        segment.assign(0, &[0.25; LABXY_CHANNELS]);

        // Act
        let actual = segment.assign(0, &[0.5; LABXY_CHANNELS]);

        // Assert
        assert!(!actual);
        assert!(!segment.is_empty());
        assert_eq!(segment.len(), 1);
        assert_eq!(segment.center(), &[0.25; LABXY_CHANNELS]);
        assert_eq!(segment.assignments().copied().collect::<Vec<_>>(), vec![0]);
    }

    #[test]
    fn test_absorb() {
        // Arrange
        let mut segment1 = Segment::<f64>::default();
        segment1.assign(0, &[0.25; LABXY_CHANNELS]);
        segment1.assign(1, &[0.75; LABXY_CHANNELS]);

        let mut segment2 = Segment::<f64>::default();
        segment2.assign(2, &[0.5; LABXY_CHANNELS]);
        segment2.assign(3, &[1.0; LABXY_CHANNELS]);
        segment2.assign(4, &[1.5; LABXY_CHANNELS]);

        // Act
        segment1.absorb(&segment2);

        // Assert
        assert!(!segment1.is_empty());
        assert_eq!(segment1.len(), 5);
        assert_eq!(segment1.center(), &[0.8; LABXY_CHANNELS]);
        assert_eq!(
            segment1.assignments().copied().collect::<HashSet<_>>(),
            HashSet::from([0, 1, 2, 3, 4])
        );

        assert!(!segment2.is_empty());
        assert_eq!(segment2.len(), 3);
        assert_eq!(segment2.center(), &[1.0; LABXY_CHANNELS]);
        assert_eq!(
            segment2.assignments().copied().collect::<HashSet<_>>(),
            HashSet::from([2, 3, 4])
        );
    }

    #[test]
    fn test_reset() {
        // Arrange
        let mut segment = Segment::<f64>::default();
        segment.assign(0, &[0.25; LABXY_CHANNELS]);

        // Act
        segment.reset();

        // Assert
        assert!(segment.is_empty());
        assert_eq!(segment.len(), 0);
        assert_eq!(segment.center(), &[0.0; LABXY_CHANNELS]);
        assert_eq!(
            segment.assignments().collect::<HashSet<_>>(),
            HashSet::new()
        );
    }

    #[test]
    fn test_default() {
        // Act
        let actual = Segment::<f64>::default();

        // Assert
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
        assert_eq!(
            actual,
            Segment {
                center: [0.0; LABXY_CHANNELS],
                assignments: HashSet::new(),
            }
        )
    }

    #[test]
    fn test_from_cluster() {
        // Arrange
        let mut cluster = Cluster::new();
        cluster.add_member(0, &[0.2; LABXY_CHANNELS]);
        cluster.add_member(1, &[0.3; LABXY_CHANNELS]);
        cluster.add_member(2, &[0.5; LABXY_CHANNELS]);

        let segment = Segment::from(&cluster);

        // Act & Assert
        assert!(!segment.is_empty());
        assert_eq!(segment.len(), 3);

        let center = segment.center();
        assert_approx_eq!(center[0], 0.333333);
        assert_approx_eq!(center[1], 0.333333);
        assert_approx_eq!(center[2], 0.333333);
        assert_approx_eq!(center[3], 0.333333);
        assert_approx_eq!(center[4], 0.333333);

        let assignments = HashSet::from_iter(segment.assignments().copied());
        assert_eq!(assignments, HashSet::from([0, 1, 2]));
    }
}
