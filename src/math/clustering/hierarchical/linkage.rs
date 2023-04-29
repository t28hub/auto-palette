use crate::math::number::Float;

pub trait Linkage<F: Float> {
    #[must_use]
    fn size(&self) -> usize;

    #[must_use]
    fn distance(&self, u: usize, v: usize) -> F;

    fn union(&mut self, u: usize, v: usize);
}

pub struct SingleLinkage<F: Float> {
    size: usize,
    distances: Vec<F>,
}

impl<F> SingleLinkage<F>
where
    F: Float,
{
    #[allow(unused)]
    #[must_use]
    pub fn build<T, DF>(dataset: &[T], distance_fn: DF) -> Self
    where
        DF: Fn(&T, &T) -> F,
    {
        let size = dataset.len();
        let capacity = size * (size - 1) / 2;
        let mut distances = Vec::with_capacity(capacity);
        for u in 0..size {
            for v in 0..u {
                let data_u = &dataset[u];
                let data_v = &dataset[v];
                distances.push(distance_fn(data_u, data_v));
            }
        }
        Self { size, distances }
    }

    /// Returns the index corresponding to the upper triangular matrix.
    #[inline]
    #[must_use]
    fn index(&self, u: usize, v: usize) -> usize {
        assert!(u < self.size && v < self.size, "Index out of bounds");
        if u > v {
            u * (u - 1) / 2 + v
        } else {
            v * (v - 1) / 2 + u
        }
    }
}

impl<F> Linkage<F> for SingleLinkage<F>
where
    F: Float,
{
    #[must_use]
    fn size(&self) -> usize {
        self.size
    }

    #[inline]
    #[must_use]
    fn distance(&self, u: usize, v: usize) -> F {
        let index = self.index(u, v);
        self.distances[index]
    }

    #[inline]
    fn union(&mut self, u: usize, v: usize) {
        for i in 0..self.size {
            if i == u || i == v {
                continue;
            }

            if i < u {
                let index = self.index(u, i);
                let min_distance = self.distance(u, i).min(self.distance(v, i));
                self.distances[index] = min_distance;
            } else {
                let index = self.index(i, u);
                let min_distance = self.distance(i, u).min(self.distance(i, v));
                self.distances[index] = min_distance;
            }
        }
    }
}
