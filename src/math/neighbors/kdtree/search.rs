use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::node::KDNode;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Div;

/// Struct representing kd-tree search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    root: Option<Box<KDNode>>,
    points: Cow<'a, [P]>,
    metric: &'a Distance,
    _marker: PhantomData<F>,
}

impl<'a, F, P> KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F> + 'a,
{
    /// Creates a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    #[must_use]
    pub fn new(points: &'a [P], metric: &'a Distance) -> Self {
        let mut indices: Vec<usize> = (0..points.len()).collect();
        let root = Self::build_node(points, &mut indices, 0);

        Self {
            root: root.map(Box::new),
            points: Cow::Borrowed(points),
            metric,
            _marker: PhantomData::default(),
        }
    }

    #[inline]
    #[must_use]
    fn build_node(points: &[P], indices: &mut [usize], depth: usize) -> Option<KDNode> {
        if indices.is_empty() {
            return None;
        }

        let axis = depth % points[0].dimension();
        indices.sort_unstable_by(|&index1, &index2| {
            let value1 = points[index1].index(axis);
            let value2 = points[index2].index(axis);
            value1.partial_cmp(value2).unwrap_or(Ordering::Equal)
        });

        let node = {
            let median = indices.len().div(2);
            KDNode::new(
                indices[median],
                axis,
                Self::build_node(points, &mut indices[..median], depth + 1),
                Self::build_node(points, &mut indices[median + 1..], depth + 1),
            )
        };
        Some(node)
    }

    #[inline]
    fn search_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        k: usize,
        neighbors: &mut Vec<Neighbor<F>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.points[node.index];
        let distance = self.metric.measure(&point, query);
        let neighbor = Neighbor::new(node.index, distance);
        if neighbors.len() < k {
            neighbors.push(neighbor);
            neighbors.sort_unstable_by(|neighbor1, neighbor2| {
                neighbor1
                    .distance
                    .partial_cmp(&neighbor2.distance)
                    .unwrap_or(Ordering::Equal)
            });
        } else if distance < neighbors[k - 1].distance {
            neighbors.pop();
            neighbors.push(neighbor);
            neighbors.sort_unstable_by(|neighbor1, neighbor2| {
                neighbor1
                    .distance
                    .partial_cmp(&neighbor2.distance)
                    .unwrap_or(Ordering::Equal)
            });
        }

        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        if neighbors.len() < k || delta.abs() <= neighbors[k - 1].distance {
            self.search_recursively(node.left(), query, k, neighbors);
            self.search_recursively(node.right(), query, k, neighbors);
        } else if delta < F::zero() {
            self.search_recursively(node.left(), query, k, neighbors);
        } else {
            self.search_recursively(node.right(), query, k, neighbors);
        }
    }

    #[inline]
    fn search_radius_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        radius: F,
        neighbors: &mut Vec<Neighbor<F>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.points[node.index];
        let distance = self.metric.measure(&point, query);
        if distance <= radius {
            neighbors.push(Neighbor::new(node.index, distance));
        }

        let delta = query[node.axis] - point[node.axis];
        if delta.abs() <= radius {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        } else if delta < F::zero() {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
        } else {
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors = Vec::new();
        self.search_recursively(&self.root, query, k, &mut neighbors);
        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors.into_iter().take(k).collect()
    }

    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut neighbors = Vec::new();
        self.search_radius_recursively(&self.root, query, radius, &mut neighbors);
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[must_use]
    fn sample_points() -> Vec<Point2<f64>> {
        vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
            Point2(0.0, 5.0),
            Point2(2.0, 1.0),
            Point2(5.0, 2.0),
        ]
    }

    #[test]
    fn test_kdtree_search() {
        let points = sample_points();
        let kdtree = KDTreeSearch::new(&points, &Distance::Euclidean);
        assert!(kdtree.root.as_ref().is_some());
        assert_eq!(kdtree.points, points);
        assert_eq!(kdtree.metric, &Distance::Euclidean);
        assert_eq!(kdtree._marker, PhantomData);
    }

    #[test]
    fn test_search() {
        let points = sample_points();
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);

        let actual = kdtree.search(&Point2(3.0, 3.0), 0);
        assert_eq!(actual, vec![]);

        let actual = kdtree.search(&Point2(3.0, 3.0), 1);
        assert_eq!(actual, vec![Neighbor::new(4, 2.0)]);

        let mut actual = kdtree.search(&Point2(3.0, 3.0), 2);
        actual.sort_unstable_by(|neighbor1, neighbors2| {
            neighbor1
                .distance
                .partial_cmp(&neighbors2.distance)
                .unwrap_or(Ordering::Equal)
        });
        assert_eq!(actual, vec![Neighbor::new(4, 2.0), Neighbor::new(1, 4.0)]);

        let mut actual = kdtree.search(&Point2(3.0, 3.0), 10);
        actual.sort_unstable_by(|neighbor1, neighbors2| {
            neighbor1
                .distance
                .partial_cmp(&neighbors2.distance)
                .unwrap_or(Ordering::Equal)
        });
        assert_eq!(
            actual,
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(7, 5.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(3, 8.0),
                Neighbor::new(5, 13.0),
            ]
        );
    }

    #[test]
    fn test_search_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);
        assert_eq!(kdtree.search(&Point2(3.0, 3.0), 4), vec![]);
    }

    #[test]
    fn test_search_nearest() {
        let points = sample_points();
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);
        assert_eq!(
            kdtree.search_nearest(&Point2(2.5, 3.0)),
            Some(Neighbor::new(4, 1.25))
        );
    }

    #[test]
    fn test_search_nearest_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);
        assert_eq!(kdtree.search_nearest(&Point2(2.5, 3.0)), None);
    }

    #[test]
    fn test_search_radius() {
        let points = sample_points();
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);

        let actual = kdtree.search_radius(&Point2(3.0, 3.0), -1.0);
        assert_eq!(actual, vec![]);

        let actual = kdtree.search_radius(&Point2(3.0, 3.0), 1.0);
        assert_eq!(actual, vec![]);

        let actual = kdtree.search_radius(&Point2(3.0, 3.0), 2.0);
        assert_eq!(actual, vec![Neighbor::new(4, 2.0)]);

        let mut actual = kdtree.search_radius(&Point2(2.0, 2.5), 2.5);
        actual.sort_unstable_by(|neighbor1, neighbors2| {
            neighbor1
                .distance
                .partial_cmp(&neighbors2.distance)
                .unwrap_or(Ordering::Equal)
        });
        assert_eq!(
            actual,
            vec![
                Neighbor::new(0, 1.25),
                Neighbor::new(6, 2.25),
                Neighbor::new(4, 2.25),
            ]
        );

        let mut actual = kdtree.search_radius(&Point2(1.0, 3.0), 23.0);
        actual.sort_unstable_by(|neighbor1, neighbors2| {
            neighbor1
                .distance
                .partial_cmp(&neighbors2.distance)
                .unwrap_or(Ordering::Equal)
        });
        assert_eq!(
            actual,
            vec![
                Neighbor::new(0, 1.0),
                Neighbor::new(4, 2.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(5, 5.0),
                Neighbor::new(1, 8.0),
                Neighbor::new(2, 13.0),
                Neighbor::new(7, 17.0),
                Neighbor::new(3, 20.0),
            ]
        );
    }

    #[test]
    fn test_search_radius_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let kdtree = KDTreeSearch::new(&points, &Distance::SquaredEuclidean);
        assert_eq!(kdtree.search_radius(&Point2(3.0, 3.0), 5.0), vec![]);
    }
}
