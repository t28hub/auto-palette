use crate::color::lab::Lab;
use crate::color::rgb::Rgb;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::color_trait::Color;
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
/// use auto_palette::color_trait::Color;
///
/// let img = image::open("/path/to/image.png").unwrap();
/// let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();
/// let palette: Palette<f64> = Palette::extract(&image_data);
/// palette.dominant_swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch.color().to_hex_string());
///     println!("{:?}", swatch.position());
///     println!("{:?}", swatch.population());
/// });
/// ```
#[derive(Debug)]
pub struct Palette<F: Float> {
    candidates: Vec<Swatch<Lab<F, D65>>>,
    color_groups: Vec<Vec<usize>>,
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
        Self::extract_with(image_data, Algorithm::DBSCAN)
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
    pub fn extract_with<I: ImageData>(image_data: &I, algorithm: Algorithm) -> Palette<F> {
        let pixels = convert_to_pixels(image_data);

        // Merge pixels that are close in color and position, and exclude outliers.
        let model = algorithm.apply(&pixels);
        let clusters = model.clusters();
        let (candidates, colors): (Vec<_>, Vec<_>) = clusters
            .iter()
            .filter_map(|cluster| {
                convert_to_swatch(cluster, image_data.width(), image_data.height()).map(|swatch| {
                    let color = swatch.color();
                    let point = Point3(color.l, color.a, color.b);
                    (swatch, point)
                })
            })
            .unzip();

        // Merge colors with small color differences and extract the dominant swatches.
        // Set epsilon to 2.0 for DBSCAN, as DeltaE below 2.0 is perceptible to human eyes only through close observation.
        let dbscan = DBSCAN::new(1, F::from_f64(2.0), Distance::Euclidean);
        let mut groups: Vec<_> = dbscan
            .train(&colors)
            .clusters()
            .iter()
            .filter_map(|cluster| {
                if cluster.is_empty() {
                    return None;
                }
                let membership = cluster.membership().to_vec();
                Some(membership)
            })
            .collect();
        groups.sort_by_key(|membership| Reverse(membership.len()));
        Self {
            candidates,
            color_groups: groups,
        }
    }

    /// Finds the dominant swatches in this palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    ///
    /// # Returns
    /// The `n` dominant swatches in this palette.
    #[must_use]
    pub fn dominant_swatches(&self, n: usize) -> Vec<Swatch<Lab<F, D65>>> {
        if self.color_groups.is_empty() {
            return Vec::new();
        }

        let swatches: Vec<_> = self
            .color_groups
            .iter()
            .map(|membership| {
                let merged = self.merge_to_swatch(membership, |swatch1, swatch2| {
                    let population = swatch1.population() + swatch2.population();
                    let fraction = F::from_usize(swatch2.population()) / F::from_usize(population);
                    let color = swatch1.color().mix(swatch2.color(), fraction);
                    let position = if fraction <= F::from_f64(0.5) {
                        swatch1.position()
                    } else {
                        swatch2.position()
                    };
                    Swatch::new(color, position, population)
                });
                merged.expect("Failed to merge swatches")
            })
            .collect();

        let mut candidates = HashMap::<usize, Swatch<Lab<F, D65>>>::new();
        let mut heap = BinaryHeap::new();
        for (i, swatch_i) in swatches.iter().enumerate() {
            candidates.insert(i, swatch_i.clone());
            for (j, swatch_j) in swatches.iter().skip(i + 1).enumerate() {
                if i == j {
                    continue;
                }
                let distance = swatch_i.distance(swatch_j);
                heap.push(Reverse(WeightedEdge::new(i, j, distance)));
            }
        }

        let mut next_label = swatches.len();
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

            let best_swatch = if swatch1.population() > swatch2.population() {
                swatch1.clone()
            } else {
                swatch2.clone()
            };

            candidates.iter().for_each(|(label, swatch)| {
                if label == &edge.u() || label == &edge.v() {
                    return;
                }
                let distance = best_swatch.distance(swatch);
                heap.push(Reverse(WeightedEdge::new(*label, next_label, distance)));
            });

            candidates.remove(&edge.u());
            candidates.remove(&edge.v());
            candidates.insert(next_label, best_swatch);
            next_label += 1;
        }

        let mut results: Vec<_> = candidates.into_values().collect();
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
    pub fn swatches_with_theme(&self, n: usize, theme: &Theme) -> Vec<Swatch<Lab<F, D65>>> {
        if self.candidates.is_empty() {
            return Vec::new();
        }

        let mut swatches: HashMap<_, _> = HashMap::with_capacity(self.color_groups.len());
        for (label, membership) in self.color_groups.iter().enumerate() {
            let merged = self.merge_to_swatch(membership, |swatch1, swatch2| {
                let population = swatch1.population() + swatch2.population();
                let score1 = theme.score(swatch1);
                let score2 = theme.score(swatch2);
                if score1 > score2 {
                    Swatch::new(swatch1.color().clone(), swatch1.position(), population)
                } else {
                    Swatch::new(swatch2.color().clone(), swatch2.position(), population)
                }
            });

            if let Some(swatch) = merged {
                swatches.insert(label, swatch);
            }
        }

        let mut results: Vec<_> = swatches.into_values().collect();
        results.sort_by(|swatch1, swatch2| {
            let weight1 = theme.score(swatch1);
            let weight2 = theme.score(swatch2);
            weight1
                .partial_cmp(&weight2)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        results.iter().take(n).cloned().collect()
    }

    #[inline]
    #[must_use]
    fn merge_to_swatch<A>(
        &self,
        membership: &[usize],
        accumulator: A,
    ) -> Option<Swatch<Lab<F, D65>>>
    where
        A: Fn(&Swatch<Lab<F, D65>>, &Swatch<Lab<F, D65>>) -> Swatch<Lab<F, D65>>,
    {
        if let Some(first_swatch) = membership.first().map(|label| &self.candidates[*label]) {
            let merged = membership
                .iter()
                .skip(1)
                .map(|label| &self.candidates[*label])
                .fold(first_swatch.clone(), |previous, swatch| {
                    accumulator(&previous, swatch)
                });
            Some(merged)
        } else {
            None
        }
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

            let rgb = Rgb::new(r, g, b);
            let xyz: XYZ<F, D65> = XYZ::from(&rgb);
            let lab: Lab<F, D65> = Lab::from(&xyz);

            let x = i % width;
            let y = i / width;

            let pixel = Point5::new(
                lab.l
                    .normalize(Lab::<F, D65>::min_l(), Lab::<F, D65>::max_l()),
                lab.a
                    .normalize(Lab::<F, D65>::min_a(), Lab::<F, D65>::max_a()),
                lab.b
                    .normalize(Lab::<F, D65>::min_b(), Lab::<F, D65>::max_b()),
                F::from_usize(x) / width_f,
                F::from_usize(y) / height_f,
            );
            Some(pixel)
        })
        .collect()
}

/// Converts the given cluster to a swatch.
///
/// # Arguments
/// * `cluster` - The cluster to convert.
/// * `width` - The width of the source image.
/// * `height` - The height of the source image.
///
/// # Returns
/// A swatch representing the given cluster.
#[must_use]
fn convert_to_swatch<F>(
    cluster: &Cluster<F, Point5<F>>,
    width: u32,
    height: u32,
) -> Option<Swatch<Lab<F, D65>>>
where
    F: Float,
{
    let width_f = F::from_u32(width);
    let height_f = F::from_u32(height);
    if cluster.is_empty() {
        return None;
    }

    let centroid = cluster.centroid();
    let color = Lab::<F, D65>::new(
        centroid[0].denormalize(Lab::<F, D65>::min_l(), Lab::<F, D65>::max_l()),
        centroid[1].denormalize(Lab::<F, D65>::min_a(), Lab::<F, D65>::max_a()),
        centroid[2].denormalize(Lab::<F, D65>::min_b(), Lab::<F, D65>::max_b()),
    );

    let x = centroid[3].denormalize(F::zero(), width_f);
    let y = centroid[4].denormalize(F::zero(), height_f);
    let position = (
        x.to_u32().expect("Could not convert x to u32"),
        y.to_u32().expect("Could not convert y to u32"),
    );
    Some(Swatch::new(color, position, cluster.size()))
}
