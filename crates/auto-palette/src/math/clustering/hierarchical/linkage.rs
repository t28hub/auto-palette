use crate::math::distance::DistanceMetric;
use crate::math::point::Point;
use crate::number::Float;
use num_traits::real::Real;
use std::borrow::Cow;

pub trait Linkage<F>
where
    F: Float,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F;

    fn merge(&mut self, i: usize, j: usize);
}

#[derive(Debug)]
pub struct SingleLinkage<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    points: Cow<'a, [P]>,
    metric: &'a DistanceMetric,
    matrix: Vec<Vec<F>>,
}

impl<'a, F, P> SingleLinkage<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    pub fn new(points: &'a [P], metric: &'a DistanceMetric) -> Self {
        let n_points = points.len();
        let n_clusters = n_points * 2 - 1;
        let mut matrix = Vec::with_capacity(n_clusters);
        for i in 0..n_clusters {
            let mut row = Vec::with_capacity(n_clusters);
            for j in i + 1..n_clusters {
                let distance = if i < n_points && j < n_points {
                    metric.distance(&points[i], &points[j])
                } else {
                    F::infinity()
                };
                row.push(distance);
            }
            matrix.push(row);
        }

        Self {
            points: Cow::Borrowed(points),
            metric,
            matrix,
        }
    }
}

impl<'a, F, P> Linkage<F> for SingleLinkage<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        if i == j {
            F::infinity()
        } else if i < j {
            self.matrix[i][j]
        } else {
            self.matrix[j][i]
        }
    }

    fn merge(&mut self, i: usize, j: usize) {
        assert!(i < j);
        for k in 0..self.matrix.len() {
            if i < k {
                self.matrix[i][k] = self.matrix[i][k].min(self.matrix[j][k]);
            } else {
                self.matrix[k][i] = self.matrix[k][i].min(self.matrix[k][j]);
            }
        }
        self.matrix[i][j] = F::infinity();
    }
}
