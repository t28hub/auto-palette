/// A group of points.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Cluster {
    /// The indices of the points, in ascending order.
    members: Vec<usize>,
}

impl Cluster {
    /// Creates a cluster of points.
    ///
    /// # Arguments
    /// * `members` - The indices of the points, in ascending order.
    ///
    /// # Returns
    /// A cluster of the given points.
    #[inline]
    pub(super) const fn new(members: Vec<usize>) -> Self {
        Self { members }
    }

    /// Returns the indices of the points.
    ///
    /// # Returns
    /// The indices of the points, in ascending order.
    #[inline]
    pub(crate) fn members(&self) -> &[usize] {
        &self.members
    }
}
