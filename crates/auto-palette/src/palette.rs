use std::{cmp::Reverse, collections::HashMap, marker::PhantomData};

use crate::{
    algorithm::Algorithm,
    color::{Color, Lab},
    error::Error,
    image::{
        filter::{AlphaFilter, CompositeFilter, Filter},
        segmentation::LabelImage,
        ImageData,
    },
    math::{
        clustering::{ClusteringAlgorithm, DBSCAN},
        denormalize,
        sampling::{DiversitySampling, SamplingAlgorithm, SamplingError, WeightedFarthestSampling},
        DistanceMetric,
        FloatNumber,
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

        let mut colors = Vec::with_capacity(self.swatches.len());
        let mut scores = Vec::with_capacity(self.swatches.len());
        let mut indices = Vec::with_capacity(self.swatches.len());
        for (index, swatch) in self.swatches.iter().enumerate() {
            let score = score_fn(swatch);
            if score < T::epsilon() {
                // Skip swatches with a score below the threshold
                continue;
            }

            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);
            scores.push(score);
            indices.push(index);
        }

        let num_swatches = num_swatches.min(indices.len());
        let sampling =
            sampling_factory(scores).map_err(|cause| Error::SwatchSelectionError { cause })?;
        let sampled = sampling
            .sample(&colors, num_swatches)
            .map_err(|cause| Error::SwatchSelectionError { cause })?;

        let mut found: Vec<_> = sampled
            .iter()
            .map(|&i| {
                let index = indices[i];
                self.swatches[index]
            })
            .collect();
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
        // Group the points into clusters using the specified algorithm.
        let label_image = self.algorithm.segment(image_data, &self.filter)?;

        // Merge similar color clusters and create swatches.
        let mut swatches = to_swatches(&label_image)?;
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));

        let palette = Palette::new(swatches.into_iter().take(self.max_swatches).collect());
        Ok(palette)
    }
}

/// The minimum number of points required to merge colors in the LAB color space.
/// This constant is used to determine whether a color cluster has enough points to be considered for merging.
const COLOR_MERGE_MIN_POINTS: usize = 1;

/// The epsilon value for merging colors in the LAB color space.
/// The value of 2.3 is a commonly used threshold for perceptual color difference in LAB space.
/// It is used to determine whether two colors are similar enough to be merged into a single swatch.
/// [Color difference - CIE76](https://en.wikipedia.org/wiki/Color_difference#CIE76)
const COLOR_MERGE_EPSILON_LAB: f64 = 2.3;

/// Converts the label image to swatches.
///
/// # Arguments
/// * `label_image` - The label image to convert.
///
/// # Returns
/// A vector of swatches extracted from the label image.
fn to_swatches<T>(label_image: &LabelImage<T>) -> Result<Vec<Swatch<T>>, Error>
where
    T: FloatNumber,
{
    let (segments, colors): (Vec<_>, Vec<_>) = label_image
        .segments()
        .map(|segment| {
            let center_pixel = segment.center();
            (
                segment,
                [
                    Lab::<T>::denormalize_l(center_pixel[0]),
                    Lab::<T>::denormalize_a(center_pixel[1]),
                    Lab::<T>::denormalize_b(center_pixel[2]),
                ],
            )
        })
        .unzip();

    let dbscan = DBSCAN::new(
        COLOR_MERGE_MIN_POINTS,
        T::from_f64(COLOR_MERGE_EPSILON_LAB),
        DistanceMetric::Euclidean,
    )
    .map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;

    let labels = dbscan
        .run(&colors)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    let width = T::from_usize(label_image.width());
    let height = T::from_usize(label_image.height());
    let area = width * height;

    let mut swatches = HashMap::new();
    let mut populations = HashMap::new();
    for (index, &label) in labels.iter().enumerate() {
        let Some(segment) = segments.get(index) else {
            continue;
        };

        let Some(color) = colors.get(index).map(|c| Color::new(c[0], c[1], c[2])) else {
            continue;
        };

        let swatch = swatches.entry(label).or_insert_with(Swatch::default);
        let population = populations.entry(label).or_insert(0usize);

        let center_pixel = segment.center();

        // Calculate the fraction of the segment's population relative to the total population
        // (current swatch population + existing swatch population).
        // The fraction is to determine the weight of the segment's color in the mix.
        let fraction =
            T::from_usize(segment.len()) / T::from_usize(segment.len() + swatch.population());

        // Blend the current swatch color with the segment's color based on the fraction.
        let best_color = swatch.color().mix(&color, fraction);

        // Calculate the total population of the swatch.
        let total_population = swatch.population() + segment.len();

        // Update the swatch if the current segment's population is greater than the existing one.
        // This ensures that the swatch's position and population are updated if the segment is larger.
        if segment.len() > *population {
            *swatch = Swatch::new(
                best_color,
                (
                    denormalize(center_pixel[3], T::zero(), width).trunc_to_u32(),
                    denormalize(center_pixel[4], T::zero(), height).trunc_to_u32(),
                ),
                total_population,
                T::from_usize(total_population) / area,
            );
            *population = segment.len();
        } else {
            *swatch = Swatch::new(
                best_color,
                swatch.position(),
                total_population,
                T::from_usize(total_population) / area,
            );
        };
    }
    Ok(swatches.into_values().collect())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;
    use crate::{assert_color_eq, Rgba};

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
    fn collect_sorted_swatches<T>(palette: &Palette<T>) -> Vec<Swatch<T>>
    where
        T: FloatNumber,
    {
        let mut swatches = palette.swatches().iter().copied().collect::<Vec<_>>();
        swatches.sort_by(|a, b| {
            a.population()
                .cmp(&b.population())
                .reverse()
                .then(a.color().to_rgb_int().cmp(&b.color().to_rgb_int()))
        });
        swatches
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
    #[case::dbscan("dbscan")]
    #[case::dbscanpp("dbscan++")]
    #[case::kmeans("kmeans")]
    #[case::slic("slic")]
    #[case::snic("snic")]
    fn test_builder_with_algorithm(#[case] name: &str) {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let algorithm = Algorithm::from_str(name).unwrap();
        let actual: Palette<f64> = Palette::builder()
            .algorithm(algorithm)
            .build(&image_data)
            .expect("Failed to extract palette with algorithm");

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
            .expect("Failed to extract palette with filter");

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 3);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::<f64>::from_str("#DC143C").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[1].color(),
            Color::<f64>::from_str("#003893").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[2].color(),
            Color::<f64>::from_str("#FFFFFF").expect("Invalid color format")
        );
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
            .expect("Failed to extract palette with max swatches");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 3);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::<f64>::from_str("#FFFFFF").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[1].color(),
            Color::<f64>::from_str("#0081C8").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[2].color(),
            Color::<f64>::from_str("#EE334E").expect("Invalid color format")
        );
    }

    #[test]
    fn test_builder_empty_image_data() {
        // Act
        let data: Vec<u8> = Vec::new();
        let image_data = ImageData::new(0, 0, &data).unwrap();
        let actual = Palette::<f64>::builder().build(&image_data);

        // Assert
        assert!(actual.is_ok());

        let palette = actual.expect("Failed to extract palette from empty image data");
        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }

    #[test]
    fn test_extract_transparent_image() {
        // Act
        let data: Vec<u8> = vec![0; 4 * 10 * 10]; // 10x10 transparent image
        let image_data = ImageData::new(10, 10, &data).unwrap();
        let actual: Result<Palette<f64>, _> = Palette::builder().build(&image_data);

        // Assert
        assert!(actual.is_ok());

        let palette = actual.expect("Failed to extract palette from transparent image");
        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_extract() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f64> =
            Palette::extract(&image_data).expect("Failed to extract palette");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 6);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::<f64>::from_str("#FFFFFF").unwrap()
        );
        assert_color_eq!(
            swatches[1].color(),
            Color::<f64>::from_str("#0081C8").unwrap()
        );
        assert_color_eq!(
            swatches[2].color(),
            Color::<f64>::from_str("#EE334E").unwrap()
        );
        assert_color_eq!(
            swatches[3].color(),
            Color::<f64>::from_str("#000000").unwrap()
        );
        assert_color_eq!(
            swatches[4].color(),
            Color::<f64>::from_str("#00A651").unwrap()
        );
        assert_color_eq!(
            swatches[5].color(),
            Color::<f64>::from_str("#FCB131").unwrap()
        );
    }

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

        let swatches = actual.expect("Failed to find swatches");
        assert_eq!(swatches.len(), 4);
        assert_eq!(swatches[0].color().to_hex_string(), "#FFFFFF");
        assert_eq!(swatches[1].color().to_hex_string(), "#EE334E");
        assert_eq!(swatches[2].color().to_hex_string(), "#00A651");
        assert_eq!(swatches[3].color().to_hex_string(), "#000000");
    }

    #[test]
    fn test_find_swatches_empty() {
        // Arrange
        let swatches: Vec<Swatch<f64>> = Vec::new();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches(10);

        // Assert
        assert!(actual.is_ok());

        let swatches = actual.expect("Failed to find swatches in empty palette");
        assert!(swatches.is_empty());
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
        let actual = palette
            .find_swatches_with_theme(2, theme)
            .expect("Failed to find swatches with theme");
        actual
            .iter()
            .for_each(|swatch| println!("{:?}", swatch.color().to_hex_string()));

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].color().to_hex_string(), expected[0]);
        assert_eq!(actual[1].color().to_hex_string(), expected[1]);
    }
}
