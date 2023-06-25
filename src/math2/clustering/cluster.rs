use crate::number::Float;
use ndarray::{Array1, ArrayView1, NdFloat};

/// Struct representing a cluster.
///
/// # Type parameters
/// * `F` - The float type used for calculations.
#[derive(Debug, Clone, PartialEq)]
pub struct Cluster<F>
where
    F: Float,
{
    centroid: Array1<F>,
    membership: Vec<usize>,
}

impl<F> Cluster<F>
where
    F: Float + NdFloat,
{
    /// Creates a new `Cluster` instance.
    ///
    /// # Arguments
    /// * `centroid` - The initial centroid of the new cluster.
    /// * `membership` - The initial membership of the new cluster.
    ///
    /// # Returns
    /// A new `Cluster` instance.
    #[must_use]
    pub fn new(centroid: Array1<F>, membership: Vec<usize>) -> Self {
        Self {
            centroid,
            membership,
        }
    }

    /// Returns a reference to the centroid of this cluster.
    ///
    /// # Returns
    /// A reference to the centroid of this cluster.
    #[inline]
    #[must_use]
    pub fn centroid(&self) -> ArrayView1<F> {
        self.centroid.view()
    }

    /// Returns a reference to the membership of this cluster.
    ///
    /// # Returns
    /// A reference to the membership of this cluster.
    #[inline]
    #[must_use]
    pub fn membership(&self) -> &[usize] {
        &self.membership
    }

    /// Checks whether this cluster is empty.
    ///
    /// # Returns
    /// `true` if this cluster is empty, otherwise `false`.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.membership.is_empty()
    }

    /// Returns the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.membership.len()
    }

    /// Inserts a point and updates the centroid.
    ///
    /// # Arguments
    /// * `index` - The index of the point to insert.
    /// * `point` - The point to insert.
    #[inline]
    pub fn insert(&mut self, index: usize, point: &ArrayView1<F>) {
        let diff = point - &self.centroid;
        let size = F::from_usize(self.membership.len() + 1);
        self.centroid += &(&diff / size);
        self.membership.push(index);
    }

    /// Clears the centroid and membership.
    #[inline]
    pub fn clear(&mut self) {
        self.centroid.fill(F::zero());
        self.membership.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, aview1};

    #[test]
    fn test_new() {
        let centroid = array![1.0, 2.0, 3.0];
        let membership = vec![0, 1, 2];
        let actual = Cluster::new(centroid, membership);

        assert_eq!(actual.centroid, array![1.0, 2.0, 3.0]);
        assert_eq!(actual.membership, vec![0, 1, 2]);
    }

    #[test]
    fn test_is_empty() {
        let centroid = array![1.0, 2.0, 3.0];
        let membership = vec![0, 1, 2];
        let cluster = Cluster::new(centroid, membership);
        assert!(!cluster.is_empty());

        let centroid = array![0.0, 0.0, 0.0];
        let membership = vec![];
        let cluster = Cluster::new(centroid, membership);
        assert!(cluster.is_empty());
    }

    #[test]
    fn test_size() {
        let centroid = array![1.0, 2.0, 3.0];
        let membership = vec![0, 1, 2];
        let mut cluster = Cluster::new(centroid, membership);
        assert_eq!(cluster.size(), 3);

        let centroid = array![0.0, 0.0, 0.0];
        let membership = vec![];
        let cluster = Cluster::new(centroid, membership);
        assert_eq!(cluster.size(), 0);
    }

    #[test]
    fn test_insert() {
        let centroid = array![1.0, 2.0, 3.0];
        let membership = vec![0, 1, 2];
        let mut cluster = Cluster::new(centroid, membership);
        cluster.insert(3, &aview1(&[4.0, 5.0, 6.0]));

        assert_eq!(cluster.centroid(), aview1(&[1.75, 2.75, 3.75]));
        assert_eq!(cluster.membership(), &[0, 1, 2, 3]);
    }

    #[test]
    fn test_clear() {
        let centroid = array![1.0, 2.0, 3.0];
        let membership = vec![0, 1, 2];
        let mut cluster = Cluster::new(centroid, membership);
        cluster.clear();

        assert_eq!(cluster.centroid(), aview1(&[0.0, 0.0, 0.0]));
        assert_eq!(cluster.membership(), &[]);
    }
}
