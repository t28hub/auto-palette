use crate::color::lab::Lab;
use crate::color::rgb::Rgb;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::color_trait::Color;
use crate::delta_e::DeltaE;
use crate::image::image_data::ImageData;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use crate::math::number::Float;
use crate::math::point::Point5;
use crate::named::EXTENDED_COLORS;
use crate::search::ColorSearch;
use crate::swatch::Swatch;
use crate::Algorithm;
use num_traits::Zero;
use std::cmp::Reverse;
use std::collections::HashMap;

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
/// let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();
/// let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
/// palette.swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch);
/// });
/// ```
pub struct Palette<F: Float + Default> {
    swatches: Vec<Swatch<Lab<F, D65>>>,
}

impl<F> Palette<F>
where
    F: Float + Default,
{
    /// Extract a color palette from the given image using the specified algorithm.
    ///
    /// # Arguments
    /// * `image_data` - The image data to use for color palette extraction.
    /// * `algorithm` - The algorithm to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[must_use]
    pub fn extract<I: ImageData>(image_data: &I, algorithm: Algorithm) -> Palette<F> {
        let pixels = convert_to_pixels(image_data);
        let model = algorithm.apply(&pixels);
        let mut swatches =
            convert_to_swatches(model.clusters(), image_data.width(), image_data.height());
        swatches.sort_unstable_by_key(|swatch| Reverse(swatch.population()));
        Self { swatches }
    }

    /// Returns swatches representing the n-dominant colors in the palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to return.
    ///
    /// # Returns
    /// A vector of swatches containing the n-dominant colors.
    #[must_use]
    pub fn swatches(&self, n: usize) -> Vec<Swatch<Lab<F, D65>>> {
        if n >= self.swatches.len() {
            return self.swatches.clone();
        }

        let hierarchical_clustering = HierarchicalClustering::fit(&self.swatches, |u, v| {
            let swatch_u = &self.swatches[u];
            let swatch_v = &self.swatches[v];
            swatch_u.color().delta_e(swatch_v.color(), DeltaE::CIE2000)
        });

        let mut swatches_map = HashMap::new();
        for (index, label) in hierarchical_clustering.partition(n).into_iter().enumerate() {
            let swatch = swatches_map.entry(label).or_insert_with(|| {
                let swatch = &self.swatches[index];
                swatch.clone()
            });
            if self.swatches[index].population() < swatch.population() {
                continue;
            }
            *swatch = self.swatches[index].clone()
        }

        let mut swatches: Vec<_> = swatches_map.into_values().collect();
        swatches.sort_unstable_by_key(|swatch| Reverse(swatch.population()));
        swatches
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
fn convert_to_swatches<F>(
    clusters: &[Cluster<F, Point5<F>>],
    width: u32,
    height: u32,
) -> Vec<Swatch<Lab<F, D65>>>
where
    F: Float + Default,
{
    let width_f = F::from_u32(width);
    let height_f = F::from_u32(height);
    let color_search = ColorSearch::<F>::new(&EXTENDED_COLORS);
    clusters
        .iter()
        .filter_map(|cluster| {
            if cluster.is_empty() {
                return None;
            }

            let centroid = cluster.centroid();
            let color = Lab::<F, D65>::new(
                centroid[0].denormalize(Lab::<F, D65>::min_l(), Lab::<F, D65>::max_l()),
                centroid[1].denormalize(Lab::<F, D65>::min_a(), Lab::<F, D65>::max_a()),
                centroid[2].denormalize(Lab::<F, D65>::min_b(), Lab::<F, D65>::max_b()),
            );
            let Some(named) = color_search.search(&color) else {
            return None;
        };

            let x = centroid[3].denormalize(F::zero(), width_f);
            let y = centroid[4].denormalize(F::zero(), height_f);
            let position = (
                x.to_u32().expect("Could not convert x to u32"),
                y.to_u32().expect("Could not convert y to u32"),
            );
            Some(Swatch::new(named.name(), color, position, cluster.size()))
        })
        .collect()
}
