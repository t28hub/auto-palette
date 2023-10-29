use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::dbscan::algorithm::DBSCAN;
use crate::math::clustering::gmeans::algorithm::Gmeans;
use crate::math::distance::DistanceMetric;
use crate::math::number::Float;
use crate::math::point::Point;

/// Enum representing the supported palette extraction algorithms.
///
/// # Examples
/// ```ignore
/// use auto_palette::{Algorithm, Palette};
///
/// let image = image::open("./path/to/image.png").unwrap();
/// let palette = Palette::extract_with_algorithm(&image, &Algorithm::GMeans);
/// let palette = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
/// ```
#[derive(Debug)]
pub enum Algorithm {
    /// G-means clustering algorithm.
    GMeans,
    /// DBSCAN clustering algorithm.
    DBSCAN,
}

impl Algorithm {
    /// Applies the clustering algorithm to the given points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    ///
    /// # Type Parameters
    /// * `F` - The float type used for calculations.
    /// * `P` - The point type used for calculations.
    pub(crate) fn apply<F, P>(&self, points: &[P]) -> Vec<Cluster<F, P>>
    where
        F: Float,
        P: Point<F>,
    {
        match self {
            Algorithm::GMeans => cluster_with_gmeans(points),
            Algorithm::DBSCAN => cluster_with_dbscan(points),
        }
    }
}

#[must_use]
fn cluster_with_gmeans<F, P>(points: &[P]) -> Vec<Cluster<F, P>>
where
    F: Float,
    P: Point<F>,
{
    let min_cluster_size = (points.len() / 4096).max(25);
    let gmeans = Gmeans::new(
        25,
        10,
        min_cluster_size,
        F::from_f64(1e-3),
        &DistanceMetric::SquaredEuclidean,
    );
    gmeans.fit(points)
}

#[must_use]
fn cluster_with_dbscan<F, P>(points: &[P]) -> Vec<Cluster<F, P>>
where
    F: Float,
    P: Point<F>,
{
    let min_samples = (points.len() / 4096).max(25);
    let dbscan = DBSCAN::new(
        min_samples,
        F::from_f64(0.0025),
        &DistanceMetric::SquaredEuclidean,
    );
    let (clusters, _) = dbscan.fit(points);
    clusters
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[must_use]
    fn sample_points() -> Vec<Point2<f64>> {
        vec![
            Point2(0.0, 0.0),
            Point2(0.1, 0.1),
            Point2(0.1, 0.2),
            Point2(0.2, 0.2),
            Point2(0.2, 0.4),
            Point2(0.3, 0.5),
            Point2(0.1, 0.0),
            Point2(0.0, 0.1),
            Point2(0.0, 0.2),
            Point2(0.0, 0.0),
            Point2(0.1, 0.1),
            Point2(0.1, 0.2),
            Point2(0.2, 0.2),
            Point2(0.2, 0.4),
            Point2(0.3, 0.5),
            Point2(0.1, 0.0),
            Point2(0.0, 0.1),
            Point2(0.0, 0.2),
            Point2(0.0, 0.0),
            Point2(0.1, 0.1),
            Point2(0.1, 0.2),
            Point2(0.2, 0.2),
            Point2(0.2, 0.4),
            Point2(0.3, 0.5),
            Point2(0.1, 0.0),
            Point2(0.0, 0.1),
            Point2(0.0, 0.2),
            Point2(0.0, 0.0),
            Point2(0.1, 0.1),
            Point2(0.1, 0.2),
            Point2(0.2, 0.2),
            Point2(0.2, 0.4),
            Point2(0.3, 0.5),
            Point2(0.1, 0.0),
            Point2(0.0, 0.1),
            Point2(0.0, 0.2),
            Point2(0.0, 0.0),
            Point2(0.1, 0.1),
            Point2(0.1, 0.2),
            Point2(0.2, 0.2),
            Point2(0.2, 0.4),
            Point2(0.3, 0.5),
            Point2(0.1, 0.0),
        ]
    }

    #[test]
    fn test_gmeans_algorithm() {
        let points = sample_points();
        let actual = Algorithm::GMeans.apply(&points);

        let clustering = Gmeans::new(25, 10, 16, 0.001, &DistanceMetric::SquaredEuclidean);
        let expected = clustering.fit(&points);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dbscan_algorithm() {
        let points = sample_points();
        let actual = Algorithm::DBSCAN.apply(&points);

        let clustering = DBSCAN::new(16, 0.0025, &DistanceMetric::SquaredEuclidean);
        let expected = clustering.fit(&points);
        assert_eq!(actual, expected.0);
    }
}
