use crate::math::clustering::hierarchical::dendrogram::Dendrogram;
use crate::math::clustering::hierarchical::linkage::{Linkage, SingleLinkage};
use crate::math::clustering::hierarchical::node::Node;
use crate::math::clustering::hierarchical::priority::Priority;
use crate::number::Float;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub struct HierarchicalClustering;

impl<'a> HierarchicalClustering {
    #[allow(unused)]
    #[must_use]
    pub fn fit<F, T, DF>(&self, dataset: &'a [T], distance_fn: &'a DF) -> Dendrogram<F>
    where
        F: Float,
        DF: Fn(&T, &T) -> F,
    {
        self.fit_with_linkage(dataset, &mut SingleLinkage::new(dataset, distance_fn))
    }

    #[must_use]
    pub fn fit_with_linkage<F, T>(
        &self,
        dataset: &'a [T],
        linkage: &mut impl Linkage<F>,
    ) -> Dendrogram<F>
    where
        F: Float,
    {
        let n_points = dataset.len();
        let mut dendrogram = Dendrogram::new(n_points * 2 - 1);
        dataset.iter().enumerate().for_each(|(i, _)| {
            let node = Node::new(i, None, None, F::zero());
            dendrogram.push(node);
        });

        let mut heap = BinaryHeap::new();
        for i in 0..dendrogram.len() {
            for j in (i + 1)..dendrogram.len() {
                let distance = linkage.distance(i, j);
                let priority = Priority::new(NodePair::new(i, j, distance), distance);
                heap.push(Reverse(priority));
            }
        }

        let mut inactive_nodes = HashSet::new();
        while let Some(Reverse(Priority(pair, _))) = heap.pop() {
            let label1 = pair.label1;
            let label2 = pair.label2;
            if inactive_nodes.contains(&label1) || inactive_nodes.contains(&label2) {
                continue;
            }

            let label = linkage.merge(label1, label2);
            inactive_nodes.insert(label1);
            inactive_nodes.insert(label2);

            let merged_node = Node::new(label, Some(label1), Some(label2), pair.distance);
            dendrogram.push(merged_node);

            for i in 0..label {
                if inactive_nodes.contains(&i) {
                    continue;
                }

                let distance = linkage.distance(i, label);
                let priority = Priority::new(NodePair::new(i, label, distance), distance);
                heap.push(Reverse(priority));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::hierarchical::node::Node;
    use crate::math::distance::DistanceMetric;
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
        let dendrogram = clustering.fit(&points, &|p1: &Point2<f64>, p2: &Point2<f64>| {
            DistanceMetric::Euclidean.measure(p1, p2)
        });
        assert_eq!(dendrogram.len(), 9);

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

        let nodes = dendrogram.partition(2);
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0], Node::new(6, Some(1), Some(4), 1.0));
        assert_eq!(nodes[1], Node::new(7, Some(2), Some(5), 1.0));
    }
}
