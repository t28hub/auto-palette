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
    pub label: usize,
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
    /// * `label` - The label of the new cluster.
    ///
    /// # Returns
    /// A new `Cluster` instance.
    pub fn new(label: usize) -> Self {
        Self {
            label,
            centroid: P::zero(),
            membership: Vec::new(),
            _marker: PhantomData::default(),
        }
    }

    /// Returns a reference to the centroid of this cluster.
    ///
    /// # Returns
    /// A reference to the centroid of this cluster.
    pub fn centroid(&self) -> &P {
        &self.centroid
    }

    /// Checks whether this cluster is empty.
    ///
    /// # Returns
    /// `true` if this cluster is empty.
    pub fn is_empty(&self) -> bool {
        self.membership.is_empty()
    }

    /// Returns the number of points in this cluster.
    ///
    /// # Returns
    /// The number of points in this cluster.
    pub fn size(&self) -> usize {
        self.membership.len()
    }

    /// Checks whether this cluster contains the point with the given index.
    ///
    /// # Arguments
    /// * `index` - The index of the point to check.
    ///
    /// # Returns
    /// `true` if this cluster contains the point with the given index.
    pub fn contains(&self, index: usize) -> bool {
        self.membership.contains(&index)
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
