use crate::math::number::Float;
use crate::math::point::Point;
use std::marker::PhantomData;

/// Struct representing a cluster.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
/// * `P` - The type of points used in the clustering algorithm.
#[derive(Debug, Clone, PartialEq)]
pub struct Cluster<F, P>
where
    F: Float,
    P: Point<F>,
{
    pub(crate) centroid: P,
    pub(crate) membership: Vec<usize>,
    _marker: PhantomData<F>,
}

impl<F, P> Cluster<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `Cluster` instance with the given label.
    ///
    /// # Arguments
    /// * `initial_centroid` - The initial centroid of the new cluster.
    ///
    /// # Returns
    /// A new `Cluster` instance.
    #[must_use]
    pub fn new(initial_centroid: P) -> Self {
        Self {
            centroid: initial_centroid,
            membership: Vec::new(),
            _marker: PhantomData::default(),
        }
    }

    /// Returns a reference to the centroid of this cluster.
    ///
    /// # Returns
    /// A reference to the centroid of this cluster.
    #[must_use]
    pub fn centroid(&self) -> &P {
        &self.centroid
    }

    /// Checks whether this cluster is empty.
    ///
    /// # Returns
    /// `true` if this cluster is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.membership.is_empty()
    }

    /// Returns the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    #[must_use]
    pub fn size(&self) -> usize {
        self.membership.len()
    }

    /// Returns a reference to the membership of this cluster.
    ///
    /// # Returns
    /// A reference to the membership of this cluster.
    #[must_use]
    pub fn membership(&self) -> &[usize] {
        &self.membership
    }

    /// Inserts a point with index.
    ///
    /// # Arguments
    /// * `index` - The index of the point to insert.
    /// * `point` - The reference of the point to insert.
    pub fn insert(&mut self, index: usize, point: &P) {
        self.centroid += *point;
        self.membership.push(index);
    }

    /// Clears the centroid and all the membership.
    pub fn clear(&mut self) {
        self.centroid.set_zero();
        self.membership.clear();
    }
}

impl<F, P> Default for Cluster<F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn default() -> Self {
        Self {
            centroid: P::zero(),
            membership: Vec::new(),
            _marker: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;
    use num_traits::Zero;

    #[test]
    fn test_cluster() {
        let actual = Cluster::new(Point2::new(1.0, 2.0));
        assert!(actual.is_empty());
        assert_eq!(actual.centroid(), &Point2::new(1.0, 2.0));
        assert_eq!(actual.size(), 0);
        assert_eq!(actual.membership(), &[]);
    }

    #[test]
    fn test_insert() {
        let mut cluster = Cluster::new(Point2::new(1.0, 2.0));
        cluster.insert(3, &Point2::new(2.0, 3.0));
        assert!(!cluster.is_empty());
        assert_eq!(cluster.centroid(), &Point2::new(3.0, 5.0));
        assert_eq!(cluster.size(), 1);
        assert_eq!(cluster.membership(), &[3]);

        cluster.insert(5, &Point2::new(3.0, 5.0));
        assert_eq!(cluster.centroid(), &Point2::new(6.0, 10.0));
        assert!(!cluster.is_empty());
        assert_eq!(cluster.size(), 2);
        assert_eq!(cluster.membership(), &[3, 5]);
    }

    #[test]
    fn test_clear() {
        let mut cluster = Cluster::new(Point2::new(1.0, 2.0));
        cluster.insert(2, &Point2::new(2.0, 3.0));
        cluster.insert(3, &Point2::new(3.0, 5.0));
        cluster.insert(5, &Point2::new(5.0, 7.0));
        assert_eq!(cluster.size(), 3);

        cluster.clear();
        assert_eq!(cluster.centroid(), &Point2::zero());
        assert!(cluster.is_empty());
        assert_eq!(cluster.size(), 0);
        assert_eq!(cluster.membership(), &[]);
    }

    #[test]
    fn test_default() {
        let actual = Cluster::<f64, Point2<f64>>::default();
        assert!(actual.is_empty());
        assert_eq!(actual.centroid(), &Point2::zero());
        assert_eq!(actual.size(), 0);
        assert_eq!(actual.membership(), &[]);
    }
}
