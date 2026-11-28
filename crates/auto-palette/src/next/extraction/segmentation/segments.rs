use crate::next::{
    color::{AlphaColor, Color},
    extraction::{centroid::Centroid, estimation::Estimation, point::Point, position::Position},
    image::{ImageBuf, Size},
};

/// The number of a segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SegmentId(u32);

impl SegmentId {
    /// Creates a segment number.
    ///
    /// # Arguments
    /// * `index` - The number.
    ///
    /// # Returns
    /// The segment number.
    #[inline]
    pub(crate) const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the number as an index.
    ///
    /// # Returns
    /// The number, counted from 0.
    #[inline]
    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The color, center, weight, and radial distance of one segment.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Segment {
    /// The color estimated from the pixels.
    color: Color,
    /// The mean position of the pixels, weighted by their opacity.
    center: Position,
    /// The sum of the opacities of the pixels.
    weight: f64,
    /// The mean distance of the pixels from the image center, weighted by their opacity.
    radial_distance: f64,
}

#[cfg(test)]
impl Segment {
    /// Creates a segment from its statistics.
    ///
    /// # Arguments
    /// * `color` - The color.
    /// * `center` - The center.
    /// * `weight` - The weight.
    /// * `radial_distance` - The mean distance of the pixels from the image center.
    ///
    /// # Returns
    /// The segment with the given statistics.
    pub(crate) const fn new(
        color: Color,
        center: Position,
        weight: f64,
        radial_distance: f64,
    ) -> Self {
        Self {
            color,
            center,
            weight,
            radial_distance,
        }
    }
}

impl Segment {
    /// Collects the statistics of the pixels of one segment.
    ///
    /// # Arguments
    /// * `image` - The image that was segmented.
    /// * `pixels` - The indices of the pixels of the segment.
    /// * `estimation` - The method that chooses the color from the pixels.
    ///
    /// # Returns
    /// The segment, or `None` if its pixels have no weight.
    fn of_pixels(
        image: &ImageBuf<AlphaColor<Color>>,
        pixels: &[usize],
        estimation: Estimation,
    ) -> Option<Self> {
        let mut centroid = Centroid::default();
        let mut weighted_sum = 0.0;
        let mut colors = Vec::with_capacity(pixels.len());
        for &index in pixels {
            let point = Point::at(image, index);
            centroid.add(point);
            weighted_sum += point.weight() * measure_radial_distance(index, image.size());
            colors.push(image.pixels()[index]);
        }
        let mean = centroid.point()?;
        let color = estimation
            .estimate(&colors)
            .expect("a segment with weight has pixels");
        Some(Self {
            color,
            center: mean.position(),
            weight: mean.weight(),
            radial_distance: weighted_sum / mean.weight(),
        })
    }
}

impl Segment {
    /// Returns the color estimated from the pixels.
    ///
    /// # Returns
    /// The color of the segment.
    #[inline]
    pub(crate) const fn color(self) -> Color {
        self.color
    }

    /// Returns the mean position of the pixels, weighted by their opacity.
    ///
    /// # Returns
    /// The center of the segment.
    #[inline]
    pub(crate) const fn center(self) -> Position {
        self.center
    }

    /// Returns the sum of the opacities of the pixels.
    ///
    /// # Returns
    /// The weight of the segment.
    #[inline]
    pub(crate) const fn weight(self) -> f64 {
        self.weight
    }

    /// Returns the mean distance of the pixels from the image center, weighted by their opacity.
    ///
    /// # Returns
    /// The distance, with each axis of the image spanning 0.0 to 1.0.
    #[inline]
    pub(crate) const fn radial_distance(self) -> f64 {
        self.radial_distance
    }
}

/// The segments of an image.
#[derive(Debug, Default)]
pub(crate) struct Segments {
    /// The segments, in the order of their numbers.
    segments: Vec<Segment>,
}

impl Segments {
    /// Collects the color, center, weight, and radial distance of each segment.
    ///
    /// # Arguments
    /// * `image` - The image that was segmented.
    /// * `labels` - The segment of each pixel of `image`, row by row, or `None` for a pixel in no
    ///   segment. The numbers need not be contiguous.
    /// * `estimation` - The method that chooses the color of each segment from its pixels.
    ///
    /// # Returns
    /// The segments of `image` that have weight.
    ///
    /// # Panics
    /// Panics if the number of `labels` does not match the number of pixels.
    pub(crate) fn of_labels(
        image: &ImageBuf<AlphaColor<Color>>,
        labels: &[Option<SegmentId>],
        estimation: Estimation,
    ) -> Self {
        assert_eq!(
            labels.len(),
            image.pixels().len(),
            "the number of labels does not match the number of pixels"
        );
        let segments = PixelGroups::of_labels(labels)
            .iter()
            .filter_map(|pixels| Segment::of_pixels(image, pixels, estimation))
            .collect();
        Self { segments }
    }

    /// Returns the segments.
    ///
    /// # Returns
    /// The segments, in the order of their numbers.
    pub(crate) fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

/// The pixels of each segment, in compressed sparse rows.
#[derive(Debug)]
struct PixelGroups {
    /// Where the pixels of each segment start in `pixels`, with the end appended.
    offsets: Vec<usize>,
    /// The indices of the pixels in a segment, segment by segment and row by row within each.
    pixels: Vec<usize>,
}

impl PixelGroups {
    /// Groups the pixels by segment.
    ///
    /// # Arguments
    /// * `labels` - The segment of each pixel, row by row, or `None` for a pixel in no segment.
    ///
    /// # Returns
    /// The pixels of each segment, with no pixels for a number no pixel has.
    fn of_labels(labels: &[Option<SegmentId>]) -> Self {
        // Count the pixels of each segment at the index where it will end.
        let mut offsets = vec![0];
        for id in labels.iter().flatten() {
            let end = id.index() + 1;
            if end >= offsets.len() {
                offsets.resize(end + 1, 0);
            }
            offsets[end] += 1;
        }

        // Sum the counts, so that each segment ends where the next one starts.
        for index in 1..offsets.len() {
            offsets[index] += offsets[index - 1];
        }

        // Place each pixel after those of its segment already placed, keeping rows in order.
        let mut next_slots = offsets.clone();
        let mut pixels = vec![0; offsets[offsets.len() - 1]];
        for (index, label) in labels.iter().enumerate() {
            let Some(id) = label else {
                continue;
            };
            pixels[next_slots[id.index()]] = index;
            next_slots[id.index()] += 1;
        }
        Self { offsets, pixels }
    }

    /// Returns the pixels of each segment.
    ///
    /// # Returns
    /// The indices of the pixels of each segment, in the order of their numbers.
    fn iter(&self) -> impl Iterator<Item = &[usize]> {
        self.offsets
            .windows(2)
            .map(|range| &self.pixels[range[0]..range[1]])
    }
}

/// Measures how far a pixel lies from the image center.
///
/// # Arguments
/// * `index` - The index of the pixel, row by row.
/// * `size` - The size of the image, with at least one pixel.
///
/// # Returns
/// The distance, with each axis of the image spanning 0.0 to 1.0.
fn measure_radial_distance(index: usize, size: Size) -> f64 {
    let width = size.width as usize;
    let (column, row) = (index % width, index / width);
    let x = (column as f64 + 0.5) / f64::from(size.width) - 0.5;
    let y = (row as f64 + 0.5) / f64::from(size.height) - 0.5;
    (x * x + y * y).sqrt()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{
        assert_approx_eq,
        next::extraction::test_helpers::{self, gray, BLACK},
    };

    /// A 2x1 image of an opaque dark pixel and a half-transparent light pixel.
    fn image() -> ImageBuf<AlphaColor<Color>> {
        ImageBuf::new(
            Size::new(2, 1),
            vec![
                AlphaColor::opaque(gray(0.2)),
                AlphaColor::new(gray(0.8), 0.5).unwrap(),
            ],
        )
    }

    #[test]
    fn test_index() {
        // Act
        let actual = SegmentId::new(3).index();

        // Assert
        assert_eq!(actual, 3);
    }

    #[test]
    fn test_of_labels_counts_segments() {
        // Act
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(0)), Some(SegmentId::new(1))],
            Estimation::default(),
        );

        // Assert
        assert_eq!(actual.segments.len(), 2);
    }

    #[test]
    fn test_of_labels_drops_unused_numbers() {
        // Act
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(3)), Some(SegmentId::new(1))],
            Estimation::default(),
        );

        // Assert
        assert_eq!(actual.segments.len(), 2);
    }

    #[test]
    fn test_of_labels_orders_segments_by_number() {
        // Act
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(3)), Some(SegmentId::new(1))],
            Estimation::default(),
        );

        // Assert: number 1 is the half-transparent pixel.
        assert_approx_eq!(actual.segments[0].weight(), 0.5);
    }

    #[test]
    fn test_of_labels_sums_opacity_as_weight() {
        // Act: both pixels in one segment.
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(0)); 2],
            Estimation::default(),
        );

        // Assert
        assert_approx_eq!(actual.segments[0].weight(), 1.5);
    }

    #[test]
    fn test_of_labels_estimates_color_from_pixels() {
        // Arrange
        let image = image();
        let estimation = Estimation::default();

        // Act: both pixels in one segment.
        let actual = Segments::of_labels(&image, &[Some(SegmentId::new(0)); 2], estimation);

        // Assert
        let color = actual.segments[0].color().to_oklab();
        let expected = estimation.estimate(image.pixels()).unwrap().to_oklab();
        assert_eq!(
            [color.l, color.a, color.b],
            [expected.l, expected.a, expected.b]
        );
    }

    #[test]
    fn test_of_labels_weights_center_by_opacity() {
        // Act: both pixels in one segment.
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(0)); 2],
            Estimation::default(),
        );

        // Assert: (0.25 * 1.0 + 0.75 * 0.5) / 1.5
        assert_approx_eq!(actual.segments[0].center().x, 0.625 / 1.5);
    }

    #[test]
    fn test_of_labels_averages_radial_distance_over_pixels() {
        // Arrange
        let image = test_helpers::image(&[[Some(BLACK); 4]]);
        let labels = [Some(SegmentId::new(0)), None, None, Some(SegmentId::new(0))];

        // Act
        let actual = Segments::of_labels(&image, &labels, Estimation::default());

        // Assert
        assert_approx_eq!(actual.segments[0].radial_distance(), 0.375);
    }

    #[test]
    fn test_of_labels_weights_radial_distance_by_opacity() {
        // Arrange
        let image = ImageBuf::new(
            Size::new(3, 1),
            vec![
                AlphaColor::opaque(BLACK),
                AlphaColor::new(BLACK, 0.5).unwrap(),
                AlphaColor::opaque(BLACK),
            ],
        );
        let labels = [Some(SegmentId::new(0)), Some(SegmentId::new(0)), None];

        // Act
        let actual = Segments::of_labels(&image, &labels, Estimation::default());

        // Assert: (1/3 * 1.0 + 0 * 0.5) / 1.5
        assert_approx_eq!(actual.segments[0].radial_distance(), 2.0 / 9.0);
    }

    #[test]
    fn test_of_labels_drops_weightless_segments() {
        // Arrange
        let image = test_helpers::image(&[[Some(BLACK), None]]);
        let labels = [Some(SegmentId::new(0)), Some(SegmentId::new(1))];

        // Act
        let actual = Segments::of_labels(&image, &labels, Estimation::default());

        // Assert
        assert_eq!(actual.segments.len(), 1);
    }

    #[test]
    fn test_of_labels_skips_unlabeled_pixels() {
        // Act
        let actual = Segments::of_labels(
            &image(),
            &[Some(SegmentId::new(0)), None],
            Estimation::default(),
        );

        // Assert
        assert_eq!(actual.segments[0].weight(), 1.0);
    }

    #[test]
    fn test_of_labels_without_labels() {
        // Act
        let actual = Segments::of_labels(&image(), &[None; 2], Estimation::default());

        // Assert
        assert!(actual.segments.is_empty());
    }

    #[test]
    #[should_panic(expected = "the number of labels does not match the number of pixels")]
    fn test_of_labels_label_count_mismatch() {
        // Act
        let _ = Segments::of_labels(&image(), &[None], Estimation::default());
    }

    #[test]
    fn test_pixel_groups_of_labels() {
        // Arrange
        let labels = [
            Some(SegmentId::new(1)),
            Some(SegmentId::new(0)),
            None,
            Some(SegmentId::new(1)),
        ];

        // Act
        let actual = PixelGroups::of_labels(&labels);

        // Assert
        assert_eq!(actual.iter().collect::<Vec<_>>(), [&[1][..], &[0, 3]]);
    }

    #[rstest]
    #[case::corner(0, 0.450_694)]
    #[case::inner(5, 0.279_508)]
    fn test_measure_radial_distance(#[case] index: usize, #[case] expected: f64) {
        // Act
        let actual = measure_radial_distance(index, Size::new(4, 2));

        // Assert
        assert_approx_eq!(actual, expected);
    }
}
