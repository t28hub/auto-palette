use crate::math::number::Float;
use std::cmp::Ordering;

/// Struct representing a neighbor point.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
#[derive(Debug)]
pub struct Neighbor<F: Float> {
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
    pub fn new(index: usize, distance: F) -> Self {
        Self { index, distance }
    }
}

impl<F> PartialEq<Self> for Neighbor<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<F> Eq for Neighbor<F> where F: Float {}

impl<F> PartialOrd<Self> for Neighbor<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<F> Ord for Neighbor<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_neighbor() {
        let neighbor = Neighbor::new(3, 2.0);
        assert_eq!(
            neighbor,
            Neighbor {
                index: 3,
                distance: 2.0,
            }
        );
    }
}
