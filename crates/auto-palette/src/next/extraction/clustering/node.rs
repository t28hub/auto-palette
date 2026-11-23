use super::{bbox::Bbox, cluster::Cluster};
use crate::next::{
    color::Color,
    extraction::{centroid::Centroid, point::Point},
    math::Vector,
};

/// A node of the merge hierarchy, with what the linkage measures.
#[derive(Debug)]
pub(super) struct Node {
    /// The indices of the points.
    members: Vec<usize>,
    /// The weighted mean of the points.
    centroid: Centroid,
    /// The box around the coordinates of the colors of the points.
    bbox: Bbox,
}

impl Node {
    /// Creates the node of one point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point, with weight.
    /// * `coordinates` - The coordinates of the color of the point.
    ///
    /// # Returns
    /// The node of the point alone.
    pub(super) fn of_point(index: usize, point: Point, coordinates: Vector<3>) -> Self {
        let mut centroid = Centroid::default();
        centroid.add(point);
        Self {
            members: vec![index],
            centroid,
            bbox: Bbox::of(coordinates),
        }
    }

    /// Absorbs the points of another node.
    ///
    /// # Arguments
    /// * `other` - The node to absorb.
    pub(super) fn absorb(&mut self, other: Self) {
        self.centroid.merge(other.centroid);
        self.bbox.extend(other.bbox);
        self.members.extend(other.members);
    }

    /// Returns the mean color of the points.
    ///
    /// # Returns
    /// The mean color, weighted by the weights of the points.
    pub(super) fn mean(&self) -> Color {
        self.centroid.point().expect("a node has weight").color()
    }

    /// Returns the box around the coordinates of the colors of the points.
    ///
    /// # Returns
    /// The box around the coordinates.
    #[inline]
    pub(super) const fn bbox(&self) -> &Bbox {
        &self.bbox
    }

    /// Converts the node into a cluster, sorting its points.
    ///
    /// # Returns
    /// The cluster of the same points.
    pub(super) fn into_cluster(mut self) -> Cluster {
        self.members.sort_unstable();
        Cluster::new(self.members)
    }
}
