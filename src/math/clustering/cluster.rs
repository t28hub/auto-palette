use crate::math::number::Float;
use crate::math::point::Point;
use std::marker::PhantomData;

/// Struct representing a cluster.
#[derive(Debug, PartialEq)]
pub struct Cluster<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// The label of this cluster.
    pub label: usize,
    /// The centroid of this cluster.
    pub(crate) centroid: P,
    /// The indices of the points in the dataset that belong to this cluster.
    membership: Vec<usize>,
    _marker: PhantomData<F>,
}

impl<F, P> Cluster<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Create a new `Cluster` with the given label.
    ///
    /// # Arguments
    /// * `label` - The label of the new cluster.
    ///
    /// # Returns
    /// A new `Cluster`.
    pub fn new(label: usize) -> Self {
        Self {
            label,
            centroid: P::zero(),
            membership: Vec::new(),
            _marker: PhantomData::default(),
        }
    }

    /// Return a reference to the centroid of this cluster.
    ///
    /// # Returns
    /// A reference to the centroid of this cluster.
    pub fn centroid(&self) -> &P {
        &self.centroid
    }

    /// Check whether this cluster is empty.
    ///
    /// # Returns
    /// `true` if this cluster is empty.
    pub fn is_empty(&self) -> bool {
        self.membership.is_empty()
    }

    /// Return the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    pub fn size(&self) -> usize {
        self.membership.len()
    }

    /// Insert a point with index.
    ///
    /// # Arguments
    /// * `index` - The index of the point to insert.
    /// * `point` - The reference of the point to insert.
    pub fn insert(&mut self, index: usize, point: &P) {
        self.centroid += *point;
        self.membership.push(index);
    }

    /// Clear the centroid and all the membership.
    pub fn clear(&mut self) {
        self.centroid.set_zero();
        self.membership.clear();
    }
}
