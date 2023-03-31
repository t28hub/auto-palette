use crate::math::number::Float;
use crate::math::point::Point;
use std::marker::PhantomData;

/// Struct representing a cluster.
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
