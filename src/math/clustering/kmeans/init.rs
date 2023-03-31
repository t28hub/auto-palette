use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::Point;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Enum representing the initialization method for K-means clustering algorithm.
#[derive(Debug, PartialEq)]
pub enum Initialization<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    /// Random initialization method.
    #[allow(unused)]
    Random(R),

    /// K-means++ initialization method.
    #[allow(unused)]
    KmeansPlusPlus(Distance, R),

    /// Precomputed centroids initialization method.
    #[allow(unused)]
    Precomputed(Vec<P>),

    /// Dummy variant to allow using F parameter.
    #[allow(unused)]
    _Maker(PhantomData<F>),
}

impl<F, P, R> Initialization<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    /// Initializes the cluster centroids.
    ///
    /// # Arguments
    /// * `dataset` - A slice of dataset points.
    /// * `k` - The number of cluster centroids.
    ///
    /// # Returns
    /// A vector of cluster centroids.
    #[must_use]
    pub(crate) fn initialize(&self, dataset: &[P], k: usize) -> Vec<P> {
        if k == 0 {
            return Vec::new();
        }

        if k >= dataset.len() {
            return Vec::from_iter(dataset.iter().cloned());
        }

        match self {
            Self::Random(rng) => random(dataset, k, &mut rng.clone()),
            Self::KmeansPlusPlus(distance, rng) => {
                kmeans_plus_plus(dataset, k, distance, &mut rng.clone())
            }
            Self::Precomputed(centroids) => {
                assert_eq!(
                    centroids.len(),
                    k,
                    "The number of precomputed centroids must match the number of clusters."
                );
                centroids.clone()
            }
            Self::_Maker(_) => unreachable!(),
        }
    }
}

fn random<F: Float, P: Point<F>>(dataset: &[P], k: usize, rng: &mut impl Rng) -> Vec<P> {
    let mut selected = vec![false; dataset.len()];
    let mut centroids = Vec::with_capacity(k);
    while centroids.len() < k {
        let index = rng.gen_range(0..dataset.len());
        if selected[index] {
            continue;
        }

        let point = dataset.get(index);
        if let Some(centroid) = point {
            selected.insert(index, true);
            centroids.push(*centroid);
        }
    }
    centroids
}

fn kmeans_plus_plus<F: Float, P: Point<F>>(
    dataset: &[P],
    k: usize,
    distance: &Distance,
    rng: &mut impl Rng,
) -> Vec<P> {
    let mut selected = vec![false; dataset.len()];
    let mut centroids = Vec::with_capacity(k);

    let index = rng.gen_range(0..dataset.len());
    selected.insert(index, true);
    centroids.push(dataset[index]);
    while centroids.len() < k {
        let furthest = dataset
            .iter()
            .enumerate()
            .map(|(index, point)| -> (usize, F) {
                if selected[index] {
                    return (index, F::zero());
                }

                let min_distance = centroids
                    .iter()
                    .map(|centroid| distance.measure(point, centroid))
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater));
                if let Some(min) = min_distance {
                    (index, min)
                } else {
                    (index, F::zero())
                }
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Greater));

        if let Some((index, _)) = furthest {
            selected.insert(index, true);
            centroids.push(dataset[index]);
        } else {
            break;
        }
    }
    centroids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::Distance;
    use crate::math::point::Point2;
    use rand::thread_rng;

    fn sample_dataset() -> Vec<Point2<f64>> {
        vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ]
    }

    #[test]
    fn test_random() {
        let dataset = sample_dataset();
        let initialization = Initialization::Random(thread_rng());

        let actual = initialization.initialize(&dataset, 2);
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn test_kmeans_plus_plus() {
        let dataset = sample_dataset();
        let initialization =
            Initialization::KmeansPlusPlus(Distance::SquaredEuclidean, thread_rng());

        let actual = initialization.initialize(&dataset, 2);
        assert_eq!(actual.len(), 2);
    }

    #[test]
    fn test_precomputed() {
        let centroids = vec![Point2(1.0, 2.0), Point2(3.0, 1.0)];
        let initialization = Initialization::Precomputed::<_, _, ThreadRng>(centroids);

        let dataset = sample_dataset();
        let actual = initialization.initialize(&dataset, 2);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Point2(1.0, 2.0));
        assert_eq!(actual[1], Point2(3.0, 1.0));
    }

    #[test]
    #[should_panic(
        expected = "The number of precomputed centroids must match the number of clusters."
    )]
    fn test_precomputed_should_panic_if_k_does_not_match() {
        let centroids = vec![Point2(1.0, 2.0)];
        let initialization = Initialization::Precomputed::<_, _, ThreadRng>(vec![Point2(1.0, 2.0)]);

        let dataset = sample_dataset();
        let _ = initialization.initialize(&dataset, 2);
    }
}
