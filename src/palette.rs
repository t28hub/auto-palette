use crate::color::lab::Lab;
use crate::color::rgb::Rgb;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::color_trait::Color;
use crate::image::image_data::ImageData;
use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::gmeans::algorithm::Gmeans;
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
pub struct Palette<F: Float + Default> {
    candidates: Vec<Swatch<Lab<F, D65>>>,
    groups: HashMap<usize, Vec<usize>>,
}

impl<F> Palette<F>
where
    F: Float + Default,
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
        let gmeans = Gmeans::new(256, 10, 1, F::from_f64(1e-4), Distance::SquaredEuclidean);
        let mut groups = HashMap::new();
        for (i, cluster) in gmeans.train(&colors).clusters().iter().enumerate() {
            if cluster.is_empty() {
                continue;
            }

            let indices = cluster.membership().to_vec();
            groups.insert(i, indices);
        }
        Self { candidates, groups }
    }

    /// Finds the n-dominant swatches in this palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    ///
    /// # Returns
    /// The n-dominant swatches in this palette.
    #[must_use]
    pub fn dominant_swatches(&self, n: usize) -> Vec<Swatch<Lab<F, D65>>> {
        if self.groups.is_empty() {
            return Vec::new();
        }

        let mut swatches: HashMap<_, _> = HashMap::with_capacity(self.groups.len());
        for (label, membership) in self.groups.iter() {
            let Some(merged) = self.merge_to_swatch(membership, |swatch1, swatch2| {
                let population = swatch1.population() + swatch2.population();
                let fraction = F::from_usize(swatch2.population()) / F::from_usize(population);
                let color = swatch1.color().mix(swatch2.color(), fraction);
                let position = if fraction <= F::from_f64(0.5) {
                    swatch1.position()
                } else {
                    swatch2.position()
                };
                Swatch::new(color, position, population)
            }) else {
                continue;
            };
            swatches.insert(label, merged);
        }

        let mut heap = BinaryHeap::new();
        for i in 0..swatches.len() {
            for j in (i + 1)..swatches.len() {
                let swatch_i = &swatches[&i];
                let swatch_j = &swatches[&j];
                let distance = swatch_i.distance(swatch_j);
                heap.push(WeightedEdge::new(i, j, distance));
            }
        }

        while swatches.len() > n {
            let Some(nearest) = heap.pop() else {
                break;
            };

            let Some(swatch_u) = swatches.get(&nearest.u()) else {
                continue;
            };
            let Some(swatch_v) = swatches.get(&nearest.v()) else {
                continue;
            };

            if swatch_u.population() < swatch_v.population() {
                swatches.remove(&nearest.u());
            } else {
                swatches.remove(&nearest.v());
            }
        }

        let mut results: Vec<_> = swatches.into_values().collect();
        results.sort_by_key(|swatch| Reverse(swatch.population()));
        results
    }

    /// Finds the n-dominant colors in this palette using the specified theme.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    /// * `theme` - The theme to use for color palette extraction.
    ///
    /// # Returns
    /// The n-dominant colors in this palette.
    #[must_use]
    pub fn find_with_theme(&self, n: usize, theme: &Theme) -> Vec<Swatch<Lab<F, D65>>> {
        if self.candidates.is_empty() {
            return Vec::new();
        }

        let points: Vec<_> = self
            .candidates
            .iter()
            .map(|swatch| {
                let lab = swatch.color();
                Point3(lab.l, lab.a, lab.b)
            })
            .collect();
        let gmeans = Gmeans::new(256, 10, 1, F::from_f64(1e-4), Distance::SquaredEuclidean);
        let model = gmeans.train(&points);

        let clusters = model.clusters();
        let mut swatches: Vec<_> = clusters
            .iter()
            .filter_map(|cluster| {
                let membership = cluster.membership();
                let first_swatch = membership.first().map(|i| &self.candidates[*i]);
                let mut best_swatch = if let Some(swatch) = first_swatch {
                    swatch.clone()
                } else {
                    return None;
                };

                membership.iter().skip(1).for_each(|i| {
                    let swatch = &self.candidates[*i];
                    if theme.score(swatch) < theme.score(&best_swatch) {
                        return;
                    }

                    best_swatch = {
                        let color = swatch.color().clone();
                        let position = swatch.position();
                        let population = swatch.population() + best_swatch.population();
                        Swatch::new(color, position, population)
                    };
                });
                Some(best_swatch)
            })
            .collect();

        swatches.sort_unstable_by(|swatch1, swatch2| {
            let weight1 = theme.score(swatch1);
            let weight2 = theme.score(swatch2);
            weight1
                .partial_cmp(&weight2)
                .unwrap_or(Ordering::Equal)
                .reverse()
        });
        swatches.iter().take(n).cloned().collect()
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
    F: Float + Default,
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

#[must_use]
fn convert_to_swatch<F>(
    cluster: &Cluster<F, Point5<F>>,
    width: u32,
    height: u32,
) -> Option<Swatch<Lab<F, D65>>>
where
    F: Float + Default,
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
