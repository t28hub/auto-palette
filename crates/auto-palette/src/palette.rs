use std::cmp::Reverse;

use rustc_hash::FxHashMap;

use crate::{
    algorithm::Algorithm,
    color::{Color, Lab},
    error::Error,
    image::{
        filter::{AlphaFilter, Filter, FilterChain},
        ImageData,
        RawImage,
    },
    math::{
        clustering::{ClusteringAlgorithm, DBSCAN},
        denormalize,
        sampling::{DiversitySampling, SamplingAlgorithm, SamplingError, WeightedFarthestSampling},
        DistanceMetric,
    },
    segmentation::{segment::SegmentMetadata, SegmentationMethod},
    theme::Theme,
    Swatch,
};

/// The color palette extracted from an image.
///
/// # Examples
/// ```
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::{ImageData, Palette};
///
///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
///     let palette = Palette::extract(&image_data).unwrap();
///     assert!(!palette.is_empty());
///     assert!(palette.len() >= 6);
///
///     let swatches = palette.find_swatches(3);
///     assert_eq!(swatches[0].color().to_hex_string(), "#007749");
///     assert_eq!(swatches[1].color().to_hex_string(), "#E03C31");
///     assert_eq!(swatches[2].color().to_hex_string(), "#001489");
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Palette {
    swatches: Vec<Swatch>,
}

impl Palette {
    /// Creates a new `Palette` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches of the palette.
    ///
    /// # Returns
    /// A new `Palette` instance.
    #[must_use]
    pub fn new(swatches: Vec<Swatch>) -> Self {
        Self { swatches }
    }

    /// Extracts the palette from the image data with the default settings
    /// (DBSCAN segmentation, transparent pixels ignored).
    ///
    /// Use [`Palette::builder`] to customize the extraction.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, Error> {
        Self::builder().build(image_data)
    }

    /// Creates a new [`PaletteBuilder`] for customizing palette extraction.
    ///
    /// # Returns
    /// A new `PaletteBuilder` instance.
    pub fn builder() -> PaletteBuilder {
        PaletteBuilder::new()
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
    pub fn swatches(&self) -> &[Swatch] {
        &self.swatches
    }

    /// Finds up to `count` prominent swatches in the palette.
    ///
    /// The swatches are selected to be both prominent (large population) and
    /// perceptually diverse, and are returned in descending order of
    /// population. If the palette contains fewer than `count` eligible
    /// swatches, fewer swatches are returned.
    ///
    /// # Arguments
    /// * `count` - The maximum number of swatches to find.
    ///
    /// # Returns
    /// The found swatches, sorted by population in descending order.
    #[must_use]
    pub fn find_swatches(&self, count: usize) -> Vec<Swatch> {
        self.select_swatches(
            count,
            |swatch| swatch.ratio(),
            |scores| DiversitySampling::new(0.6, scores, DistanceMetric::SquaredEuclidean),
        )
    }

    /// Finds up to `count` swatches matching the given theme.
    ///
    /// The swatches are scored against the theme, and high-scoring,
    /// perceptually diverse swatches are returned in descending order of
    /// population. If the palette contains fewer than `count` eligible
    /// swatches, fewer swatches are returned.
    ///
    /// # Arguments
    /// * `count` - The maximum number of swatches to find.
    /// * `theme` - The theme to score the swatches against.
    ///
    /// # Returns
    /// The found swatches, sorted by population in descending order.
    #[must_use]
    pub fn find_swatches_with_theme(&self, count: usize, theme: Theme) -> Vec<Swatch> {
        self.select_swatches(
            count,
            |swatch| theme.score(swatch),
            |scores| WeightedFarthestSampling::new(scores, DistanceMetric::SquaredEuclidean),
        )
    }

    /// Selects swatches using a scoring function and a sampling algorithm.
    ///
    /// # Arguments
    /// * `count` - The maximum number of swatches to select.
    /// * `score_fn` - The function to score the swatches.
    /// * `sampling_factory` - The function to create the sampling algorithm.
    ///
    /// # Returns
    /// The selected swatches, sorted by population in descending order.
    fn select_swatches<S, F1, F2>(
        &self,
        count: usize,
        score_fn: F1,
        sampling_factory: F2,
    ) -> Vec<Swatch>
    where
        S: SamplingAlgorithm<f64>,
        F1: Fn(&Swatch) -> f64,
        F2: FnOnce(Vec<f64>) -> Result<S, SamplingError>,
    {
        if self.swatches.is_empty() || count == 0 {
            return Vec::new();
        }

        let mut colors = Vec::with_capacity(self.swatches.len());
        let mut scores = Vec::with_capacity(self.swatches.len());
        let mut indices = Vec::with_capacity(self.swatches.len());
        for (index, swatch) in self.swatches.iter().enumerate() {
            let score = score_fn(swatch);
            if score < SWATCH_SCORE_THRESHOLD {
                // Skip swatches with a score below the threshold
                continue;
            }

            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);
            scores.push(score);
            indices.push(index);
        }

        let count = count.min(indices.len());
        let sampled = sampling_factory(scores.clone()).and_then(|s| s.sample(&colors, count));
        let mut found: Vec<Swatch> = match sampled {
            Ok(sampled) => sampled.iter().map(|&i| self.swatches[indices[i]]).collect(),
            Err(_) => {
                // Sampling cannot fail for non-empty, equally sized inputs;
                // fall back to the highest-scored swatches just in case.
                let mut ranked: Vec<_> = indices.iter().zip(scores.iter()).collect();
                ranked.sort_by(|(_, a), (_, b)| {
                    b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)
                });
                ranked
                    .into_iter()
                    .take(count)
                    .map(|(&index, _)| self.swatches[index])
                    .collect()
            }
        };
        found.sort_by_key(|swatch| Reverse(swatch.population()));
        found
    }
}

/// The builder for customizing palette extraction.
///
/// # Examples
/// ```
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::{Algorithm, ImageData, Palette, Rgba};
///
///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
///     let palette = Palette::builder()
///         .algorithm(Algorithm::KMeans)
///         .filter(|pixel: &Rgba| pixel[3] >= 64)
///         .build(&image_data)
///         .unwrap();
///
///     assert!(!palette.is_empty());
///     assert!(palette.len() >= 6);
/// }
/// ```
#[must_use]
pub struct PaletteBuilder {
    method: SegmentationMethod<f64>,
    filters: FilterChain,
    max_swatches: usize,
    max_pixels: Option<usize>,
}

impl PaletteBuilder {
    /// The default maximum number of swatches to extract.
    const DEFAULT_MAX_SWATCHES: usize = 256;

    /// Creates a new `PaletteBuilder` with the default settings.
    ///
    /// # Returns
    /// A new `PaletteBuilder` instance.
    fn new() -> Self {
        let mut filters = FilterChain::default();
        filters.push(AlphaFilter::default());
        Self {
            method: SegmentationMethod::default(),
            filters,
            max_swatches: Self::DEFAULT_MAX_SWATCHES,
            max_pixels: None,
        }
    }

    /// Sets the clustering algorithm to use.
    ///
    /// # Arguments
    /// * `algorithm` - The clustering algorithm to use.
    ///
    /// # Returns
    /// The `PaletteBuilder` with the algorithm applied.
    pub fn algorithm(mut self, algorithm: Algorithm) -> Self {
        self.method = algorithm.into();
        self
    }

    /// Adds a pixel filter. All filters must accept a pixel for it to
    /// participate in palette extraction. Fully transparent pixels are always
    /// ignored.
    ///
    /// # Arguments
    /// * `filter` - The filter to add. Any `Fn(&Rgba) -> bool` closure works.
    ///
    /// # Returns
    /// The `PaletteBuilder` with the filter added.
    pub fn filter(mut self, filter: impl Filter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Sets the maximum number of swatches to extract.
    ///
    /// # Arguments
    /// * `max_swatches` - The maximum number of swatches to extract.
    ///
    /// # Returns
    /// The `PaletteBuilder` with the maximum swatches applied.
    pub fn max_swatches(mut self, max_swatches: usize) -> Self {
        self.max_swatches = max_swatches;
        self
    }

    /// Limits the number of pixels processed during extraction.
    ///
    /// Images larger than `max_pixels` are downsampled with nearest-neighbor
    /// sampling before segmentation, bounding the running time regardless of
    /// the input resolution. Swatch positions are always reported in the
    /// coordinates of the original image.
    ///
    /// Palette extraction is at best linear in the number of pixels, so this
    /// is the most effective knob for processing large photographs quickly.
    /// A budget in the range of `100_000..=250_000` pixels is usually
    /// indistinguishable from processing the full image.
    ///
    /// # Arguments
    /// * `max_pixels` - The maximum number of pixels to process.
    ///
    /// # Returns
    /// The `PaletteBuilder` with the pixel budget applied.
    pub fn max_pixels(mut self, max_pixels: usize) -> Self {
        self.max_pixels = Some(max_pixels);
        self
    }

    /// Builds the palette from the image data.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The `Palette` instance built from the image data.
    pub fn build(self, image_data: &ImageData) -> Result<Palette, Error> {
        self.build_raw(image_data.as_raw())
    }

    /// Builds the palette directly from a borrowed RGBA pixel buffer.
    ///
    /// This is the zero-copy counterpart of [`build`](Self::build): the
    /// buffer is only borrowed for the duration of the extraction and is
    /// never copied (unless [`max_pixels`](Self::max_pixels) triggers
    /// downsampling, which allocates only the smaller image). Use this when
    /// the pixel data lives in a buffer you cannot or do not want to hand
    /// over to [`ImageData`], e.g. a frame from a camera or a canvas.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `data` - The pixel data in RGBA order (`width * height * 4` bytes).
    ///
    /// # Returns
    /// The `Palette` instance built from the pixel data.
    ///
    /// # Errors
    /// Returns [`Error::InvalidImageData`] if the length of the pixel data is
    /// not equal to `width * height * 4`.
    ///
    /// # Examples
    /// ```
    /// use auto_palette::Palette;
    ///
    /// // A 128x128 image filled with red pixels.
    /// let pixels: Vec<u8> = [255, 0, 0, 255].repeat(128 * 128);
    /// let palette = Palette::builder()
    ///     .build_from_raw(128, 128, &pixels)
    ///     .unwrap();
    /// assert_eq!(palette.len(), 1);
    /// ```
    pub fn build_from_raw(self, width: u32, height: u32, data: &[u8]) -> Result<Palette, Error> {
        self.build_raw(RawImage::new(width, height, data)?)
    }

    /// Builds the palette from a borrowed image view.
    ///
    /// # Arguments
    /// * `raw` - The borrowed image to extract the palette from.
    ///
    /// # Returns
    /// The `Palette` instance built from the image.
    fn build_raw(self, raw: RawImage<'_>) -> Result<Palette, Error> {
        // Downsample the image if a pixel budget is set.
        let downsampled;
        let image = match self.max_pixels {
            Some(max_pixels) if raw.area() > max_pixels => {
                downsampled = raw.downsample(max_pixels);
                downsampled.as_raw()
            }
            _ => raw,
        };

        // Group the points into clusters using the specified algorithm.
        let result = self.method.segment(image, &self.filters)?;

        // Merge similar color clusters and create swatches. Positions are
        // reported in the original image coordinates, while the ratio is
        // relative to the (possibly downsampled) segmented image.
        let mut swatches = to_swatches(
            raw.width(),
            raw.height(),
            result.width() * result.height(),
            result.segments(),
        )?;
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));

        let palette = Palette::new(swatches.into_iter().take(self.max_swatches).collect());
        Ok(palette)
    }
}

impl Default for PaletteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The minimum score threshold for swatches.
/// Swatches with a score below this value are ignored to filter out noise and less significant colors.
const SWATCH_SCORE_THRESHOLD: f64 = 1e-5;

/// The minimum number of points required to merge colors in the LAB color space.
/// This constant is used to determine whether a color cluster has enough points to be considered for merging.
const COLOR_MERGE_MIN_POINTS: usize = 1;

/// The epsilon value for merging colors in the LAB color space.
/// The value of 2.3 is a commonly used threshold for perceptual color difference in LAB space.
/// It is used to determine whether two colors are similar enough to be merged into a single swatch.
/// [Color difference - CIE76](https://en.wikipedia.org/wiki/Color_difference#CIE76)
const COLOR_MERGE_EPSILON_LAB: f64 = 2.3;

/// Converts the segments to swatches.
///
/// # Arguments
/// * `width` - The width of the original image, used for swatch positions.
/// * `height` - The height of the original image, used for swatch positions.
/// * `area` - The number of pixels in the segmented image, used for ratios.
/// * `segments` - The segments to convert.
///
/// # Returns
/// A vector of swatches extracted from the segments.
fn to_swatches(
    width: u32,
    height: u32,
    area: usize,
    segments: &[SegmentMetadata<f64>],
) -> Result<Vec<Swatch>, Error> {
    let (indexed_segments, colors): (Vec<_>, Vec<_>) = segments
        .iter()
        .map(|segment| {
            let center_pixel = segment.center();
            (
                segment,
                [
                    Lab::denormalize_l(center_pixel[0]),
                    Lab::denormalize_a(center_pixel[1]),
                    Lab::denormalize_b(center_pixel[2]),
                ],
            )
        })
        .unzip();

    let dbscan = DBSCAN::new(
        COLOR_MERGE_MIN_POINTS,
        COLOR_MERGE_EPSILON_LAB,
        DistanceMetric::Euclidean,
    )
    .map_err(|e| Error::InvalidParameter {
        reason: e.to_string(),
    })?;

    let labels = dbscan.run(&colors).map_err(|e| Error::InvalidParameter {
        reason: e.to_string(),
    })?;

    let width = f64::from(width);
    let height = f64::from(height);
    let area = area as f64;

    let mut swatches = FxHashMap::default();
    let mut populations = FxHashMap::default();
    for (index, &label) in labels.iter().enumerate() {
        let Some(segment) = indexed_segments.get(index) else {
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
        let fraction = segment.len() as f64 / (segment.len() + swatch.population()) as f64;

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
                    denormalize(center_pixel[3], 0.0, width).trunc() as u32,
                    denormalize(center_pixel[4], 0.0, height).trunc() as u32,
                ),
                total_population,
                total_population as f64 / area,
            );
            *population = segment.len();
        } else {
            *swatch = Swatch::new(
                best_color,
                swatch.position(),
                total_population,
                total_population as f64 / area,
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
    fn sample_swatches() -> Vec<Swatch> {
        vec![
            Swatch::new(
                Color::from_str("#FFFFFF").unwrap(),
                (159, 106),
                61228,
                0.9214,
            ),
            Swatch::new(Color::from_str("#EE334E").unwrap(), (238, 89), 1080, 0.0163),
            Swatch::new(Color::from_str("#0081C8").unwrap(), (82, 88), 1064, 0.0160),
            Swatch::new(
                Color::from_str("#00A651").unwrap(),
                (197, 123),
                1037,
                0.0156,
            ),
            Swatch::new(Color::from_str("#000000").unwrap(), (157, 95), 1036, 0.0156),
            Swatch::new(
                Color::from_str("#FCB131").unwrap(),
                (119, 123),
                1005,
                0.0151,
            ),
        ]
    }

    #[must_use]
    fn collect_sorted_swatches(palette: &Palette) -> Vec<Swatch> {
        let mut swatches = palette.swatches().to_vec();
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
            Swatch::new(Color::from_str("#FFFFFF").unwrap(), (5, 10), 256, 0.5714),
            Swatch::new(Color::from_str("#C8102E").unwrap(), (15, 20), 128, 0.2857),
            Swatch::new(Color::from_str("#012169").unwrap(), (30, 30), 64, 0.1429),
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
        let actual = Palette::new(swatches.clone());

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
        let actual = Palette::builder()
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
        let actual = Palette::builder()
            .filter(|rgba: &Rgba| rgba[3] != 0)
            .build(&image_data)
            .expect("Failed to extract palette with filter");

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 3);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::from_str("#DC143C").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[1].color(),
            Color::from_str("#003893").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[2].color(),
            Color::from_str("#FFFFFF").expect("Invalid color format")
        );
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_builder_max_swatches() {
        // Arrange
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();

        // Act
        let actual = Palette::builder()
            .max_swatches(3)
            .build(&image_data)
            .expect("Failed to extract palette with max swatches");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 3);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::from_str("#FFFFFF").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[1].color(),
            Color::from_str("#0081C8").expect("Invalid color format")
        );
        assert_color_eq!(
            swatches[2].color(),
            Color::from_str("#EE334E").expect("Invalid color format")
        );
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_builder_max_pixels() {
        // Arrange
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();

        // Act
        let actual = Palette::builder()
            .max_pixels(image_data.area() / 4)
            .build(&image_data)
            .expect("Failed to extract palette with max pixels");

        // Assert
        assert!(!actual.is_empty());

        // The dominant colors survive downsampling, and the positions are
        // reported in the original image coordinates.
        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(swatches[0].color(), Color::from_str("#FFFFFF").unwrap());
        for swatch in &swatches {
            let position = swatch.position();
            assert!(position.x < image_data.width());
            assert!(position.y < image_data.height());
            assert!(swatch.ratio() <= 1.0);
        }
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_builder_build_from_raw() {
        // Arrange
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();

        // Act
        let actual = Palette::builder()
            .build_from_raw(image_data.width(), image_data.height(), image_data.data())
            .expect("Failed to extract palette from raw pixel data");

        // Assert
        // Extracting from a borrowed buffer is equivalent to extracting from
        // the owned ImageData.
        let expected = Palette::extract(&image_data).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_builder_build_from_raw_invalid_length() {
        // Act
        let actual = Palette::builder().build_from_raw(2, 2, &[255, 255, 255, 255]);

        // Assert
        assert!(matches!(
            actual.unwrap_err(),
            Error::InvalidImageData {
                expected: 16,
                actual: 4,
            }
        ));
    }

    #[test]
    fn test_builder_empty_image_data() {
        // Act
        let data: Vec<u8> = Vec::new();
        let image_data = ImageData::new(0, 0, data).unwrap();
        let actual = Palette::builder().build(&image_data);

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
        let image_data = ImageData::new(10, 10, data).unwrap();
        let actual = Palette::builder().build(&image_data);

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
        let actual = Palette::extract(&image_data).expect("Failed to extract palette");

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 6);

        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(swatches[0].color(), Color::from_str("#FFFFFF").unwrap());
        assert_color_eq!(swatches[1].color(), Color::from_str("#0081C8").unwrap());
        assert_color_eq!(swatches[2].color(), Color::from_str("#EE334E").unwrap());
        assert_color_eq!(swatches[3].color(), Color::from_str("#000000").unwrap());
        assert_color_eq!(swatches[4].color(), Color::from_str("#00A651").unwrap());
        assert_color_eq!(swatches[5].color(), Color::from_str("#FCB131").unwrap());
    }

    #[test]
    fn test_find_swatches() {
        // Arrange
        let swatches = sample_swatches();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches(4);

        // Assert
        assert_eq!(actual.len(), 4);
        assert_eq!(actual[0].color().to_hex_string(), "#FFFFFF");
        assert_eq!(actual[1].color().to_hex_string(), "#EE334E");
        assert_eq!(actual[2].color().to_hex_string(), "#00A651");
        assert_eq!(actual[3].color().to_hex_string(), "#000000");
    }

    #[test]
    fn test_find_swatches_empty() {
        // Arrange
        let palette = Palette::new(Vec::new());

        // Act
        let actual = palette.find_swatches(10);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_find_swatches_zero_count() {
        // Arrange
        let palette = Palette::new(sample_swatches());

        // Act
        let actual = palette.find_swatches(0);

        // Assert
        assert!(actual.is_empty());
    }

    #[rstest]
    #[case::colorful(Theme::Colorful, vec ! ["#EE334E", "#00A651"])]
    #[case::vivid(Theme::Vivid, vec ! ["#0081C8", "#FCB131"])]
    #[case::muted(Theme::Muted, vec ! ["#00A651", "#000000"])]
    #[case::light(Theme::Light, vec ! ["#00A651", "#FCB131"])]
    #[case::dark(Theme::Dark, vec ! ["#00A651", "#000000"])]
    fn test_find_swatches_with_theme(#[case] theme: Theme, #[case] expected: Vec<&str>) {
        // Arrange
        let swatches = sample_swatches();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches_with_theme(2, theme);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_color_eq!(actual[0].color(), Color::from_str(expected[0]).unwrap());
        assert_color_eq!(actual[1].color(), Color::from_str(expected[1]).unwrap());
    }
}
