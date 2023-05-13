use crate::color::lab::Lab;
use crate::color::rgb::RGB;
use crate::color::xyz::XYZ;
use crate::color_struct::Color;
use crate::image::image_data::ImageData;
use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::dbscan::algorithm::DBSCAN;
use crate::math::distance::Distance;
use crate::math::graph::edge::Edge;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::Float;
use crate::math::point::{Point3, Point5};
use crate::swatch::Swatch;
use crate::{Algorithm, Theme};
use num_traits::Zero;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

/// Struct representing a color palette.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
///
/// # Example
/// ```no_run
/// extern crate image;
///
/// use auto_palette::{Algorithm, Palette, SimpleImageData};
///
/// let img = image::open("/path/to/image.png").unwrap();
/// let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();
/// let palette: Palette<f64> = Palette::extract(&image_data);
/// palette.swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch.color().to_hex_string());
///     println!("{:?}", swatch.position());
///     println!("{:?}", swatch.population());
/// });
/// ```
#[derive(Debug)]
pub struct Palette<F: Float> {
    swatches: Vec<Swatch<F>>,
}

impl<F> Palette<F>
where
    F: Float,
{
    /// Extract a color palette from the given image.
    ///
    /// # Arguments
    /// * `image_data` - The image data to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[must_use]
    pub fn extract<I: ImageData>(image_data: &I) -> Palette<F> {
        Self::extract_with_algorithm(image_data, &Algorithm::DBSCAN)
    }

    /// Extract a color palette from the given image using the specified algorithm.
    ///
    /// # Arguments
    /// * `image_data` - The image data to use for color palette extraction.
    /// * `algorithm` - The algorithm to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[must_use]
    pub fn extract_with_algorithm<I: ImageData>(
        image_data: &I,
        algorithm: &Algorithm,
    ) -> Palette<F> {
        let pixels = convert_to_pixels(image_data);

        // Merge pixels that are close in color and position, and exclude outliers.
        let model = algorithm.apply(&pixels);
        let pixel_clusters = model.clusters();
        let (candidates, colors): (Vec<_>, Vec<_>) = pixel_clusters
            .iter()
            .filter_map(|cluster| {
                pixel_cluster_to_swatch(cluster, image_data.width(), image_data.height())
            })
            .map(|swatch| {
                let Lab { l, a, b, .. } = swatch.color().to_lab();
                let point = Point3(l, a, b);
                (swatch, point)
            })
            .unzip();

        // Merge colors with small color differences and extract the dominant swatches.
        // Set epsilon to 2.0 for DBSCAN, as DeltaE below 2.0 is perceptible to human eyes only through close observation.
        let dbscan = DBSCAN::new(1, F::from_f64(2.0), Distance::Euclidean);
        let model2 = dbscan.train(&colors);
        let mut swatches: Vec<_> = model2
            .clusters()
            .iter()
            .filter_map(|cluster| color_cluster_to_swatch(cluster, &candidates))
            .collect();
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));
        Self { swatches }
    }

    /// Returns the number of swatches in this palette.
    ///
    /// # Returns
    /// The number of swatches in this palette.
    #[must_use]
    pub fn len(&self) -> usize {
        self.swatches.len()
    }

    /// Returns `true` if this palette contains no swatches.
    ///
    /// # Returns
    /// `true` if this palette contains no swatches.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swatches.is_empty()
    }

    /// Finds the dominant swatches in this palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    ///
    /// # Returns
    /// The `n` dominant swatches in this palette.
    #[must_use]
    pub fn swatches(&self, n: usize) -> Vec<Swatch<F>> {
        if self.swatches.is_empty() {
            return Vec::new();
        }

        let mut results = self.find_swatches(n, |swatch| F::from_usize(swatch.population()));
        results.sort_by_key(|swatch| Reverse(swatch.population()));
        results
    }

    /// Finds the dominant swatches in this palette using the specified theme.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    /// * `theme` - The theme to use for color palette extraction.
    ///
    /// # Returns
    /// The `n` dominant swatches in this palette.
    #[must_use]
    pub fn swatches_with_theme(&self, n: usize, theme: &impl Theme) -> Vec<Swatch<F>> {
        if self.swatches.is_empty() {
            return Vec::new();
        }

        let mut results = self.find_swatches(n, |swatch| {
            let fraction = theme.weight(swatch);
            fraction.value()
        });
        results.sort_by(|swatch1, swatch2| {
            let weight1 = theme.weight(swatch1).value();
            let weight2 = theme.weight(swatch2).value();
            weight1
                .partial_cmp(&weight2)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        results.iter().take(n).cloned().collect()
    }

    #[must_use]
    fn find_swatches<SF>(&self, n: usize, score_fn: SF) -> Vec<Swatch<F>>
    where
        SF: Fn(&Swatch<F>) -> F,
    {
        let mut candidates = HashMap::<usize, Swatch<F>>::new();
        let mut heap = BinaryHeap::new();
        for (i, swatch_i) in self.swatches.iter().enumerate() {
            candidates.insert(i, swatch_i.clone());
            for (j, swatch_j) in self.swatches.iter().skip(i + 1).enumerate() {
                if i == j {
                    continue;
                }

                let distance = swatch_i.distance(swatch_j);
                heap.push(Reverse(WeightedEdge::new(i, j, distance)));
            }
        }

        let mut next_label = self.swatches.len();
        while candidates.len() > n {
            let Some(Reverse(edge)) = heap.pop() else {
                break;
            };

            let Some(swatch1) = candidates.get(&edge.u()) else {
                continue;
            };
            let Some(swatch2) = candidates.get(&edge.v()) else {
                continue;
            };

            let score1 = score_fn(swatch1);
            let score2 = score_fn(swatch2);
            if score1.is_zero() || score2.is_zero() {
                continue;
            }

            let population = swatch1.population() + swatch2.population();
            let best_swatch = if score1 >= score2 {
                Swatch::new(swatch1.color().clone(), swatch1.position(), population)
            } else {
                Swatch::new(swatch2.color().clone(), swatch2.position(), population)
            };

            candidates.iter().for_each(|(label, swatch)| {
                if label == &edge.u() || label == &edge.v() {
                    return;
                }

                let distance1 = swatch.distance(swatch1);
                let distance2 = swatch.distance(swatch2);
                let distance = (distance1 * score1 + distance2 * score2) / (score1 + score2);
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            candidates.remove(&edge.u());
            candidates.remove(&edge.v());
            candidates.insert(next_label, best_swatch);
            next_label += 1;
        }
        candidates.into_values().collect()
    }
}

/// Converts the given image data to pixels.
///
/// # Arguments
/// * `image_data` - The image data to convert.
///
/// # Returns
/// A vector of `Point5` instances.
#[must_use]
fn convert_to_pixels<F, I>(image_data: &I) -> Vec<Point5<F>>
where
    F: Float,
    I: ImageData,
{
    let width = image_data.width() as usize;
    let width_f = F::from_u32(image_data.width());
    let height_f = F::from_u32(image_data.height());
    image_data
        .data()
        .chunks_exact(4)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = chunk[3];

            // Ignore a transparent pixel
            if a.is_zero() {
                return None;
            }

            let rgb = RGB::new(r, g, b);
            let xyz: XYZ<F> = XYZ::from(&rgb);
            let lab: Lab<F> = Lab::from(&xyz);

            let x = i % width;
            let y = i / width;

            let pixel = Point5::new(
                lab.l.normalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
                lab.a.normalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
                lab.b.normalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
                F::from_usize(x) / width_f,
                F::from_usize(y) / height_f,
            );
            Some(pixel)
        })
        .collect()
}

/// Converts the given pixel cluster to a swatch.
///
/// # Arguments
/// * `pixel_cluster` - The pixel cluster to convert.
/// * `width` - The width of the source image.
/// * `height` - The height of the source image.
///
/// # Returns
/// A swatch representing the given cluster.
#[must_use]
fn pixel_cluster_to_swatch<F>(
    pixel_cluster: &Cluster<F, Point5<F>>,
    width: u32,
    height: u32,
) -> Option<Swatch<F>>
where
    F: Float,
{
    let width_f = F::from_u32(width);
    let height_f = F::from_u32(height);
    if pixel_cluster.is_empty() {
        return None;
    }

    let centroid = pixel_cluster.centroid();
    let lab = Lab::<F>::new(
        centroid[0].denormalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
        centroid[1].denormalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
        centroid[2].denormalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
    );
    let color = Color::from(&lab);

    let x = centroid[3].denormalize(F::zero(), width_f);
    let y = centroid[4].denormalize(F::zero(), height_f);
    let position = (
        x.to_u32().expect("Could not convert x to u32"),
        y.to_u32().expect("Could not convert y to u32"),
    );
    Some(Swatch::new(color, position, pixel_cluster.size()))
}

/// Converts the given color cluster to a swatch.
///
/// # Arguments
/// * `color_cluster` - The color cluster to convert.
/// * `candidates` - The candidate swatches.
///
/// # Returns
/// A swatch representing the given cluster.
#[inline]
#[must_use]
fn color_cluster_to_swatch<F>(
    color_cluster: &Cluster<F, Point3<F>>,
    candidates: &[Swatch<F>],
) -> Option<Swatch<F>>
where
    F: Float,
{
    if color_cluster.is_empty() {
        return None;
    }

    let membership = color_cluster.membership();
    let Some(first_swatch) = membership.first().map(|label| candidates[*label].clone()) else {
        return None;
    };

    let best_swatch = membership
        .iter()
        .skip(1)
        .map(|label| &candidates[*label])
        .fold(first_swatch, |previous, current| {
            let population = previous.population() + current.population();
            let fraction = F::from_usize(current.population()) / F::from_usize(population);
            let color = previous.color().mix(current.color(), fraction);
            let position = if fraction <= F::from_f64(0.5) {
                previous.position()
            } else {
                current.position()
            };
            Swatch::new(color, position, population)
        });
    Some(best_swatch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleImageData;

    #[test]
    fn test_extract() {
        let data = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 255, 255, // white
        ];
        let image_data = SimpleImageData::new(2, 2, &data).unwrap();
        let palette: Palette<f64> = Palette::extract(&image_data);

        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }
}
