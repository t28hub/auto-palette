use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use crate::next::{
    extraction::point::Point,
    math::{DistanceMetric, Vector},
};

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

    /// Finds the member closest to the others in total, weighted by their weights.
    ///
    /// # Arguments
    /// * `points` - The points that were clustered.
    ///
    /// # Returns
    /// The index of the member, the lowest one on a tie.
    pub(crate) fn find_medoid<const N: usize>(&self, points: &[Point<N>]) -> usize {
        // Members at one vector are equally far from the others, so each vector is measured once.
        let locations = merge_same_locations(points, &self.members);

        let mut medoid = self.members[0];
        let mut smallest = f64::INFINITY;
        for location in &locations {
            let total_distance = location.total_distances_to(&locations);
            // Locations come in the order of their first member, so a tie keeps the lowest index.
            if total_distance < smallest {
                smallest = total_distance;
                medoid = location.first;
            }
        }
        medoid
    }
}

/// The members of a cluster at one vector, with their total weight.
#[derive(Debug, Clone, Copy)]
struct Location<const N: usize> {
    /// The lowest index of the members at this vector.
    first: usize,
    /// The vector of the members.
    vector: Vector<N>,
    /// The sum of the weights of the members at this vector.
    weight: f64,
}

impl<const N: usize> Location<N> {
    /// Creates the location of one point.
    ///
    /// # Arguments
    /// * `index` - The index of the point.
    /// * `point` - The point, with weight.
    ///
    /// # Returns
    /// The location of the point alone.
    fn of_point(index: usize, point: Point<N>) -> Self {
        Self {
            first: index,
            vector: point.vector(),
            weight: point.weight(),
        }
    }

    /// Adds a point at the same vector as the first point.
    ///
    /// # Arguments
    /// * `point` - The point, with weight.
    fn add_point(&mut self, point: Point<N>) {
        self.weight += point.weight();
    }

    /// Totals the distances to some locations, weighted by their weights.
    ///
    /// # Arguments
    /// * `others` - The locations to measure the distances to.
    ///
    /// # Returns
    /// The sum of the distances to `others`, each times its weight.
    fn total_distances_to(&self, others: &[Self]) -> f64 {
        others
            .iter()
            .map(|other| {
                let distance = DistanceMetric::Euclidean.measure(self.vector, other.vector);
                other.weight * distance
            })
            .sum()
    }
}

/// Merges the members of a cluster at the same vector.
///
/// # Arguments
/// * `points` - The points that were clustered.
/// * `members` - The indices of the points in the cluster, in ascending order.
///
/// # Returns
/// Each vector once, in the order of its first member.
fn merge_same_locations<const N: usize>(
    points: &[Point<N>],
    members: &[usize],
) -> Vec<Location<N>> {
    let mut indices: FxHashMap<[u64; N], usize> = FxHashMap::default();
    let mut locations: Vec<Location<N>> = Vec::new();
    for &member in members {
        let point = points[member];
        let coordinates = point.vector().0;
        let bits = coordinates.map(f64::to_bits);
        match indices.entry(bits) {
            // A member at the same vector came first, so this one adds its weight to it.
            Entry::Occupied(entry) => {
                let location = &mut locations[*entry.get()];
                location.add_point(point);
            }
            // The first member at this vector.
            Entry::Vacant(entry) => {
                entry.insert(locations.len());
                locations.push(Location::of_point(member, point));
            }
        }
    }
    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a point on a line.
    fn point(x: f64, weight: f64) -> Point<1> {
        Point::of_vector(Vector([x]), weight)
    }

    #[test]
    fn test_find_medoid_weighs_distances() {
        // Arrange: the middle point is the medoid by count, the heavy one by weight.
        let points = [point(0.0, 1.0), point(0.1, 1.0), point(0.2, 3.0)];
        let cluster = Cluster::new(vec![0, 1, 2]);

        // Act
        let actual = cluster.find_medoid(&points);

        // Assert
        assert_eq!(actual, 2);
    }

    #[test]
    fn test_find_medoid_picks_lowest_index_on_tie() {
        // Arrange
        let points = [point(0.1, 1.0), point(0.0, 1.0)];
        let cluster = Cluster::new(vec![0, 1]);

        // Act
        let actual = cluster.find_medoid(&points);

        // Assert
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_find_medoid_counts_every_member_at_a_vector() {
        // Arrange: a tie, unless both points at 0.0 count.
        let points = [point(0.1, 1.0), point(0.0, 1.0), point(0.0, 1.0)];
        let cluster = Cluster::new(vec![0, 1, 2]);

        // Act
        let actual = cluster.find_medoid(&points);

        // Assert
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_find_medoid_skips_points_of_other_clusters() {
        // Arrange
        let points = [
            // 1st cluster
            point(0.0, 1.0),
            // 2nd cluster
            point(5.0, 9.0),
            // 1st cluster
            point(0.1, 1.0),
            point(0.2, 1.0),
        ];
        let cluster = Cluster::new(vec![0, 2, 3]);

        // Act
        let actual = cluster.find_medoid(&points);

        // Assert
        assert_eq!(actual, 2);
    }
}
