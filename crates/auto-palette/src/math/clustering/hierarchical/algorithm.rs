use crate::math::clustering::hierarchical::dendrogram::Dendrogram;
use crate::math::clustering::hierarchical::linkage::{Linkage, SingleLinkage};
use crate::math::clustering::hierarchical::node::Node;
use crate::math::distance::DistanceMetric;
use crate::math::point::Point;
use crate::number::Float;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

pub struct HierarchicalClustering;

impl<'a> HierarchicalClustering {
    #[must_use]
    pub fn fit<F, P>(&self, points: &'a [P]) -> Dendrogram<F>
    where
        F: Float,
        P: Point<F>,
    {
        self.fit_with_linkage(
            points,
            &mut SingleLinkage::new(points, &DistanceMetric::Euclidean),
        )
    }

    #[must_use]
    pub fn fit_with_linkage<F, P>(
        &self,
        points: &'a [P],
        linkage: &mut impl Linkage<F>,
    ) -> Dendrogram<F>
    where
        F: Float,
        P: Point<F>,
    {
        let mut dendrogram = Dendrogram::new(
            points
                .iter()
                .enumerate()
                .map(|(i, _)| Node::new(i, None, None, F::zero()))
                .collect(),
        );
        let mut heap = BinaryHeap::new();
        for i in 0..dendrogram.size() {
            for j in (i + 1)..dendrogram.size() {
                let pair = NodePair::new(i, j, linkage.distance(i, j));
                heap.push(Reverse(pair));
            }
        }

        let mut inactive_nodes = HashSet::new();
        while let Some(Reverse(pair)) = heap.pop() {
            let index1 = pair.label1;
            let index2 = pair.label2;
            if inactive_nodes.contains(&index1) || inactive_nodes.contains(&index2) {
                continue;
            }

            let index = linkage.merge(index1, index2);
            inactive_nodes.insert(index1);
            inactive_nodes.insert(index2);

            let merged_node = Node::new(index, Some(index1), Some(index2), pair.distance);
            dendrogram.add(merged_node);

            for i in 0..index {
                if inactive_nodes.contains(&i) {
                    continue;
                }

                let pair = NodePair::new(i, index, linkage.distance(i, index));
                heap.push(Reverse(pair));
            }
        }
        dendrogram
    }
}

#[derive(Debug)]
struct NodePair<F> {
    label1: usize,
    label2: usize,
    distance: F,
}

impl<F> NodePair<F>
where
    F: Float,
{
    #[must_use]
    fn new(label1: usize, label2: usize, distance: F) -> Self {
        Self {
            label1,
            label2,
            distance,
        }
    }
}

impl<F> PartialOrd for NodePair<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F> Ord for NodePair<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

impl<F> PartialEq for NodePair<F>
where
    F: Float,
{
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<F> Eq for NodePair<F> where F: Float {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::hierarchical::node::Node;
    use crate::math::point::Point2;

    #[test]
    fn test_fit() {
        let points = vec![
            Point2(1.0, 0.0),
            Point2(10.0, 0.0),
            Point2(3.0, 0.0),
            Point2(2.0, 0.0),
            Point2(11.0, 0.0),
        ];
        let clustering = HierarchicalClustering;
        let dendrogram = clustering.fit(&points);
        assert_eq!(dendrogram.size(), 9);

        let nodes = dendrogram.nodes();
        assert_eq!(nodes[0], Node::new(0, None, None, 0.0));
        assert_eq!(nodes[1], Node::new(1, None, None, 0.0));
        assert_eq!(nodes[2], Node::new(2, None, None, 0.0));
        assert_eq!(nodes[3], Node::new(3, None, None, 0.0));
        assert_eq!(nodes[4], Node::new(4, None, None, 0.0));
        assert_eq!(nodes[5], Node::new(5, Some(0), Some(3), 1.0));
        assert_eq!(nodes[6], Node::new(6, Some(1), Some(4), 1.0));
        assert_eq!(nodes[7], Node::new(7, Some(2), Some(5), 1.0));
        assert_eq!(nodes[8], Node::new(8, Some(6), Some(7), 7.0));
    }
}
