use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;
use std::cmp::Ordering;

/// Wrapper struct for comparing clusters based on their size.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the clustering algorithm.
#[derive(Debug)]
pub struct SizeOrdered<F: Float, P: Point<F>>(pub Cluster<F, P>);

impl<F, P> SizeOrdered<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Returns the size of the wrapped cluster.
    ///
    /// # Returns
    /// The size of the wrapped cluster.
    #[must_use]
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

impl<F, P> PartialOrd for SizeOrdered<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.size().cmp(&other.size()))
    }
}

impl<F, P> Ord for SizeOrdered<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.size().cmp(&other.size())
    }
}

impl<F, P> PartialEq for SizeOrdered<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl<F, P> Eq for SizeOrdered<F, P>
where
    F: Float,
    P: Point<F>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::cluster::Cluster;
    use crate::math::point::Point2;
    use std::collections::BinaryHeap;

    #[test]
    fn test_size_ordered() {
        let cluster1 = {
            let mut cluster = Cluster::default();
            cluster.insert(0, &Point2(2.0, 3.0));
            cluster
        };
        let cluster2 = {
            let mut cluster = Cluster::default();
            cluster.insert(1, &Point2(0.0, 1.0));
            cluster.insert(2, &Point2(0.0, 0.0));
            cluster.insert(3, &Point2(1.0, 0.0));
            cluster.insert(4, &Point2(1.0, 1.0));
            cluster
        };
        let cluster3 = {
            let mut cluster = Cluster::default();
            cluster.insert(5, &Point2(5.0, 7.0));
            cluster.insert(6, &Point2(5.0, 5.0));
            cluster
        };

        let mut heap = BinaryHeap::new();
        heap.push(SizeOrdered(cluster1));
        heap.push(SizeOrdered(cluster2));
        heap.push(SizeOrdered(cluster3));

        assert_eq!(heap.pop().map(|ordered| ordered.size()), Some(4));
        assert_eq!(heap.pop().map(|ordered| ordered.size()), Some(2));
        assert_eq!(heap.pop().map(|ordered| ordered.size()), Some(1));
        assert_eq!(heap.pop(), None);
    }
}
