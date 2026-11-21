use std::num::NonZeroU32;

use crate::next::{
    color::{AlphaColor, Color},
    extract::{
        clustering::{self, Initialization, Lloyd},
        point::Point,
        position::Position,
        segment::{segments::SegmentId, Segmenter},
    },
    image::ImageBuf,
};

/// Lloyd's k-means over color and position, which groups nearby pixels of similar colors.
#[derive(Debug, Clone, Copy)]
pub(crate) struct KMeans {
    /// How much distance counts against color difference.
    compactness: f64,
    /// The k-means that groups the pixels, aiming for one cluster per segment.
    kmeans: clustering::KMeans,
}

impl KMeans {
    /// The default number of segments to aim for.
    const DEFAULT_SEGMENTS: NonZeroU32 = NonZeroU32::new(128).unwrap();

    /// The default for `compactness`.
    const DEFAULT_COMPACTNESS: f64 = 0.1;

    /// Creates k-means with the given options.
    ///
    /// # Arguments
    /// * `compactness` - How much distance counts against color difference.
    /// * `kmeans` - The k-means that groups the pixels, aiming for one cluster per segment.
    ///
    /// # Returns
    /// K-means, or `None` if `compactness` is negative or not finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(compactness: f64, kmeans: clustering::KMeans) -> Option<Self> {
        if !compactness.is_finite() || compactness < 0.0 {
            return None;
        }
        Some(Self {
            compactness,
            kmeans,
        })
    }
}

impl Default for KMeans {
    fn default() -> Self {
        Self {
            compactness: Self::DEFAULT_COMPACTNESS,
            kmeans: clustering::KMeans::new(
                Self::DEFAULT_SEGMENTS,
                Initialization::default(),
                Lloyd::default(),
            ),
        }
    }
}

impl Segmenter for KMeans {
    fn label(&self, image: &ImageBuf<AlphaColor<Color>>) -> Vec<Option<SegmentId>> {
        let count = self.kmeans.count().get() as usize;
        let scale = self.compactness / Position::segment_length(image.size(), count);
        let points = Point::of_pixels(image, scale);
        let mut labels = vec![None; points.len()];
        for (id, cluster) in (0..).zip(self.kmeans.cluster(&points)) {
            for &member in cluster.members() {
                labels[member] = Some(SegmentId::new(id));
            }
        }
        labels
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extract::test_helpers::{gradient, image, BLACK, WHITE};

    /// Creates k-means that aims for `segments` segments, with initial centers from `initialization`.
    fn kmeans(segments: u32, initialization: Initialization) -> KMeans {
        KMeans {
            compactness: KMeans::DEFAULT_COMPACTNESS,
            kmeans: clustering::KMeans::new(
                NonZeroU32::new(segments).unwrap(),
                initialization,
                Lloyd::default(),
            ),
        }
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::positive(0.1)]
    fn test_new_accepts_compactness(#[case] compactness: f64) {
        // Act
        let actual = KMeans::new(compactness, KMeans::default().kmeans);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::negative(-0.1)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_compactness(#[case] compactness: f64) {
        // Act
        let actual = KMeans::new(compactness, KMeans::default().kmeans);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case::plus_plus(Initialization::default())]
    #[case::farthest(Initialization::Farthest)]
    fn test_label_groups_colors_across_the_image(#[case] initialization: Initialization) {
        // Arrange: black and white in a checkerboard, so neither color is connected.
        let image = image(&[[Some(BLACK), Some(WHITE)], [Some(WHITE), Some(BLACK)]]);
        let kmeans = kmeans(2, initialization);

        // Act
        let actual = kmeans.label(&image);

        // Assert
        assert_ne!(actual[0], actual[1]);
        assert_eq!(actual, [actual[0], actual[1], actual[1], actual[0]]);
    }

    #[test]
    fn test_label_splits_one_color_by_position() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(BLACK)]]);

        // Act
        let actual = kmeans(2, Initialization::default()).label(&image);

        // Assert
        assert_ne!(actual[0], actual[1]);
    }

    #[test]
    fn test_label_ignores_position_without_compactness() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(BLACK)]]);
        let kmeans = KMeans {
            compactness: 0.0,
            ..kmeans(2, Initialization::default())
        };

        // Act
        let actual = kmeans.label(&image);

        // Assert
        assert_eq!(actual[0], actual[1]);
    }

    #[test]
    fn test_label_skips_transparent_pixels() {
        // Arrange
        let image = image(&[[Some(BLACK), None]]);

        // Act
        let actual = kmeans(1, Initialization::default()).label(&image);

        // Assert
        assert_eq!(actual, [Some(SegmentId::new(0)), None]);
    }

    #[rstest]
    #[case::plus_plus(Initialization::default())]
    #[case::farthest(Initialization::Farthest)]
    fn test_label_fewer_pixels_than_segments(#[case] initialization: Initialization) {
        // Arrange
        let image = image(&[[Some(BLACK), Some(WHITE), Some(BLACK)]]);
        let kmeans = kmeans(8, initialization);

        // Act
        let actual = kmeans.label(&image);

        // Assert: one segment per pixel.
        let mut ids: Vec<usize> = actual.iter().map(|id| id.unwrap().index()).collect();
        ids.sort_unstable();
        assert_eq!(ids, [0, 1, 2]);
    }

    #[test]
    fn test_label_is_reproducible() {
        // Arrange: a gradient, so the initial centers depend on the random draws.
        let image = image(&[gradient::<16>().map(Some)]);

        // Act
        let first = kmeans(4, Initialization::default()).label(&image);
        let second = kmeans(4, Initialization::default()).label(&image);

        // Assert
        assert_eq!(first, second);
    }
}
