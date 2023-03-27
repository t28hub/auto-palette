use crate::color::lab::Lab;
use crate::color::rgba::Rgba;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::image::data::ImageData;
use crate::math::clustering::hierarchical::clustering::HierarchicalClustering;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::{Point3, Point5};
use crate::swatch::Swatch;
use crate::Algorithm;
use num_traits::Zero;
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
/// let image_data = SimpleImageData::new(img.as_bytes().to_vec(), img.width(), img.height()).unwrap();
/// let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
/// palette.get_swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch);
/// });
/// ```
pub struct Palette<F: Float> {
    width: F,
    height: F,
    model: Model<F, Point5<F>>,
}

impl<F> Palette<F>
where
    F: Float,
{
    /// Extract a color palette from the given image using the specified algorithm.
    ///
    /// # Arguments
    /// * `image` - The image data to use for color palette extraction.
    /// * `algorithm` - The algorithm to use for color palette extraction.
    ///
    /// # Returns
    /// A new extracted `Palette` instance.
    #[must_use]
    pub fn extract<I: ImageData>(image: &I, algorithm: Algorithm) -> Palette<F> {
        let width = F::from_u32(image.width());
        let height = F::from_u32(image.height());

        let mut pixels = Vec::new();
        for x in 0..image.width() {
            for y in 0..image.height() {
                let [r, g, b, a] = image.get_pixel(x, y).unwrap_or([0, 0, 0, 0]);
                // Ignore a transparent pixel
                if a.is_zero() {
                    continue;
                }

                let rgba = Rgba::new(r, g, b, a);
                let xyz: XYZ<F, D65> = XYZ::from(&rgba);
                let lab: Lab<F, D65> = Lab::from(&xyz);
                pixels.push(Point5::new(
                    lab.l.normalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
                    lab.a.normalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
                    lab.b.normalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
                    F::from_u32(x).normalize(F::zero(), width),
                    F::from_u32(y).normalize(F::zero(), height),
                ));
            }
        }

        let model = algorithm.apply(&pixels);
        Self {
            width,
            height,
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
    pub fn get_swatches(&self, n: usize) -> Vec<Swatch<F>> {
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

        let size = self.width * self.height;
        let mut swatches = HashMap::new();
        for (index, label) in hierarchical_clustering.partition(n).into_iter().enumerate() {
            let mut swatch = swatches.entry(label).or_insert_with(Swatch::default);

            if let Some(cluster) = clusters.get(index) {
                let percentage = F::from_usize(cluster.size()) / size;
                if percentage < swatch.percentage {
                    continue;
                }

                let centroid = cluster.centroid();
                let lab = Lab::new(
                    centroid[0].denormalize(Lab::<F>::min_l(), Lab::<F>::max_l()),
                    centroid[1].denormalize(Lab::<F>::min_a(), Lab::<F>::max_a()),
                    centroid[2].denormalize(Lab::<F>::min_b(), Lab::<F>::max_b()),
                );
                let xyz = XYZ::from(&lab);
                let rgba = Rgba::from(&xyz);
                swatch.color = (rgba.r, rgba.g, rgba.b);

                let x = centroid[3].denormalize(F::zero(), self.width);
                let y = centroid[4].denormalize(F::zero(), self.height);
                swatch.position = (
                    x.to_u32().expect("Could not convert x to u32"),
                    y.to_u32().expect("Could not convert y to u32"),
                );
                swatch.percentage = percentage;
            }
        }
        swatches.into_values().collect()
    }
}
