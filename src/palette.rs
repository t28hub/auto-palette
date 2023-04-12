use crate::color::lab::Lab;
use crate::color::rgb::Rgb;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::image::image_data::ImageData;
use crate::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::{Point3, Point5};
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
    width: F,
    height: F,
    model: Model<F, Point5<F>>,
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
        let width_f = F::from_u32(image_data.width());
        let height_f = F::from_u32(image_data.height());
        let pixels = convert_to_pixels(image_data);
        let model = algorithm.apply(&pixels);
        Self {
            width: width_f,
            height: height_f,
            model,
        }
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
        let clusters = self.model.clusters();
        let centroids: Vec<Point3<F>> = clusters
            .iter()
            .map(|cluster| {
                let centroid = cluster.centroid();
                Point3::new(centroid.0, centroid.1, centroid.2)
            })
            .collect();
        let hierarchical_clustering = HierarchicalClustering::fit(&centroids, |u, v| {
            let point_u = centroids[u];
            let point_v = centroids[v];
            Distance::SquaredEuclidean.measure(&point_u, &point_v)
        });

        let color_search = ColorSearch::<F>::new(&EXTENDED_COLORS);
        let mut swatches_map = HashMap::new();
        for (index, label) in hierarchical_clustering.partition(n).into_iter().enumerate() {
            let swatch = swatches_map.entry(label).or_insert_with(Swatch::default);
            if let Some(cluster) = clusters.get(index) {
                if cluster.size() < swatch.population() {
                    continue;
                }

                let centroid = cluster.centroid();
                let color = Lab::<F, D65>::new(
                    centroid[0].denormalize(Lab::<F, D65>::min_l(), Lab::<F, D65>::max_l()),
                    centroid[1].denormalize(Lab::<F, D65>::min_a(), Lab::<F, D65>::max_a()),
                    centroid[2].denormalize(Lab::<F, D65>::min_b(), Lab::<F, D65>::max_b()),
                );

                let Some(named) = color_search.search(&color) else {
                    continue;
                };

                let x = centroid[3].denormalize(F::zero(), self.width);
                let y = centroid[4].denormalize(F::zero(), self.height);
                let position = (
                    x.to_u32().expect("Could not convert x to u32"),
                    y.to_u32().expect("Could not convert y to u32"),
                );
                *swatch = Swatch::new(named.name(), color, position, cluster.size());
            }
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
