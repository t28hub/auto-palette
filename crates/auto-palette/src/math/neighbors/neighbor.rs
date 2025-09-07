use std::cmp::Ordering;

use crate::math::FloatNumber;

/// A neighbor in a nearest neighbor search.
///
/// Represents a point found during a neighbor search, containing its index
/// in the original dataset and the distance from the query point.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances (e.g., `f32`, `f64`).
#[derive(Debug)]
pub struct Neighbor<T>
where
    T: FloatNumber,
{
    /// The index of the neighbor.
    index: usize,

    /// The distance to the neighbor.
    distance: T,
}

impl<T> Neighbor<T>
where
    T: FloatNumber,
{
    /// Creates a new `Neighbor` instance.
    ///
    /// # Arguments
    /// * `index` - The index of the neighbor.
    /// * `distance` - The distance to the neighbor. Must be non-negative.
    ///
    /// # Returns
    /// A `Neighbor` instance.
    ///
    /// # Panics
    /// Panics if the distance is negative (`distance < 0`).
    #[must_use]
    pub fn new(index: usize, distance: T) -> Self {
        debug_assert!(distance >= T::zero(), "Distance must be non-negative");
        Self { index, distance }
    }

    /// Returns the index of the neighbor.
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the distance to the neighbor.
    #[inline(always)]
    pub fn distance(&self) -> T {
        self.distance
    }
}

impl<T> PartialEq for Neighbor<T>
where
    T: FloatNumber,
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.distance == other.distance
    }
}

impl<T> Eq for Neighbor<T> where T: FloatNumber {}

impl<T> PartialOrd for Neighbor<T>
where
    T: FloatNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Neighbor<T>
where
    T: FloatNumber,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_neighbor() {
        // Act
        let neighbor = Neighbor::new(0, 2.0);

        // Assert
        assert_eq!(neighbor.index, 0);
        assert_eq!(neighbor.distance, 2.0);
    }

    #[test]
    fn test_neighbor_zero_distance() {
        // Act
        let neighbor = Neighbor::new(1, 0.0);

        // Assert
        assert_eq!(neighbor.index, 1);
        assert_eq!(neighbor.distance, 0.0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "Distance must be non-negative")]
    fn test_neighbor_panic() {
        let _ = Neighbor::new(0, -2.0);
    }

    #[test]
    fn test_eq_true() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(0, 2.0);

        // Act & Assert
        assert_eq!(neighbor1, neighbor2);
    }

    #[test]
    fn test_eq_false() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 2.0);

        // Act & Assert
        assert_ne!(neighbor1, neighbor2);
    }

    #[test]
    fn test_cmp_less() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 2.0);

        // Act
        let actual = neighbor1.cmp(&neighbor2);

        // Assert
        assert_eq!(actual, Ordering::Less);
    }

    #[test]
    fn test_cmp_equal() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 2.0);

        // Act
        let actual = neighbor1.cmp(&neighbor2);

        // Assert
        assert_eq!(actual, Ordering::Equal);
    }

    #[test]
    fn test_cmp_greater() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 1.0);

        // Act
        let actual = neighbor1.cmp(&neighbor2);

        // Act
        assert_eq!(actual, Ordering::Greater);
    }
}
