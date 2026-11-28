use super::{point::Point, position::Position};
use crate::next::{
    color::{AlphaColor, Color, Oklab},
    image::{ImageBuf, Size},
};

/// Black.
pub(super) const BLACK: Color = gray(0.0);

/// White.
pub(super) const WHITE: Color = gray(1.0);

/// Creates a color from its OKLab values.
///
/// # Arguments
/// * `l` - The lightness.
/// * `a` - The green-red value.
/// * `b` - The blue-yellow value.
///
/// # Returns
/// The color with the given OKLab values.
pub(super) const fn color(l: f64, a: f64, b: f64) -> Color {
    Color::from_oklab(Oklab::new(l, a, b))
}

/// Creates a gray.
///
/// # Arguments
/// * `l` - The lightness of the gray in OKLab, from 0.0 to 1.0.
///
/// # Returns
/// The gray with lightness `l`.
pub(super) const fn gray(l: f64) -> Color {
    color(l, 0.0, 0.0)
}

/// Creates a gradient from black to white.
///
/// # Returns
/// `N` grays of evenly spaced lightness, from black to white.
pub(super) fn gradient<const N: usize>() -> [Color; N] {
    std::array::from_fn(|index| gray(index as f64 / (N - 1) as f64))
}

/// Creates an image from rows of colors, with `None` for a transparent pixel.
///
/// # Arguments
/// * `rows` - The colors of the pixels, row by row.
///
/// # Returns
/// The image with opaque pixels of the given colors.
pub(super) fn image<const W: usize>(rows: &[[Option<Color>; W]]) -> ImageBuf<AlphaColor<Color>> {
    let pixels = rows
        .iter()
        .flatten()
        .map(|color| match color {
            Some(color) => AlphaColor::opaque(*color),
            None => AlphaColor::transparent(BLACK),
        })
        .collect();
    ImageBuf::new(Size::new(W as u32, rows.len() as u32), pixels)
}

/// Creates a point of a color at the origin with weight 1.0.
///
/// # Arguments
/// * `color` - The color of the point.
///
/// # Returns
/// The point of `color` at the origin, counting fully in a mean.
pub(super) fn point(color: Color) -> Point<5> {
    Point::new(color, Position::new(0.0, 0.0), 1.0)
}

/// Creates a point of a color at the origin with weight 0.0.
///
/// # Arguments
/// * `color` - The color of the point.
///
/// # Returns
/// The point of `color` at the origin, counting for nothing in a mean.
pub(super) fn weightless(color: Color) -> Point<5> {
    Point::new(color, Position::new(0.0, 0.0), 0.0)
}

/// Clusters items the textbook way, comparing every pair of clusters before each merge.
///
/// # Arguments
/// * `count` - The number of items.
/// * `threshold` - The distance at or above which two clusters stay apart.
/// * `measure` - The distance between two clusters, from their items.
///
/// # Returns
/// The items of each cluster in ascending order, in the order of their first item.
pub(super) fn cluster_naively(
    count: usize,
    threshold: f64,
    measure: impl Fn(&[usize], &[usize]) -> f64,
) -> Vec<Vec<usize>> {
    let mut clusters: Vec<Vec<usize>> = (0..count).map(|index| vec![index]).collect();
    while merge_closest_naively(threshold, &measure, &mut clusters) {}
    clusters
}

/// Merges the closest pair of clusters, if it is under the threshold.
///
/// # Arguments
/// * `threshold` - The distance at or above which two clusters stay apart.
/// * `measure` - The distance between two clusters, from their items.
/// * `clusters` - The items of each cluster.
///
/// # Returns
/// `true` if a pair was merged.
fn merge_closest_naively(
    threshold: f64,
    measure: &impl Fn(&[usize], &[usize]) -> f64,
    clusters: &mut Vec<Vec<usize>>,
) -> bool {
    // The closest pair under the threshold, the first one on a tie.
    let mut smallest = threshold;
    let mut lower = 0;
    let mut upper = 0;
    for first in 0..clusters.len() {
        for second in first + 1..clusters.len() {
            let distance = measure(&clusters[first], &clusters[second]);
            if distance < smallest {
                smallest = distance;
                lower = first;
                upper = second;
            }
        }
    }
    if smallest >= threshold {
        return false;
    }

    let merged = clusters.remove(upper);
    clusters[lower].extend(merged);
    clusters[lower].sort_unstable();
    true
}
