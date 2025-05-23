use std::{cmp::Reverse, marker::PhantomData};

use crate::{
    algorithm::Algorithm,
    color::{Color, Lab},
    error::Error,
    image::{
        filter::{AlphaFilter, CompositeFilter, Filter},
        segmentation::Segment,
        ImageData,
    },
    math::{
        clustering::{Cluster, ClusteringAlgorithm, DBSCAN},
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
///     let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
///     assert!(!palette.is_empty());
///     assert!(palette.len() >= 6);
///
///     let mut swatches = palette.find_swatches(3).unwrap();
///
///     assert_eq!(swatches[0].color().to_hex_string(), "#007749");
///     assert_eq!(swatches[1].color().to_hex_string(), "#E03C31");
///     assert_eq!(swatches[2].color().to_hex_string(), "#001489");
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Palette<T>
where
    T: FloatNumber,
{
    swatches: Vec<Swatch<T>>,
}

impl<T> Palette<T>
where
    T: FloatNumber,
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

        let num_swatches = num_swatches.min(self.swatches.len());
        let (colors, scores): (Vec<Point<T, 3>>, Vec<T>) = self
            .swatches
            .iter()
            .map(|swatch| {
                let color = swatch.color();
                ([color.l, color.a, color.b], score_fn(swatch))
            })
            .unzip();

        let sampler =
            sampling_factory(scores).map_err(|cause| Error::SwatchSelectionError { cause })?;
        let sampled = sampler
            .sample(&colors, num_swatches)
            .map_err(|cause| Error::SwatchSelectionError { cause })?;

        let mut found: Vec<_> = sampled.iter().map(|&index| self.swatches[index]).collect();
        found.sort_by_key(|swatch| Reverse(swatch.population()));
        Ok(found)
    }

    /// Creates a new `PaletteBuilder` instance.
    ///
    /// # Returns
    /// A new `PaletteBuilder` instance.
    #[must_use]
    pub fn builder() -> PaletteBuilder<T, AlphaFilter> {
        PaletteBuilder::new()
    }

    /// Extracts the palette from the image data. The default clustering algorithm is DBSCAN.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, Error> {
        Self::builder().build(image_data)
    }

    /// Extracts the palette from the image data with the given algorithm.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    /// * `algorithm` - The clustering algorithm to use.
    ///
    /// # Returns
    /// The extracted palette.
    #[deprecated(
        since = "0.8.0",
        note = "Use `Palette::extract` or `Palette::builder` instead."
    )]
    pub fn extract_with_algorithm(
        image_data: &ImageData,
        algorithm: Algorithm,
    ) -> Result<Self, Error> {
        Self::builder().algorithm(algorithm).build(image_data)
    }
}

/// The builder for creating a `Palette` instance.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `F` - The filter function type.
///
/// # Examples
/// ```
/// use auto_palette::Rgba;
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::{Algorithm, ImageData, Palette, Rgba};
///
///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
///     let palette: Palette<f64> = Palette::builder()
///         .algorithm(Algorithm::KMeans)
///         .filter(|pixel: &Rgba| pixel[3] >= 64)
///         .build(&image_data)
///         .unwrap();
///
///     assert!(!palette.is_empty());
///     assert!(palette.len() >= 6);
/// }
/// ```
pub struct PaletteBuilder<T, F>
where
    T: FloatNumber,
    F: Filter,
{
    algorithm: Algorithm,
    filter: F,
    max_swatches: usize,
    _marker: PhantomData<T>,
}

impl<T> PaletteBuilder<T, AlphaFilter>
where
    T: FloatNumber,
{
    /// The default maximum number of swatches to extract.
    const DEFAULT_MAX_SWATCHES: usize = 256;

    /// Creates a new `PaletteBuilder` instance.
    ///
    /// # Returns
    /// A new `PaletteBuilder` instance.
    #[must_use]
    fn new() -> Self {
        PaletteBuilder {
            algorithm: Algorithm::default(),
            filter: AlphaFilter::default(),
            max_swatches: Self::DEFAULT_MAX_SWATCHES,
            _marker: PhantomData,
        }
    }
}

impl<T, F> PaletteBuilder<T, F>
where
    T: FloatNumber,
    F: Filter,
{
    /// Sets the clustering algorithm to use.
    ///
    /// # Arguments
    /// * `algorithm` - The clustering algorithm to use.
    ///
    /// # Returns
    /// A `PaletteBuilder` instance with the algorithm applied.
    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Sets the filter to use.
    ///
    /// # Type Parameters
    /// * `F2` - The filter type.
    ///
    /// # Arguments
    /// * `filter` - The filter to use.
    ///
    /// # Returns
    /// A `PaletteBuilder` instance with the filter applied.
    #[must_use]
    pub fn filter<F2>(self, filter: F2) -> PaletteBuilder<T, CompositeFilter<F, F2>>
    where
        F2: Filter,
    {
        PaletteBuilder {
            algorithm: self.algorithm,
            filter: self.filter.composite(filter),
            max_swatches: self.max_swatches,
            _marker: PhantomData,
        }
    }

    /// Sets the maximum number of swatches to extract.
    ///
    /// # Arguments
    /// * `max_swatches` - The maximum number of swatches to extract.
    ///
    /// # Returns
    /// A `PaletteBuilder` instance with the maximum swatches applied.
    #[must_use]
    pub fn max_swatches(mut self, max_swatches: usize) -> Self {
        self.max_swatches = max_swatches;
        self
    }

    /// Builds the palette from the image data.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The `Palette` instance built from the image data.
    pub fn build(self, image_data: &ImageData) -> Result<Palette<T>, Error> {
        if image_data.is_empty() {
            return Err(Error::EmptyImageData);
        }

        // Group the points into clusters using the specified algorithm.
        let image_segments = self.algorithm.segment(image_data, &self.filter)?;

        // Merge similar color clusters and create swatches.
        let color_clusters = to_color_group(&image_segments)?;
        let mut swatches =
            convert_swatches_from_segments(image_data, &color_clusters, &image_segments);
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));

        let palette = Palette::new(swatches.into_iter().take(self.max_swatches).collect());
        Ok(palette)
    }
}

const COLOR_GROUP_DBSCAN_MIN_POINTS: usize = 1;
const COLOR_GROUP_DBSCAN_EPSILON: f64 = 2.5;

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
fn to_color_group<T>(image_segments: &[Segment<T>]) -> Result<Vec<Cluster<T, 3>>, Error>
where
    T: FloatNumber,
{
    let colors: Vec<_> = image_segments
        .iter()
        .map(|segment| -> Point<T, 3> {
            let center_pixel = segment.center();
            [
                Lab::<T>::denormalize_l(center_pixel[0]),
                Lab::<T>::denormalize_a(center_pixel[1]),
                Lab::<T>::denormalize_b(center_pixel[2]),
            ]
        })
        .collect();

    let dbscan = DBSCAN::new(
        COLOR_GROUP_DBSCAN_MIN_POINTS,
        T::from_f64(COLOR_GROUP_DBSCAN_EPSILON),
        DistanceMetric::Euclidean,
    )
    .map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;
    dbscan
        .fit(&colors)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
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
fn convert_swatches_from_segments<T>(
    image_data: &ImageData,
    color_clusters: &[Cluster<T, 3>],
    image_segments: &[Segment<T>],
) -> Vec<Swatch<T>>
where
    T: FloatNumber,
{
    let area = T::from_usize(image_data.area());
    color_clusters
        .iter()
        .filter_map(|color_cluster| {
            let mut best_color = [T::zero(); 3];
            let mut best_position = None;
            let mut best_population = 0;
            let mut total_population = 0;

            for &member in color_cluster.members() {
                let Some(segment) = image_segments.get(member) else {
                    continue;
                };

                if segment.is_empty() {
                    continue;
                }

                let fraction =
                    T::from_usize(segment.len()) / T::from_usize(segment.len() + best_population);
                let center_pixel = segment.center();
                best_color.iter_mut().enumerate().for_each(|(i, color)| {
                    *color += fraction * (center_pixel[i] - *color);
                });

                if fraction >= T::from_f64(0.5) || best_population == 0 {
                    best_position = Some((
                        image_data.denormalize_x(center_pixel[3]),
                        image_data.denormalize_y(center_pixel[4]),
                    ));
                    best_population = segment.len();
                }
                total_population += segment.len();
            }

            if let Some(position) = best_position {
                let l = Lab::<T>::denormalize_l(best_color[0]);
                let a = Lab::<T>::denormalize_a(best_color[1]);
                let b = Lab::<T>::denormalize_b(best_color[2]);
                Some(Swatch::new(
                    Color::new(l, a, b),
                    position,
                    total_population,
                    T::from_usize(total_population) / area,
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
    use crate::Rgba;

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
            Swatch::<f64>::new(Color::from_str("#FFFFFF").unwrap(), (5, 10), 256, 0.5714),
            Swatch::<f64>::new(Color::from_str("#C8102E").unwrap(), (15, 20), 128, 0.2857),
            Swatch::<f64>::new(Color::from_str("#012169").unwrap(), (30, 30), 64, 0.1429),
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
        let actual: Palette<f64> = Palette::new(swatches.clone());

        // Assert
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
    }

    #[cfg(feature = "image")]
    #[rstest]
    #[case::kmeans("kmeans")]
    #[case::dbscan("dbscan")]
    #[case::dbscanpp("dbscan++")]
    fn test_builder_with_algorithm(#[case] name: &str) {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let algorithm = Algorithm::from_str(name).unwrap();
        let actual: Palette<f64> = Palette::builder()
            .algorithm(algorithm)
            .build(&image_data)
            .unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 5);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_builder_with_filter() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/np.png").unwrap();
        let actual: Palette<f64> = Palette::builder()
            .filter(|rgba: &Rgba| rgba[3] != 0)
            .build(&image_data)
            .unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 5);
        assert_eq!(actual.swatches[0].color().to_hex_string(), "#DC143C");
        assert_eq!(actual.swatches[1].color().to_hex_string(), "#003893");
        assert_eq!(actual.swatches[2].color().to_hex_string(), "#FFFFFF");
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_builder_max_swatches() {
        // Arrange
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();

        // Act
        let actual: Palette<f64> = Palette::builder()
            .max_swatches(3)
            .build(&image_data)
            .unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_builder_empty_image_data() {
        // Act
        let data: Vec<u8> = Vec::new();
        let image_data = ImageData::new(0, 0, &data).unwrap();
        let result: Result<Palette<f64>, _> = Palette::builder().build(&image_data);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Image data is empty: no pixels to process"
        );
    }

    #[test]
    fn test_extract_transparent_image() {
        // Act
        let data: Vec<u8> = vec![0; 4 * 10 * 10]; // 10x10 transparent image
        let image_data = ImageData::new(10, 10, &data).unwrap();
        let result: Result<Palette<f64>, _> = Palette::builder().build(&image_data);

        // Assert
        assert!(result.is_ok());

        let palette = result.unwrap();
        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_extract() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f64> = Palette::extract(&image_data).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 3);
    }

    #[warn(deprecated)]
    #[test]
    #[cfg(feature = "image")]
    fn test_extract_with_algorithm() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f64> =
            Palette::extract_with_algorithm(&image_data, Algorithm::DBSCANpp).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 6);
    }

    #[test]
    fn test_find_swatches() {
        // Arrange
        let swatches = sample_swatches::<f64>();
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
        let swatches = empty_swatches::<f64>();
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
        let swatches = sample_swatches::<f64>();
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
