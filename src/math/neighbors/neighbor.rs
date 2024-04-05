use std::cmp::Ordering;

/// A neighbor in a nearest neighbor search.
#[derive(Debug)]
pub struct Neighbor {
    /// The index of the neighbor.
    pub(crate) index: usize,
    /// The distance to the neighbor.
    pub(crate) distance: f32,
}

impl Neighbor {
    /// Creates a new `Neighbor` instance.
    ///
    /// # Arguments
    /// * `index` - The index of the neighbor.
    /// * `distance` - The distance to the neighbor.
    ///
    /// # Returns
    /// A `Neighbor` instance.
    #[must_use]
    pub fn new(index: usize, distance: f32) -> Self {
        Self { index, distance }
    }
}

impl PartialEq for Neighbor {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.distance == other.distance
    }
}

impl Eq for Neighbor {}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
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
    fn test_eq_true() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(0, 2.0);

        // Act
        let equality = neighbor1.eq(&neighbor2);

        // Assert
        assert!(equality);
    }

    #[test]
    fn test_eq_false() {
        // Arrange
        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 2.0);

        // Act
        let equality = neighbor1.eq(&neighbor2);

        // Assert
        assert!(!equality);
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
