use super::{centroid::Centroid, point::Point, position::Position};
use crate::next::{
    color::{AlphaColor, Oklab},
    image::ImageBuf,
};

/// The number of a segment.
#[derive(Debug, Clone, Copy, PartialEq)]
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

    /// Returns the number as an index into the segments.
    ///
    /// # Returns
    /// The number, counted from 0.
    #[inline]
    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

/// The mean color, center, and weight of one segment.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Segment(Point);

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the color selection, which is not implemented yet"
    )
)]
impl Segment {
    /// Returns the mean color of the pixels, weighted by their opacity.
    ///
    /// # Returns
    /// The mean color.
    #[inline]
    pub(crate) fn color(self) -> Oklab {
        self.0.color()
    }

    /// Returns the mean position of the pixels, weighted by their opacity.
    ///
    /// # Returns
    /// The center of the segment.
    #[inline]
    pub(crate) fn center(self) -> Position {
        self.0.position()
    }

    /// Returns the sum of the opacities of the pixels.
    ///
    /// # Returns
    /// The weight of the segment.
    #[inline]
    pub(crate) fn weight(self) -> f64 {
        self.0.weight()
    }

    /// Returns the segment as a point, with its position unscaled.
    ///
    /// # Returns
    /// The mean color at the center, with the weight.
    #[inline]
    pub(crate) fn point(self) -> Point {
        self.0
    }
}

/// The segments of an image.
#[derive(Debug, Default)]
pub(crate) struct Segments {
    /// The segments, in the order of their numbers.
    segments: Vec<Segment>,
}

impl Segments {
    /// Collects the color, weight, and center of each segment.
    ///
    /// # Arguments
    /// * `image` - The image that was segmented.
    /// * `labels` - The segment of each pixel of `image`, row by row, or `None` for a pixel in no
    ///   segment. The numbers need not be contiguous.
    ///
    /// # Returns
    /// The segments of `image` that have weight.
    ///
    /// # Panics
    /// Panics if the number of `labels` does not match the number of pixels.
    pub(crate) fn new(image: &ImageBuf<AlphaColor<Oklab>>, labels: &[Option<SegmentId>]) -> Self {
        assert_eq!(
            labels.len(),
            image.pixels().len(),
            "the number of labels does not match the number of pixels"
        );
        let count = labels
            .iter()
            .flatten()
            .map(|id| id.index() + 1)
            .max()
            .unwrap_or(0);
        let mut centroids = vec![Centroid::default(); count];
        for (index, label) in labels.iter().enumerate() {
            let Some(id) = label else {
                continue;
            };
            centroids[id.index()].add(Point::at(image, index));
        }
        let segments = centroids
            .iter()
            .filter_map(Centroid::point)
            .map(Segment)
            .collect();
        Self { segments }
    }

    /// Returns the segments.
    ///
    /// # Returns
    /// The segments, in the order of their numbers.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the color selection, which is not implemented yet"
        )
    )]
    pub(crate) fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_eq, next::image::Size};

    /// A 2x1 image of an opaque dark pixel and a half-transparent light pixel.
    fn image() -> ImageBuf<AlphaColor<Oklab>> {
        ImageBuf::new(
            Size::new(2, 1),
            vec![
                AlphaColor::opaque(Oklab::new(0.2, 0.0, 0.0)),
                AlphaColor::new(Oklab::new(0.8, 0.0, 0.0), 0.5).unwrap(),
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
    fn test_new_counts_segments() {
        // Act
        let actual = Segments::new(
            &image(),
            &[Some(SegmentId::new(0)), Some(SegmentId::new(1))],
        );

        // Assert
        assert_eq!(actual.segments.len(), 2);
    }

    #[test]
    fn test_new_drops_unused_numbers() {
        // Act
        let actual = Segments::new(
            &image(),
            &[Some(SegmentId::new(3)), Some(SegmentId::new(1))],
        );

        // Assert
        assert_eq!(actual.segments.len(), 2);
    }

    #[test]
    fn test_new_orders_segments_by_number() {
        // Act
        let actual = Segments::new(
            &image(),
            &[Some(SegmentId::new(3)), Some(SegmentId::new(1))],
        );

        // Assert: number 1 is the light pixel.
        assert_approx_eq!(actual.segments[0].color().l, 0.8);
    }

    #[test]
    fn test_new_sums_opacity_as_weight() {
        // Act: both pixels in one segment.
        let actual = Segments::new(&image(), &[Some(SegmentId::new(0)); 2]);

        // Assert
        assert_approx_eq!(actual.segments[0].weight(), 1.5);
    }

    #[test]
    fn test_new_weights_color_by_opacity() {
        // Act: both pixels in one segment.
        let actual = Segments::new(&image(), &[Some(SegmentId::new(0)); 2]);

        // Assert: (0.2 * 1.0 + 0.8 * 0.5) / 1.5 = 0.4
        assert_approx_eq!(actual.segments[0].color().l, 0.4);
    }

    #[test]
    fn test_new_weights_center_by_opacity() {
        // Act: both pixels in one segment.
        let actual = Segments::new(&image(), &[Some(SegmentId::new(0)); 2]);

        // Assert: (0.25 * 1.0 + 0.75 * 0.5) / 1.5
        assert_approx_eq!(actual.segments[0].center().x, 0.625 / 1.5);
    }

    #[test]
    fn test_segment_point() {
        // Arrange
        let segments = Segments::new(&image(), &[Some(SegmentId::new(0)), None]);
        let segment = segments.segments[0];

        // Act
        let actual = segment.point();

        // Assert: the center as the position, not scaled.
        assert_eq!(actual.position(), segment.center());
    }

    #[test]
    fn test_new_skips_unlabeled_pixels() {
        // Act
        let actual = Segments::new(&image(), &[Some(SegmentId::new(0)), None]);

        // Assert
        assert_eq!(actual.segments[0].weight(), 1.0);
    }

    #[test]
    fn test_new_without_labels() {
        // Act
        let actual = Segments::new(&image(), &[None; 2]);

        // Assert
        assert!(actual.segments.is_empty());
    }

    #[test]
    #[should_panic(expected = "the number of labels does not match the number of pixels")]
    fn test_new_label_count_mismatch() {
        // Act
        let _ = Segments::new(&image(), &[None]);
    }
}
