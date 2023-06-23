use crate::number::Float;
use std::cmp::Ordering;

/// Struct representing a neighbor point.
///
/// # Type parameters
/// * `F` - The float type used for the distance.
#[derive(Debug)]
pub struct Neighbor<F>
where
    F: Float,
{
    /// The index of the neighbor.
    pub index: usize,
    /// The distance between the query point and the neighbor.
    pub distance: F,
}

impl<F> Neighbor<F>
where
    F: Float,
{
    /// Creates a new `Neighbor` instance.
    ///
    /// # Arguments
    /// * `index` - The index of the neighbor.
    /// * `distance` - The distance between the query point and the neighbor.
    ///
    /// # Returns
    /// A new `Neighbor` instance.
    #[must_use]
    pub fn new(index: usize, distance: F) -> Self {
        Self { index, distance }
    }
}

impl<F> PartialEq<Self> for Neighbor<F>
where
    F: Float,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.distance == other.distance
    }
}

impl<F> Eq for Neighbor<F> where F: Float {}

impl<F> PartialOrd<Self> for Neighbor<F>
where
    F: Float,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<F> Ord for Neighbor<F>
where
    F: Float,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Float;

    #[test]
    fn test_neighbor() {
        let neighbor = Neighbor::new(1, 2.0);
        assert_eq!(1, neighbor.index);
        assert_eq!(2.0, neighbor.distance);
    }

    #[test]
    fn test_eq() {
        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(0, 1.0);
        assert_eq!(neighbor1, neighbor2);

        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_ne!(neighbor1, neighbor2);
    }

    #[test]
    fn test_partial_cmp() {
        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 2.0);
        assert_eq!(neighbor1.partial_cmp(&neighbor2), Some(Ordering::Less));

        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.partial_cmp(&neighbor2), Some(Ordering::Equal));

        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.partial_cmp(&neighbor2), Some(Ordering::Greater));

        let neighbor1 = Neighbor::new(0, f64::nan());
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.partial_cmp(&neighbor2), None);
    }

    #[test]
    fn test_cmp() {
        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 2.0);
        assert_eq!(neighbor1.cmp(&neighbor2), Ordering::Less);

        let neighbor1 = Neighbor::new(0, 1.0);
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.cmp(&neighbor2), Ordering::Equal);

        let neighbor1 = Neighbor::new(0, 2.0);
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.cmp(&neighbor2), Ordering::Greater);

        let neighbor1 = Neighbor::new(0, f64::nan());
        let neighbor2 = Neighbor::new(1, 1.0);
        assert_eq!(neighbor1.cmp(&neighbor2), Ordering::Equal);
    }
}
