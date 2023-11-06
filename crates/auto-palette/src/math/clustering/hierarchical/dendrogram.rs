use crate::math::clustering::hierarchical::node::Node;
use crate::math::clustering::hierarchical::priority::Priority;
use crate::number::Float;
use std::collections::BinaryHeap;

/// Struct representing a dendrogram.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug)]
pub struct Dendrogram<F>
where
    F: Float,
{
    nodes: Vec<Node<F>>,
}

impl<F> Dendrogram<F>
where
    F: Float,
{
    /// Creates a new `Dendrogram` instance with the given capacity.
    ///
    /// # Arguments
    /// * `capacity` - The capacity of the new dendrogram.
    ///
    /// # Returns
    /// A new `Dendrogram` instance.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of nodes in this dendrogram.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns a reference to the nodes of this dendrogram.
    #[must_use]
    pub fn nodes(&self) -> &[Node<F>] {
        &self.nodes
    }

    /// Pushes a new node to this dendrogram.
    ///
    /// # Arguments
    /// * `node` - The node to push.
    pub fn push(&mut self, node: Node<F>) {
        assert!(self.len() < self.nodes.capacity());
        self.nodes.push(node);
    }

    /// Partitions this dendrogram into `n` clusters.
    ///
    /// # Arguments
    /// * `n` - The number of clusters to partition this dendrogram into.
    ///
    /// # Returns
    /// A vector of nodes representing the clusters.
    #[must_use]
    pub fn partition(&self, n: usize) -> Vec<Node<F>> {
        let mut heap = BinaryHeap::new();
        if let Some(node) = self.nodes.last() {
            heap.push(Priority::new(node, node.distance));
        }

        let mut membership = Vec::with_capacity(n);
        while membership.len() < n {
            if heap.len() + membership.len() >= n {
                membership.extend(heap.iter().map(|&Priority(node, _)| node.clone()));
                membership.truncate(n);
                break;
            }

            let Some(Priority(node, _)) = heap.pop() else {
                break;
            };

            if let Some(node1) = node.node1 {
                let node = &self.nodes[node1];
                heap.push(Priority::new(node, node.distance));
            } else {
                membership.push(node.clone());
            }

            if let Some(node2) = node.node2 {
                let node = &self.nodes[node2];
                heap.push(Priority::new(node, node.distance));
            } else {
                membership.push(node.clone());
            }
        }
        membership
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dendrogram = Dendrogram::<f32>::new(5);
        assert_eq!(dendrogram.len(), 0);
    }

    #[test]
    fn test_partition() {
        let mut dendrogram = Dendrogram::new(5);
        dendrogram.push(Node::new(0, None, None, 0.0));
        dendrogram.push(Node::new(1, None, None, 0.0));
        dendrogram.push(Node::new(2, None, None, 0.0));
        dendrogram.push(Node::new(3, Some(0), Some(2), 2.0));
        dendrogram.push(Node::new(4, Some(1), Some(3), 5.0));

        let actual = dendrogram.partition(0);
        assert_eq!(actual.len(), 0);

        let actual = dendrogram.partition(1);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].label, 4);

        let actual = dendrogram.partition(2);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].label, 3);
        assert_eq!(actual[1].label, 1);

        let actual = dendrogram.partition(3);
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0].label, 1);
        assert_eq!(actual[1].label, 0);
        assert_eq!(actual[2].label, 2);

        let actual = dendrogram.partition(4);
        assert_eq!(actual.len(), 4);
        assert_eq!(actual[0].label, 1);
        assert_eq!(actual[1].label, 1);
        assert_eq!(actual[2].label, 0);
        assert_eq!(actual[3].label, 2);
    }
}
