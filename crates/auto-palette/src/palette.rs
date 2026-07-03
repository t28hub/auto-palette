use std::cmp::Reverse;

use rustc_hash::FxHashMap;

use crate::{
    color::{Color, Lab},
    error::{Error, ExtractionError, ExtractionErrorKind, SelectionError},
    image::{
        filter::{AlphaFilter, CompositeFilter, Filter},
        ImageData,
    },
    math::{
        clustering::{ClusteringAlgorithm, DBSCAN},
        denormalize,
        sampling::{DiversitySampling, SamplingAlgorithm, SamplingError},
        DistanceMetric,
        FloatNumber,
    },
    segmentation::{segment::SegmentMetadata, SegmentationMethod},
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
///     let swatches = palette.find_swatches(3).unwrap();
///     assert_eq!(swatches.len(), 3);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Creates a query for selecting swatches from this palette.
    ///
    /// The query starts from sensible defaults (population-ratio scoring,
    /// diversity-based selection, all swatches) and can be refined with
    /// chained method calls before running it with
    /// [`execute`](SwatchQuery::execute).
    ///
    /// # Returns
    /// A new [`SwatchQuery`] borrowing this palette.
    ///
    /// # Examples
    /// ```
    /// #[cfg(feature = "image")]
    /// {
    ///     use auto_palette::{ImageData, Palette, Theme};
    ///
    ///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
    ///     let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
    ///
    ///     let swatches = palette
    ///         .query()
    ///         .count(3)
    ///         .theme(Theme::Vivid)
    ///         .execute()
    ///         .unwrap();
    ///     assert!(swatches.len() <= 3);
    /// }
    /// ```
    #[must_use]
    pub fn query(&self) -> SwatchQuery<'_, T> {
        SwatchQuery::new(self)
    }

    /// Finds the best swatches in the palette.
    ///
    /// This is a shorthand for `self.query().count(num_swatches).execute()`.
    ///
    /// # Arguments
    /// * `num_swatches` - The number of swatches to find.
    ///
    /// # Returns
    /// The swatches in the palette.
    pub fn find_swatches(&self, num_swatches: usize) -> Result<Vec<Swatch<T>>, Error> {
        self.query().count(num_swatches).execute()
    }

    /// Finds the best swatches in the palette based on the theme.
    ///
    /// This is a shorthand for
    /// `self.query().count(num_swatches).theme(theme).execute()`.
    ///
    /// # Arguments
    /// * `num_swatches` - The maximum number of swatches to find.
    /// * `theme` - The theme to use.
    ///
    /// # Returns
    /// The swatches in the palette based on the theme. Fewer swatches than
    /// requested may be returned when the palette has few colors matching
    /// the theme.
    pub fn find_swatches_with_theme(
        &self,
        num_swatches: usize,
        theme: Theme,
    ) -> Result<Vec<Swatch<T>>, Error> {
        self.query().count(num_swatches).theme(theme).execute()
    }

    /// Finds the swatches in the palette using a custom sampling algorithm.
    ///
    /// # Arguments
    /// * `num_swatches` - The number of swatches to find.
    /// * `score_fn` - The function to score the swatches.
    /// * `min_score_ratio` - When set, swatches scoring below this fraction of
    ///   the best score are excluded before sampling.
    /// * `sampling_factory` - The function to create the sampling algorithm.
    ///
    /// # Returns
    /// The swatches in the palette. If the palette is empty, an empty vector is returned.
    fn find_swatches_internal<S, F1, F2>(
        &self,
        num_swatches: usize,
        score_fn: F1,
        min_score_ratio: Option<T>,
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
            if score < T::from_f64(SWATCH_SCORE_THRESHOLD) {
                // Skip swatches with a score below the threshold
                continue;
            }

            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);
            scores.push(score);
            indices.push(index);
        }

        // Filter out swatches scoring well below the best candidate, so that
        // the sampling below chooses for diversity among qualified candidates
        // instead of trading score fidelity against visual variety.
        if let Some(ratio) = min_score_ratio {
            let max_score = scores.iter().copied().fold(T::zero(), T::max);
            let cutoff = max_score * ratio;
            let mut kept = 0;
            for i in 0..scores.len() {
                if scores[i] >= cutoff {
                    colors.swap(kept, i);
                    scores.swap(kept, i);
                    indices.swap(kept, i);
                    kept += 1;
                }
            }
            colors.truncate(kept);
            scores.truncate(kept);
            indices.truncate(kept);
        }

        let num_swatches = num_swatches.min(indices.len());
        let sampling = sampling_factory(scores).map_err(SelectionError::from)?;
        let sampled = sampling
            .sample(&colors, num_swatches)
            .map_err(SelectionError::from)?;

        // Sort the selected indices first: `sampled` is a `HashSet` whose
        // iteration order varies between runs, and the stable population sort
        // below preserves that order for equal populations. Sorting makes the
        // returned swatch order deterministic.
        let mut selected: Vec<_> = sampled.iter().map(|&i| indices[i]).collect();
        selected.sort_unstable();

        let mut found: Vec<_> = selected
            .into_iter()
            .map(|index| self.swatches[index])
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
}

impl<'a, T> IntoIterator for &'a Palette<T>
where
    T: FloatNumber,
{
    type IntoIter = std::slice::Iter<'a, Swatch<T>>;
    type Item = &'a Swatch<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.swatches.iter()
    }
}

/// The default diversity factor for population-ratio scoring.
const DEFAULT_DIVERSITY_FACTOR: f64 = 0.6;

/// The fraction of the best theme score below which candidates are excluded.
///
/// Themed selection first drops swatches scoring well below the best
/// candidate, then samples for diversity among the remaining qualified
/// candidates. Without this cutoff the selection has to trade score fidelity
/// against visual variety, which tends to pick several close shades of the
/// best-scoring hue.
const THEME_CANDIDATE_SCORE_RATIO: f64 = 0.1;

/// The default diversity factor for themed scoring.
///
/// Theme fit is already guaranteed by [`THEME_CANDIDATE_SCORE_RATIO`], so the
/// selection can lean towards visual variety among the qualified candidates.
const DEFAULT_THEME_DIVERSITY_FACTOR: f64 = 0.85;

/// The default diversity factor for custom scoring.
///
/// Custom scores have no candidate cutoff, so the selection stays closer to
/// score fidelity.
const DEFAULT_SCORED_DIVERSITY_FACTOR: f64 = 0.3;

/// The exponent applied to the normalized population when scoring swatches
/// against a theme.
///
/// The theme score describes only how well a color matches the theme, so a
/// few noise pixels with a perfectly matching color would otherwise win over
/// a prominent, slightly-less-matching color. Weighting by
/// `(population / max_population) ^ EXPONENT` discounts rare colors while the
/// sub-linear exponent keeps mid-sized swatches competitive against the
/// dominant one.
const THEME_POPULATION_EXPONENT: f64 = 0.25;

/// The source of the score assigned to each swatch during selection.
enum ScoreSource<'a, T>
where
    T: FloatNumber,
{
    /// Score each swatch by its population ratio.
    Ratio,

    /// Score each swatch by the given theme.
    Theme(Theme),

    /// Score each swatch with a caller-provided function.
    Custom(Box<dyn Fn(&Swatch<T>) -> T + 'a>),
}

/// A query for selecting swatches from a [`Palette`].
///
/// Created by [`Palette::query`]. The query collects the selection criteria
/// through chained method calls and runs when [`execute`](Self::execute) is
/// called; validation errors are reported by `execute`.
///
/// # Defaults
/// * All swatches are candidates (no count limit).
/// * Swatches are scored by their population ratio; [`theme`](Self::theme) or
///   [`score_with`](Self::score_with) replaces the scoring (the last call
///   wins). Theme scores are additionally weighted by each swatch's
///   population relative to the dominant swatch, so rare noise colors do not
///   outrank prominent colors.
/// * Themed selection first excludes candidates scoring below 10% of the
///   best score, then selects for visual variety among the qualified
///   candidates (trade-off factor `0.85`). Because of this cutoff, fewer
///   swatches than requested may be returned when the palette has few colors
///   matching the theme.
/// * Population-ratio scoring uses a balanced trade-off factor of `0.6`;
///   custom scoring stays close to score fidelity (`0.3`) and applies no
///   cutoff. Set the factor explicitly with [`diversity`](Self::diversity).
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::{ImageData, Palette};
///
///     let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
///     let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
///
///     // Select up to 4 swatches, preferring light colors.
///     let swatches = palette
///         .query()
///         .count(4)
///         .score_with(|swatch| swatch.color().lightness() / 100.0)
///         .execute()
///         .unwrap();
///     assert!(swatches.len() <= 4);
/// }
/// ```
#[must_use]
pub struct SwatchQuery<'a, T>
where
    T: FloatNumber,
{
    palette: &'a Palette<T>,
    count: Option<usize>,
    score: ScoreSource<'a, T>,
    diversity: Option<T>,
}

impl<'a, T> SwatchQuery<'a, T>
where
    T: FloatNumber,
{
    /// Creates a new `SwatchQuery` instance for the given palette.
    fn new(palette: &'a Palette<T>) -> Self {
        Self {
            palette,
            count: None,
            score: ScoreSource::Ratio,
            diversity: None,
        }
    }

    /// Sets the maximum number of swatches to select.
    ///
    /// When not set, all swatches that pass the scoring are candidates.
    ///
    /// # Arguments
    /// * `count` - The maximum number of swatches to select.
    pub fn count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    /// Scores the swatches by the given theme.
    ///
    /// Replaces any previously set scoring (the last call to `theme` or
    /// [`score_with`](Self::score_with) wins).
    ///
    /// # Arguments
    /// * `theme` - The theme to score the swatches by.
    pub fn theme(mut self, theme: Theme) -> Self {
        self.score = ScoreSource::Theme(theme);
        self
    }

    /// Scores the swatches with a caller-provided function.
    ///
    /// Higher scores are preferred. Swatches scoring below `1e-5` are
    /// excluded from the selection. Replaces any previously set scoring
    /// (the last call to [`theme`](Self::theme) or `score_with` wins).
    ///
    /// # Arguments
    /// * `score_fn` - The function assigning a score to each swatch.
    pub fn score_with<F>(mut self, score_fn: F) -> Self
    where
        F: Fn(&Swatch<T>) -> T + 'a,
    {
        self.score = ScoreSource::Custom(Box::new(score_fn));
        self
    }

    /// Selects the swatches by balancing score against color diversity.
    ///
    /// # Arguments
    /// * `factor` - The trade-off factor in `[0, 1]`: `0` selects purely by
    ///   score, `1` purely by color diversity. Values outside the range are
    ///   reported as an error by [`execute`](Self::execute).
    pub fn diversity(mut self, factor: T) -> Self {
        self.diversity = Some(factor);
        self
    }

    /// Runs the query and returns the selected swatches.
    ///
    /// The swatches are sorted by population in descending order. If the
    /// palette is empty, an empty vector is returned.
    ///
    /// # Returns
    /// The selected swatches, or an error if the query is invalid (e.g., a
    /// diversity factor outside `[0, 1]`).
    pub fn execute(self) -> Result<Vec<Swatch<T>>, Error> {
        let count = self.count.unwrap_or_else(|| self.palette.len());
        let (default_factor, min_score_ratio) = match self.score {
            ScoreSource::Ratio => (DEFAULT_DIVERSITY_FACTOR, None),
            // Theme fit is guaranteed by the candidate cutoff, so the
            // selection can lean towards visual variety.
            ScoreSource::Theme(_) => (
                DEFAULT_THEME_DIVERSITY_FACTOR,
                Some(T::from_f64(THEME_CANDIDATE_SCORE_RATIO)),
            ),
            ScoreSource::Custom(_) => (DEFAULT_SCORED_DIVERSITY_FACTOR, None),
        };
        let score_fn: Box<dyn Fn(&Swatch<T>) -> T + 'a> = match self.score {
            ScoreSource::Ratio => Box::new(|swatch: &Swatch<T>| swatch.ratio()),
            ScoreSource::Theme(theme) => {
                // Discount rare colors: the theme score alone would let a few
                // perfectly-matching noise pixels win over prominent colors.
                let max_population = self
                    .palette
                    .swatches
                    .iter()
                    .map(Swatch::population)
                    .max()
                    .unwrap_or(1)
                    .max(1);
                Box::new(move |swatch: &Swatch<T>| {
                    let prominence = (T::from_usize(swatch.population())
                        / T::from_usize(max_population))
                    .powf(T::from_f64(THEME_POPULATION_EXPONENT));
                    theme.score(swatch) * prominence
                })
            }
            ScoreSource::Custom(score_fn) => score_fn,
        };

        // All score sources select with diversity sampling; only the default
        // trade-off factor and the candidate cutoff differ.
        let factor = self
            .diversity
            .unwrap_or_else(|| T::from_f64(default_factor));
        self.palette
            .find_swatches_internal(count, score_fn, min_score_ratio, |scores| {
                DiversitySampling::new(factor, scores, DistanceMetric::SquaredEuclidean)
            })
    }
}

impl<T> std::fmt::Debug for SwatchQuery<'_, T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let score = match &self.score {
            ScoreSource::Ratio => "Ratio".to_string(),
            ScoreSource::Theme(theme) => format!("Theme({theme:?})"),
            ScoreSource::Custom(_) => "Custom".to_string(),
        };
        let diversity = self.diversity.map(|factor| factor.to_string());
        f.debug_struct("SwatchQuery")
            .field("count", &self.count)
            .field("score", &score)
            .field("diversity", &diversity)
            .finish_non_exhaustive()
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
    algorithm: SegmentationMethod<T>,
    filter: F,
    max_swatches: usize,
    max_pixels: usize,
}

impl<T> PaletteBuilder<T, AlphaFilter>
where
    T: FloatNumber,
{
    /// The default maximum number of swatches to extract.
    const DEFAULT_MAX_SWATCHES: usize = 256;

    /// The default maximum number of pixels to process (256x256).
    ///
    /// Larger images are downsampled before extraction. This bounds the
    /// extraction time regardless of the input resolution while retaining
    /// enough pixels for accurate palettes.
    const DEFAULT_MAX_PIXELS: usize = 65_536;

    /// Creates a new `PaletteBuilder` instance.
    ///
    /// # Returns
    /// A new `PaletteBuilder` instance.
    #[must_use]
    fn new() -> Self {
        PaletteBuilder {
            algorithm: SegmentationMethod::default(),
            filter: AlphaFilter::default(),
            max_swatches: Self::DEFAULT_MAX_SWATCHES,
            max_pixels: Self::DEFAULT_MAX_PIXELS,
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
    #[must_use]
    pub fn algorithm(mut self, algorithm: impl Into<SegmentationMethod<T>>) -> Self {
        self.algorithm = algorithm.into();
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
            max_pixels: self.max_pixels,
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

    /// Sets the maximum number of pixels to process.
    ///
    /// Images with more pixels are downsampled (preserving the aspect ratio)
    /// before extraction, which bounds the extraction time regardless of the
    /// input resolution. Swatch positions are reported in the coordinate
    /// space of the original image.
    ///
    /// Defaults to 65,536 pixels (256x256). Pass `usize::MAX` to process
    /// every pixel of the original image.
    ///
    /// # Arguments
    /// * `max_pixels` - The maximum number of pixels to process.
    ///
    /// # Returns
    /// A `PaletteBuilder` instance with the maximum pixels applied.
    #[must_use]
    pub fn max_pixels(mut self, max_pixels: usize) -> Self {
        self.max_pixels = max_pixels;
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
        if image_data.area() <= self.max_pixels {
            return self.build_from(image_data);
        }

        // Downsample large images to bound the extraction time, then map the
        // swatch positions back to the coordinate space of the original image.
        let downsampled = image_data.downsample(self.max_pixels);
        let scale_x = T::from_u32(image_data.width()) / T::from_u32(downsampled.width());
        let scale_y = T::from_u32(image_data.height()) / T::from_u32(downsampled.height());
        let max_x = image_data.width().saturating_sub(1);
        let max_y = image_data.height().saturating_sub(1);

        let palette = self.build_from(&downsampled)?;
        let swatches = palette
            .swatches
            .into_iter()
            .map(|swatch| {
                let (x, y) = swatch.position();
                let x = (T::from_u32(x) * scale_x).round().trunc_to_u32().min(max_x);
                let y = (T::from_u32(y) * scale_y).round().trunc_to_u32().min(max_y);
                Swatch::new(*swatch.color(), (x, y), swatch.population(), swatch.ratio())
            })
            .collect();
        Ok(Palette::new(swatches))
    }

    /// Builds the palette from the image data without downsampling.
    fn build_from(self, image_data: &ImageData) -> Result<Palette<T>, Error> {
        // Group the points into clusters using the specified algorithm.
        let result = self.algorithm.segment(image_data, &self.filter)?;

        // Merge similar color clusters and create swatches.
        let mut swatches = to_swatches(result.width(), result.height(), result.segments())?;
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));

        let palette = Palette::new(swatches.into_iter().take(self.max_swatches).collect());
        Ok(palette)
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
/// * `width` - The width of the image.
/// * `height` - The height of the image.
/// * `segments` - The segments to convert.
///
/// # Returns
/// A vector of swatches extracted from the segments.
fn to_swatches<T>(
    width: usize,
    height: usize,
    segments: &[SegmentMetadata<T>],
) -> Result<Vec<Swatch<T>>, Error>
where
    T: FloatNumber,
{
    let (indexed_segments, colors): (Vec<_>, Vec<_>) = segments
        .iter()
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
    .map_err(|_| ExtractionError::from(ExtractionErrorKind::InvalidParameter))?;

    let labels = dbscan
        .run(&colors)
        .map_err(|_| ExtractionError::from(ExtractionErrorKind::InvalidParameter))?;

    let width = T::from_usize(width);
    let height = T::from_usize(height);
    let area = width * height;

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
    use crate::{assert_color_eq, Algorithm, Rgba};

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

        // The most dominant color (the white background) is always first;
        // the following swatches are the largest of the five ring colors.
        let swatches = collect_sorted_swatches(&actual);
        assert_color_eq!(
            swatches[0].color(),
            Color::<f64>::from_str("#FFFFFF").expect("Invalid color format")
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

        // Every color of the Olympic logo is present, in any order.
        let expected = [
            "#FFFFFF", "#0081C8", "#EE334E", "#000000", "#00A651", "#FCB131",
        ];
        for hex in expected {
            let expected_color = Color::<f64>::from_str(hex).unwrap();
            let matched = actual
                .swatches()
                .iter()
                .any(|swatch| swatch.color().delta_e(&expected_color) < 5.0);
            assert!(matched, "expected a color close to {hex} in the palette");
        }
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_builder_with_dbscanpp() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f64> = Palette::builder()
            .algorithm(Algorithm::DBSCANpp)
            .build(&image_data)
            .unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 6);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_round_trip() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let serialized = serde_json::to_string(&palette).unwrap();
        let deserialized: Palette<f64> = serde_json::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(deserialized, palette);
    }

    #[test]
    fn test_query_defaults_match_find_swatches() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let actual = palette.query().count(4).execute().unwrap();
        let expected = palette.find_swatches(4).unwrap();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_query_theme_matches_find_swatches_with_theme() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let actual = palette
            .query()
            .count(3)
            .theme(Theme::Vivid)
            .execute()
            .unwrap();
        let expected = palette.find_swatches_with_theme(3, Theme::Vivid).unwrap();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_query_without_count_selects_all() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let actual = palette.query().execute().unwrap();

        // Assert
        assert_eq!(actual.len(), palette.len());
    }

    #[test]
    fn test_query_score_with() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act: score by lightness; the single selected swatch is the lightest.
        let actual = palette
            .query()
            .count(1)
            .score_with(|swatch| swatch.color().lightness() / 100.0)
            .execute()
            .unwrap();

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].color().to_hex_string(), "#FFFFFF");
    }

    #[test]
    fn test_query_score_last_call_wins() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act: `score_with` after `theme` replaces the theme scoring.
        let actual = palette
            .query()
            .count(1)
            .theme(Theme::Dark)
            .score_with(|swatch| swatch.color().lightness() / 100.0)
            .execute()
            .unwrap();

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].color().to_hex_string(), "#FFFFFF");
    }

    #[test]
    fn test_query_invalid_diversity() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let actual = palette.query().count(3).diversity(1.5).execute();

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_query_empty_palette() {
        // Arrange
        let palette = Palette::new(Vec::<Swatch<f64>>::new());

        // Act
        let actual = palette.query().count(3).execute().unwrap();

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_query_debug() {
        // Arrange
        let palette = Palette::new(sample_swatches::<f64>());

        // Act
        let actual = format!("{:?}", palette.query().count(3).theme(Theme::Vivid));

        // Assert
        assert!(actual.contains("count: Some(3)"));
        assert!(actual.contains("Theme(Vivid)"));
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
    #[case::vivid(Theme::Vivid, vec ! ["#0081C8", "#FCB131"])]
    // Only black passes the muted/dark candidate cutoff on this fixture, so
    // fewer swatches than requested are returned.
    #[case::muted(Theme::Muted, vec ! ["#000000"])]
    #[case::light(Theme::Light, vec ! ["#EE334E", "#00A651"])]
    #[case::dark(Theme::Dark, vec ! ["#000000"])]
    fn test_find_swatches_with_theme(#[case] theme: Theme, #[case] expected: Vec<&str>) {
        // Arrange
        let swatches = sample_swatches::<f64>();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette
            .find_swatches_with_theme(2, theme)
            .expect("Failed to find swatches with theme");

        // Assert
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert_color_eq!(actual.color(), Color::<f64>::from_str(expected).unwrap());
        }
    }
}
