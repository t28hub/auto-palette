use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::HashSet;

/// Struct representing a trained clustering model.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
/// * `P` - The type of points used in the clustering algorithm.
#[derive(Debug, PartialEq)]
pub struct Model<F: Float, P: Point<F>> {
    clusters: Vec<Cluster<F, P>>,
    outliers: HashSet<usize>,
}

impl<F, P> Model<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Create a new `Model` instance.
    ///
    /// # Arguments
    /// * `clusters` - The clusters found in the dataset.
    /// * `outliers` - The indices of the data points that are outliers.
    ///
    /// # Returns
    /// A new `Model` instance.
    pub fn new(clusters: Vec<Cluster<F, P>>, outliers: HashSet<usize>) -> Self {
        Self { clusters, outliers }
    }

    /// Return the clusters.
    ///
    /// # Returns
    /// A reference to a slice of clusters.
    #[must_use]
    pub fn clusters(&self) -> &[Cluster<F, P>] {
        &self.clusters
    }

    /// Return the indices of outliers.
    ///
    /// # Returns
    /// A slice containing the indices of the outliers.
    #[allow(unused)]
    #[must_use]
    pub fn outliers(&self) -> &HashSet<usize> {
        &self.outliers
    }
}

impl<F, P> Default for Model<F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn default() -> Self {
        Self {
            clusters: Vec::new(),
            outliers: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::cluster::Cluster;
    use crate::math::point::Point2;
    use std::collections::HashSet;

    #[test]
    fn test_model() {
        let clusters = vec![{
            let mut cluster = Cluster::new(Point2::new(0.0, 0.0));
            cluster.insert(0, &Point2::new(1.0, 1.0));
            cluster.insert(4, &Point2::new(2.0, 2.0));
            cluster.insert(5, &Point2::new(3.0, 3.0));
            cluster
        }];
        let outliers = HashSet::from([1, 2, 3]);
        let actual = Model::new(clusters, outliers);
        assert_eq!(actual.clusters().len(), 1);
        assert_eq!(actual.outliers(), &HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_default() {
        let actual = Model::<f64, Point2<f64>>::default();
        assert!(actual.clusters().is_empty());
        assert!(actual.outliers().is_empty());
    }
}
