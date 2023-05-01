use crate::collection::Collection;
use crate::color::lab::Lab;
use crate::color::rgb::Rgb;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::image::image_data::ImageData;
use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point5;
use crate::swatch::Swatch;
use crate::Algorithm;
use num_traits::Zero;

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
/// palette.swatches(5).iter().for_each(|swatch| {
///     println!("{:?}", swatch.color().to_hex_string());
///     println!("{:?}", swatch.position());
///     println!("{:?}", swatch.population());
/// });
/// ```
#[derive(Debug)]
pub struct Palette<F: Float + Default> {
    collection: Collection<Lab<F, D65>>,
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
        let model = algorithm.apply(&pixels);
        let swatches =
            convert_to_swatches(model.clusters(), image_data.width(), image_data.height());
        let collection = Collection::new(swatches);
        Self { collection }
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
        if self.collection.is_empty() {
            return Vec::new();
        }

        if self.collection.len() <= n {
            return self.collection.swatches().to_vec();
        }
        self.collection.find_best_swatches(n)
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

            let x = centroid[3].denormalize(F::zero(), width_f);
            let y = centroid[4].denormalize(F::zero(), height_f);
            let position = (
                x.to_u32().expect("Could not convert x to u32"),
                y.to_u32().expect("Could not convert y to u32"),
            );
            Some(Swatch::new(color, position, cluster.size()))
        })
        .collect()
}
