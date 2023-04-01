use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::dbscan::clustering::DBSCAN;
use crate::math::clustering::gmeans::clustering::Gmeans;
use crate::math::clustering::hdbscan::clustering::HDBSCAN;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::Point;

/// Enum representing the supported palette extraction algorithms.
pub enum Algorithm {
    DBSCAN,
    GMEANS,
    HDBSCAN,
}

impl Algorithm {
    /// Applies the selected palette extraction algorithm.
    ///
    /// # Arguments
    /// * `dataset` - A slice of data points.
    ///
    /// # Returns
    /// A trained `Model` containing the results of the clustering algorithm applied to the dataset.
    pub(crate) fn apply<F, P>(&self, dataset: &[P]) -> Model<F, P>
    where
        F: Float,
        P: Point<F>,
    {
        match self {
            Algorithm::DBSCAN => {
                let dbscan = DBSCAN::new(9, F::from_f64(0.0025), Distance::SquaredEuclidean);
                dbscan.train(dataset)
            }
            Algorithm::GMEANS => {
                let gmeans =
                    Gmeans::new(25, 10, 25, F::from_f64(0.001), Distance::SquaredEuclidean);
                gmeans.train(dataset)
            }
            Algorithm::HDBSCAN => {
                let hdbscan = HDBSCAN::new(9, 25, Distance::SquaredEuclidean);
                hdbscan.train(dataset)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    fn sample_dataset() -> Vec<Point2<f64>> {
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.1, 0.1),
            Point2::new(0.1, 0.2),
            Point2::new(0.2, 0.2),
            Point2::new(0.2, 0.4),
            Point2::new(0.3, 0.5),
            Point2::new(0.1, 0.0),
            Point2::new(0.0, 0.1),
            Point2::new(0.0, 0.2),
        ]
    }

    #[test]
    fn test_dbscan_algorithm() {
        let dataset = sample_dataset();
        let actual = Algorithm::DBSCAN.apply(&dataset);

        let clustering = DBSCAN::new(9, 0.0025, Distance::SquaredEuclidean);
        let expected = clustering.train(&dataset);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_gmeans_algorithm() {
        let dataset = sample_dataset();
        let actual = Algorithm::GMEANS.apply(&dataset);

        let clustering = Gmeans::new(25, 10, 25, 0.001, Distance::SquaredEuclidean);
        let expected = clustering.train(&dataset);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hdbscan_algorithm() {
        let dataset = sample_dataset();
        let actual = Algorithm::HDBSCAN.apply(&dataset);

        let clustering = HDBSCAN::new(9, 25, Distance::SquaredEuclidean);
        let expected = clustering.train(&dataset);
        assert_eq!(actual, expected);
    }
}
