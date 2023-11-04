use crate::math::distance::DistanceMetric;
use crate::math::point::Point;
use crate::number::Float;
use std::cmp::Ordering;

pub trait Linkage<F>
where
    F: Float,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F;

    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize;
}

#[derive(Debug)]
pub struct SingleLinkage<F>
where
    F: Float,
{
    matrix: Vec<Vec<F>>,
    next_index: usize,
}

impl<F> SingleLinkage<F>
where
    F: Float,
{
    #[must_use]
    pub fn new<'a, P>(points: &'a [P], metric: &'a DistanceMetric) -> Self
    where
        P: Point<F>,
    {
        let n_points = points.len();
        let n_clusters = n_points * 2 - 1;
        let mut matrix = Vec::with_capacity(n_clusters);
        for i in 0..n_clusters {
            let mut row = Vec::with_capacity(n_clusters);
            for j in i..n_clusters {
                let distance = if i == j {
                    F::max_value()
                } else if i < n_points && j < n_points {
                    metric.measure(&points[i], &points[j])
                } else {
                    F::max_value()
                };
                row.push(distance);
            }
            matrix.push(row);
        }

        Self {
            matrix,
            next_index: points.len(),
        }
    }
}

impl<F> Linkage<F> for SingleLinkage<F>
where
    F: Float,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        match i.cmp(&j) {
            Ordering::Less => self.matrix[i][j - i],
            Ordering::Equal => F::max_value(),
            Ordering::Greater => self.matrix[j][i - j],
        }
    }

    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize {
        assert!(i < j, "i must be less than j: {} < {}", i, j);

        let label = self.next_index;
        for k in 0..label {
            let distance1 = self.distance(i, k);
            let distance2 = self.distance(j, k);
            self.matrix[k][label - k] = distance1.min(distance2);
        }

        self.next_index += 1;
        label
    }
}
