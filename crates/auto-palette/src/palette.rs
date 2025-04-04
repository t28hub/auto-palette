use std::cmp::Reverse;

use rand_distr::weighted::AliasableWeight;

use crate::{
    algorithm::Algorithm,
    color::{rgb_to_xyz, xyz_to_lab, Color, Lab, D65},
    error::Error,
    image::ImageData,
    math::{
        clustering::{Cluster, ClusteringAlgorithm, DBSCAN},
        denormalize,
        normalize,
        sampling::{DiversitySampling, SamplingAlgorithm, SamplingError, WeightedFarthestSampling},
        DistanceMetric,
        FloatNumber,
        Point,
    },
    theme::Theme,
    Swatch,
};

/// The color palette representation extracted from the image data.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::{color::Color, ImageData, Palette, Swatch, Theme};
///
///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
///     let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
///     assert!(!palette.is_empty());
///     assert!(palette.len() >= 6);
///
///     let mut swatches = palette.find_swatches(3).unwrap();
///
///     assert_eq!(swatches[0].color().to_hex_string(), "#007749");
///     assert_eq!(swatches[1].color().to_hex_string(), "#001489");
///     assert_eq!(swatches[2].color().to_hex_string(), "#E03C31");
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Palette<T>
where
    T: FloatNumber + AliasableWeight,
{
    swatches: Vec<Swatch<T>>,
}

impl<T> Palette<T>
where
    T: FloatNumber + AliasableWeight,
{
    /// Creates a new `Palette` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches of the palette.
    ///
    /// # Returns
    /// A new `Palette` instance.
    #[must_use]
    pub fn new(swatches: Vec<Swatch<T>>) -> Self {
        Self { swatches }
    }

    /// Returns the number of swatches in the palette.
    ///
    /// # Returns
    /// The number of swatches in the palette.
    #[must_use]
    pub fn len(&self) -> usize {
        self.swatches.len()
    }

    /// Returns whether the palette is empty.
    ///
    /// # Returns
    /// `true` if the palette is empty, otherwise `false`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swatches.is_empty()
    }

    /// Returns the swatches in the palette.
    ///
    /// # Returns
    /// The swatches in the palette.
    #[must_use]
    pub fn swatches(&self) -> &[Swatch<T>] {
        &self.swatches
    }

    /// Finds the swatches in the palette based on the theme.
    ///
    /// # Arguments
    /// * `num_swatches` - The number of swatches to find.
    ///
    /// # Returns
    /// The swatches in the palette.
    pub fn find_swatches(&self, num_swatches: usize) -> Result<Vec<Swatch<T>>, Error> {
        self.find_swatches_internal(
            num_swatches,
            |swatch| swatch.ratio(),
            |scores| {
                DiversitySampling::new(T::from_f64(0.6), scores, DistanceMetric::SquaredEuclidean)
            },
        )
    }

    /// Finds the swatches in the palette based on the theme.
    ///
    /// # Arguments
    /// * `num_swatches` - The number of swatches to find.
    /// * `theme` - The theme to use.
    ///
    /// # Returns
    /// The swatches in the palette based on the theme.
    pub fn find_swatches_with_theme(
        &self,
        num_swatches: usize,
        theme: Theme,
    ) -> Result<Vec<Swatch<T>>, Error> {
        self.find_swatches_internal(
            num_swatches,
            |swatch| theme.score(swatch),
            |scores| WeightedFarthestSampling::new(scores, DistanceMetric::SquaredEuclidean),
        )
    }

    /// Finds the swatches in the palette using a custom sampling algorithm.
    ///
    /// # Arguments
    /// * `num_swatches` - The number of swatches to find.
    /// * `score_fn` - The function to score the swatches.
    /// * `sampling_factory` - The function to create the sampling algorithm.
    ///
    /// # Returns
    /// The swatches in the palette. If the palette is empty, an empty vector is returned.
    pub fn find_swatches_internal<S, F1, F2>(
        &self,
        num_swatches: usize,
        score_fn: F1,
        sampling_factory: F2,
    ) -> Result<Vec<Swatch<T>>, Error>
    where
        S: SamplingAlgorithm<T>,
        F1: Fn(&Swatch<T>) -> T,
        F2: FnOnce(Vec<T>) -> Result<S, SamplingError>,
    {
        if self.swatches.is_empty() {
            return Ok(vec![]);
        }

        let (colors, scores): (Vec<Point<T, 3>>, Vec<T>) = self
            .swatches
            .iter()
            .map(|swatch| {
                let color = swatch.color();
                let score = score_fn(swatch);
                ([color.l, color.a, color.b], score)
            })
            .unzip();

        let sampled = sampling_factory(scores)
            .map_err(|cause| Error::SwatchSelectionError { cause })?
            .sample(&colors, num_swatches)
            .map_err(|cause| Error::SwatchSelectionError { cause })?;

        let mut found: Vec<_> = sampled.iter().map(|&index| self.swatches[index]).collect();
        found.sort_by_key(|swatch| Reverse(swatch.population()));
        Ok(found)
    }

    /// Extracts the palette from the image data. The default clustering algorithm is DBSCAN.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, Error> {
        Self::extract_with_algorithm(image_data, Algorithm::default())
    }

    /// Extracts the palette from the image data with the given algorithm.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    /// * `algorithm` - The clustering algorithm to use.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract_with_algorithm(
        image_data: &ImageData,
        algorithm: Algorithm,
    ) -> Result<Self, Error> {
        let pixels = image_data.data();
        if pixels.is_empty() {
            return Err(Error::EmptyImageData);
        }

        let width = image_data.width();
        let height = image_data.height();
        let pixel_clusters = cluster_pixels(width as usize, height as usize, pixels, algorithm);
        let color_clusters = cluster_to_color_groups(&pixel_clusters)?;
        let mut swatches = convert_swatches_from_clusters(
            T::from_u32(width),
            T::from_u32(height),
            &color_clusters,
            &pixel_clusters,
        );
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));
        Ok(Self { swatches })
    }
}

/// Clusters the pixels in the image data using the specified algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `width` - The width of the image data.
/// * `height` - The height of the image data.
/// * `data` - The pixel data of the image data.
/// * `algorithm` - The clustering algorithm to use.
///
/// # Returns
/// The pixel clusters containing the color information and pixel positions.
#[must_use]
fn cluster_pixels<T>(
    width: usize,
    height: usize,
    data: &[u8],
    algorithm: Algorithm,
) -> Vec<Cluster<T, 5>>
where
    T: FloatNumber + AliasableWeight,
{
    let width_f = T::from_usize(width);
    let height_f = T::from_usize(height);
    let points = data
        .chunks(4)
        .enumerate()
        .filter_map(|(index, pixel)| {
            // Ignore transparent pixels.
            if pixel[3] == 0 {
                None
            } else {
                let (x, y, z) = rgb_to_xyz::<T>(pixel[0], pixel[1], pixel[2]);
                let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);
                let x = T::from_usize(index % width);
                let y = T::from_usize(index / width);
                Some([
                    normalize(l, Lab::<T>::min_l(), Lab::<T>::max_l()),
                    normalize(a, Lab::<T>::min_a(), Lab::<T>::max_a()),
                    normalize(b, Lab::<T>::min_b(), Lab::<T>::max_b()),
                    normalize(x, T::zero(), width_f),
                    normalize(y, T::zero(), height_f),
                ])
            }
        })
        .collect::<Vec<_>>();
    algorithm.cluster::<T>(&points)
}

/// Clusters the color groups from the pixel clusters.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `pixel_clusters` - The pixel clusters containing the color information and pixel positions.
///
/// # Returns
/// The color clusters containing the color information.
fn cluster_to_color_groups<T>(pixel_clusters: &[Cluster<T, 5>]) -> Result<Vec<Cluster<T, 3>>, Error>
where
    T: FloatNumber,
{
    let colors = pixel_clusters
        .iter()
        .map(|cluster| -> Point<T, 3> {
            let centroid = cluster.centroid();
            [
                denormalize(centroid[0], Lab::<T>::min_l(), Lab::<T>::max_l()),
                denormalize(centroid[1], Lab::<T>::min_a(), Lab::<T>::max_a()),
                denormalize(centroid[2], Lab::<T>::min_b(), Lab::<T>::max_b()),
            ]
        })
        .collect::<Vec<_>>();
    let algorithm = DBSCAN::new(1, T::from_f32(2.5), DistanceMetric::Euclidean).map_err(|_| {
        Error::PaletteExtractionError {
            details: "Failed to initialize DBSCAN algorithm".to_string(),
        }
    })?;
    let clusters = algorithm.fit(&colors);
    Ok(clusters)
}

/// Converts the color clusters and pixel clusters into swatches.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `width` - The width of the image data.
/// * `height` - The height of the image data.
/// * `color_clusters` - The color clusters containing the color information.
/// * `pixel_clusters` - The pixel clusters containing the color information and pixel positions.
///
/// # Returns
/// The swatches created from the color clusters and pixel clusters.
#[must_use]
fn convert_swatches_from_clusters<T>(
    width: T,
    height: T,
    color_clusters: &[Cluster<T, 3>],
    pixel_clusters: &[Cluster<T, 5>],
) -> Vec<Swatch<T>>
where
    T: FloatNumber,
{
    color_clusters
        .iter()
        .filter_map(|color_cluster| {
            let mut best_color = [T::zero(); 3];
            let mut best_position = None;
            let mut best_population = 0;
            let mut total_population = 0;

            for &member in color_cluster.members() {
                let Some(pixel_cluster) = pixel_clusters.get(member) else {
                    continue;
                };

                if pixel_cluster.is_empty() {
                    continue;
                }

                let fraction = T::from_usize(pixel_cluster.len())
                    / T::from_usize(pixel_cluster.len() + best_population);
                let centroid = pixel_cluster.centroid();
                best_color.iter_mut().enumerate().for_each(|(i, color)| {
                    *color += fraction * (centroid[i] - *color);
                });

                if fraction >= T::from_f32(0.5) || best_population == 0 {
                    best_position = Some((
                        denormalize(centroid[3], T::zero(), width).trunc_to_u32(),
                        denormalize(centroid[4], T::zero(), height).trunc_to_u32(),
                    ));
                    best_population = pixel_cluster.len();
                }
                total_population += pixel_cluster.len();
            }

            if let Some(position) = best_position {
                let l = denormalize(best_color[0], Lab::<T>::min_l(), Lab::<T>::max_l());
                let a = denormalize(best_color[1], Lab::<T>::min_a(), Lab::<T>::max_a());
                let b = denormalize(best_color[2], Lab::<T>::min_b(), Lab::<T>::max_b());
                Some(Swatch::new(
                    Color::new(l, a, b),
                    position,
                    total_population,
                    T::from_usize(total_population) / (width * height),
                ))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_swatches<T>() -> Vec<Swatch<T>>
    where
        T: FloatNumber,
    {
        vec![
            Swatch::new(
                Color::from_str("#FFFFFF").unwrap(),
                (159, 106),
                61228,
                T::from_f64(0.9214),
            ),
            Swatch::new(
                Color::from_str("#EE334E").unwrap(),
                (238, 89),
                1080,
                T::from_f64(0.0163),
            ),
            Swatch::new(
                Color::from_str("#0081C8").unwrap(),
                (82, 88),
                1064,
                T::from_f64(0.0160),
            ),
            Swatch::new(
                Color::from_str("#00A651").unwrap(),
                (197, 123),
                1037,
                T::from_f64(0.0156),
            ),
            Swatch::new(
                Color::from_str("#000000").unwrap(),
                (157, 95),
                1036,
                T::from_f64(0.0156),
            ),
            Swatch::new(
                Color::from_str("#FCB131").unwrap(),
                (119, 123),
                1005,
                T::from_f64(0.0151),
            ),
        ]
    }

    #[must_use]
    fn empty_swatches<T>() -> Vec<Swatch<T>>
    where
        T: FloatNumber,
    {
        vec![]
    }

    #[test]
    fn test_new() {
        // Act
        let swatches = vec![
            Swatch::<f32>::new(Color::from_str("#FFFFFF").unwrap(), (5, 10), 256, 0.5714),
            Swatch::<f32>::new(Color::from_str("#C8102E").unwrap(), (15, 20), 128, 0.2857),
            Swatch::<f32>::new(Color::from_str("#012169").unwrap(), (30, 30), 64, 0.1429),
        ];
        let actual = Palette::new(swatches.clone());

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 3);
        assert_eq!(actual.swatches, swatches);
    }

    #[test]
    fn test_new_empty() {
        // Act
        let swatches = vec![];
        let actual: Palette<f32> = Palette::new(swatches.clone());

        // Assert
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_extract() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f32> = Palette::extract(&image_data).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 3);
    }

    #[cfg(feature = "image")]
    #[rstest]
    #[case::kmeans("kmeans")]
    #[case::dbscan("dbscan")]
    #[case::dbscanpp("dbscan++")]
    fn test_extract_with_algorithm(#[case] name: &str) {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let algorithm = Algorithm::from_str(name).unwrap();
        let actual: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 5);
    }

    #[test]
    fn test_extract_empty_image_data() {
        // Act
        let data = Vec::<u8>::new();
        let image_data = ImageData::new(0, 0, &data).unwrap();
        let result = Palette::<f32>::extract(&image_data);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Image data is empty: no pixels to process"
        );
    }

    #[test]
    fn test_find_swatches() {
        // Arrange
        let swatches = sample_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches(4);

        // Assert
        assert!(actual.is_ok());
        let actual = actual.unwrap();
        assert_eq!(actual.len(), 4);
        assert_eq!(actual[0].color().to_hex_string(), "#FFFFFF");
        assert_eq!(actual[1].color().to_hex_string(), "#EE334E");
        assert_eq!(actual[2].color().to_hex_string(), "#00A651");
        assert_eq!(actual[3].color().to_hex_string(), "#000000");
    }

    #[test]
    fn test_find_swatches_empty() {
        // Arrange
        let swatches = empty_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches(10);

        // Assert
        assert!(actual.is_ok());
        assert!(actual.unwrap().is_empty(), "Expected empty swatches");
    }

    #[rstest]
    #[case::colorful(Theme::Colorful, vec ! ["#EE334E", "#00A651"])]
    #[case::vivid(Theme::Vivid, vec ! ["#EE334E", "#00A651"])]
    #[case::muted(Theme::Muted, vec ! ["#0081C8", "#000000"])]
    #[case::light(Theme::Light, vec ! ["#0081C8", "#00A651"])]
    #[case::dark(Theme::Dark, vec ! ["#0081C8", "#000000"])]
    fn test_find_swatches_with_theme(#[case] theme: Theme, #[case] expected: Vec<&str>) {
        // Arrange
        let swatches = sample_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches_with_theme(2, theme).unwrap();
        actual
            .iter()
            .for_each(|swatch| println!("{:?}", swatch.color().to_hex_string()));

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].color().to_hex_string(), expected[0]);
        assert_eq!(actual[1].color().to_hex_string(), expected[1]);
    }
}
