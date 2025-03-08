use std::cmp::Ordering;

use crate::math::FloatNumber;

/// A neighbor in a nearest neighbor search.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug)]
pub struct Neighbor<T>
where
    T: FloatNumber,
{
    /// The index of the neighbor.
    pub(crate) index: usize,
    /// The distance to the neighbor.
    pub(crate) distance: T,
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
        let ordering = neighbor1.cmp(&neighbor2);

        // Act & Assert
        assert_eq!(ordering, Ordering::Less);
    }

    #[test]
    fn test_cmp_equal() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 2.0);

        // Act
        let ordering = neighbor1.cmp(&neighbor2);

        // Act & Assert
        assert_eq!(ordering, Ordering::Equal);
    }

    #[test]
    fn test_cmp_greater() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 1.0);

        // Act
        let ordering = neighbor1.cmp(&neighbor2);

        // Act & Assert
        assert_eq!(ordering, Ordering::Greater);
    }
}
