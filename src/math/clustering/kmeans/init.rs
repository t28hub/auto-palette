use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::Point;
use rand::Rng;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Initializer<R>
where
    R: Rng + Clone,
{
    #[allow(unused)]
    Random(R),
    #[allow(unused)]
    KmeansPlusPlus(R),
}

impl<R> Initializer<R>
where
    R: Rng + Clone,
{
    pub(crate) fn initialize<F: Float, P: Point<F>>(
        &self,
        dataset: &[P],
        k: usize,
        metric: &Distance,
    ) -> Vec<P> {
        if k == 0 {
            return vec![];
        }
        if k >= dataset.len() {
            let mut centroids = Vec::with_capacity(dataset.len());
            centroids.extend(dataset.iter());
            return centroids;
        }
        match self {
            Self::Random(rng) => Self::random(dataset, k, &mut rng.clone()),
            Self::KmeansPlusPlus(rng) => {
                Self::kmeans_plus_plus(dataset, k, metric, &mut rng.clone())
            }
        }
    }

    fn random<F: Float, P: Point<F>>(dataset: &[P], k: usize, rng: &mut R) -> Vec<P> {
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
        metric: &Distance,
        rng: &mut R,
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
                        .map(|centroid| metric.measure(point, centroid))
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
}

#[cfg(test)]
mod tests {
    use crate::math::clustering::kmeans::init::Initializer;
    use crate::math::distance::Distance;
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn random_initialize() {
        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let initializer = Initializer::Random(thread_rng());
        let result = initializer.initialize(&dataset, 2, &Distance::Euclidean);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn kmeans_plus_plus_initialize() {
        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let initializer = Initializer::KmeansPlusPlus(thread_rng());
        let result = initializer.initialize(&dataset, 2, &Distance::SquaredEuclidean);
        assert_eq!(result.len(), 2);
    }
}
