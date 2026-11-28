use super::random::SplitMix64;
use crate::next::{
    extraction::{centroid::Centroid, point::Point},
    math::DistanceMetric,
};

/// A method that picks the initial centers of k-means.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Initialization {
    /// Greedy k-means++, which draws centers at random, favoring those far from the previous ones.
    PlusPlus {
        /// The seed of the random number generator.
        seed: u64,
    },
    /// Farthest first, which picks the point farthest from the previous centers.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    Farthest,
}

impl Initialization {
    /// The default seed for [`Self::PlusPlus`].
    const DEFAULT_SEED: u64 = 0;

    /// Picks the initial centers.
    ///
    /// # Arguments
    /// * `points` - The points, with at least one of positive weight.
    /// * `count` - The number of centers to pick.
    ///
    /// # Returns
    /// Up to `count` centers, fewer if every point already sits on a center.
    pub(super) fn centers<const N: usize>(
        self,
        points: &[Point<N>],
        count: usize,
    ) -> Vec<Point<N>> {
        match self {
            Self::PlusPlus { seed } => plus_plus(points, count, &mut SplitMix64::new(seed)),
            Self::Farthest => farthest_first(points, count),
        }
    }
}

impl Default for Initialization {
    fn default() -> Self {
        Self::PlusPlus {
            seed: Self::DEFAULT_SEED,
        }
    }
}

/// Picks the initial centers with greedy k-means++ ([`Initialization::PlusPlus`]).
///
/// # Arguments
/// * `points` - The points, with at least one of positive weight.
/// * `count` - The number of centers to pick.
/// * `random` - The random number generator.
///
/// # Returns
/// Up to `count` centers, fewer if every point already sits on a center.
fn plus_plus<const N: usize>(
    points: &[Point<N>],
    count: usize,
    random: &mut SplitMix64,
) -> Vec<Point<N>> {
    let weights = cumulative(points.iter().map(|point| point.weight()));
    let first = draw(&weights, random);
    let mut centers = vec![points[first]];
    let mut distances = vec![f64::INFINITY; points.len()];
    shrink(&mut distances, points, centers[0]);
    let log_count = (count as f64).ln() as usize;
    let candidates = 2 + log_count;
    while centers.len() < count {
        let weights = cumulative(distances.iter().copied());
        if weights.last() == Some(&0.0) {
            break;
        }
        let potential = |candidate: Point<N>| -> f64 {
            distances
                .iter()
                .zip(points)
                .map(|(&distance, &point)| nearest_distance(distance, point, candidate))
                .sum()
        };
        let (_, best) = (0..candidates)
            .map(|_| points[draw(&weights, random)])
            .map(|candidate| (potential(candidate), candidate))
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .expect("there are at least two candidates");
        shrink(&mut distances, points, best);
        centers.push(best);
    }
    centers
}

/// Picks the initial centers farthest first ([`Initialization::Farthest`]).
///
/// # Arguments
/// * `points` - The points, with at least one of positive weight.
/// * `count` - The number of centers to pick.
///
/// # Returns
/// Up to `count` centers, fewer if every point already sits on a center.
fn farthest_first<const N: usize>(points: &[Point<N>], count: usize) -> Vec<Point<N>> {
    let mut mean = Centroid::default();
    for &point in points {
        mean.add(point);
    }
    let mut centers = vec![mean.point().expect("there is a point of positive weight")];
    let mut distances = vec![f64::INFINITY; points.len()];
    shrink(&mut distances, points, centers[0]);
    while centers.len() < count {
        let (farthest, distance) = distances
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .expect("there is at least one point");
        if *distance == 0.0 {
            break;
        }
        let center = points[farthest];
        shrink(&mut distances, points, center);
        centers.push(center);
    }
    centers
}

/// Lowers the weighted distance of each point to its nearest center for a new center.
///
/// # Arguments
/// * `distances` - The weighted squared distance of each point to its nearest center so far.
/// * `points` - The points to measure from.
/// * `center` - The new center.
fn shrink<const N: usize>(distances: &mut [f64], points: &[Point<N>], center: Point<N>) {
    for (distance, point) in distances.iter_mut().zip(points) {
        *distance = nearest_distance(*distance, *point, center);
    }
}

/// Returns the weighted squared distance of a point to its nearest center, with a new center added.
///
/// # Arguments
/// * `distance` - The weighted squared distance of the point to its nearest center so far.
/// * `point` - The point to measure from.
/// * `center` - The new center.
///
/// # Returns
/// The smaller of `distance` and the weight of `point` times its squared distance to `center`.
#[inline]
fn nearest_distance<const N: usize>(distance: f64, point: Point<N>, center: Point<N>) -> f64 {
    let squared = point.distance(center, DistanceMetric::SquaredEuclidean);
    let weighted = point.weight() * squared;
    distance.min(weighted)
}

/// Returns the running totals of weights.
///
/// # Arguments
/// * `weights` - The weights, zero or more each.
///
/// # Returns
/// The sum of the weights up to and including each index.
fn cumulative(weights: impl IntoIterator<Item = f64>) -> Vec<f64> {
    weights
        .into_iter()
        .scan(0.0, |total, weight| {
            *total += weight;
            Some(*total)
        })
        .collect()
}

/// Draws an index with probability proportional to its weight.
///
/// # Arguments
/// * `cumulative` - The running totals of the weights, with a positive last total.
/// * `random` - The random number generator.
///
/// # Returns
/// The index drawn, less than the number of weights.
fn draw(cumulative: &[f64], random: &mut SplitMix64) -> usize {
    let target = random.next_f64() * cumulative[cumulative.len() - 1];
    cumulative
        .partition_point(|&total| total <= target)
        .min(cumulative.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::{
            color::Color,
            extraction::test_helpers::{gradient, gray, image, BLACK, WHITE},
        },
    };

    /// Creates the points of a row of opaque pixels, with positions scaled.
    fn row<const W: usize>(colors: [Color; W], scale: f64) -> Vec<Point<5>> {
        Point::of_pixels(&image(&[colors.map(Some)]), scale)
    }

    #[test]
    fn test_centers_depend_on_seed() {
        // Arrange: a gradient, so the initial centers depend on the random draws.
        let points = row(gradient::<16>(), 0.0);
        let pick_lightnesses = |seed| -> Vec<f64> {
            let centers = Initialization::PlusPlus { seed }.centers(&points, 4);
            centers
                .iter()
                .map(|center| {
                    let [lightness, ..] = center.vector().0;
                    lightness
                })
                .collect()
        };

        // Act
        let first = pick_lightnesses(0);
        let second = pick_lightnesses(1);

        // Assert
        assert_ne!(first, second);
    }

    #[test]
    fn test_farthest_first_starts_from_mean() {
        // Arrange
        let points = row([BLACK, gray(0.4), WHITE], 0.0);

        // Act
        let actual = farthest_first(&points, 1);

        // Assert: (0.0 + 0.4 + 1.0) / 3
        let [lightness, ..] = actual[0].vector().0;
        assert_approx_eq!(lightness, 1.4 / 3.0);
    }

    #[test]
    fn test_farthest_first_picks_farthest_point() {
        // Arrange: white is farther than black from the mean.
        let points = row([BLACK, gray(0.4), WHITE], 0.0);

        // Act
        let actual = farthest_first(&points, 2);

        // Assert
        let [lightness, ..] = actual[1].vector().0;
        assert_approx_eq!(lightness, 1.0);
    }

    #[test]
    fn test_farthest_first_weighs_position() {
        // Arrange: the same color in a row, so only the positions differ.
        let points = row([BLACK; 4], 1.0);

        // Act
        let actual = farthest_first(&points, 2);

        // Assert: the mean, then an end of the row.
        assert_approx_eq!((actual[1].position().x - 0.5).abs(), 0.375);
    }

    #[test]
    fn test_farthest_first_stops_when_every_point_is_a_center() {
        // Arrange
        let points = row([BLACK, WHITE], 0.0);

        // Act
        let actual = farthest_first(&points, 8);

        // Assert: the mean, white, and black.
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_shrink() {
        // Arrange
        let points = row([BLACK, WHITE], 0.0);
        let mut distances = [f64::INFINITY, 0.5];

        // Act
        shrink(&mut distances, &points, points[0]);

        // Assert: black is on the center; white keeps the smaller distance.
        assert_eq!(distances, [0.0, 0.5]);
    }

    #[test]
    fn test_cumulative() {
        // Act
        let actual = cumulative([1.0, 0.0, 2.0]);

        // Assert
        assert_eq!(actual, [1.0, 1.0, 3.0]);
    }

    #[test]
    fn test_draw_skips_zero_weights() {
        // Arrange: only the middle index has weight.
        let weights = [0.0, 1.0, 1.0];
        let mut random = SplitMix64::new(0);

        // Act
        let actual: Vec<usize> = (0..10).map(|_| draw(&weights, &mut random)).collect();

        // Assert
        assert_eq!(actual, [1; 10]);
    }
}
