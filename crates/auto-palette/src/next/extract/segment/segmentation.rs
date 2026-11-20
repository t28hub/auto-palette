use super::{kmeans::KMeans, segments::SegmentId, snic::Snic};
use crate::next::{
    color::{AlphaColor, Color},
    image::ImageBuf,
};

/// An algorithm that assigns each pixel of an image to a segment.
pub(crate) trait Segmenter {
    /// Assigns each pixel of `image` to a segment.
    ///
    /// # Arguments
    /// * `image` - The image to split, with at least one opaque pixel.
    ///
    /// # Returns
    /// The segment of each pixel, row by row, or `None` for a pixel in no segment, such as a
    /// transparent pixel or noise. The numbers need not be contiguous, but stay below the number
    /// of pixels.
    fn label(&self, image: &ImageBuf<AlphaColor<Color>>) -> Vec<Option<SegmentId>>;
}

/// The segmentation algorithm to use.
#[derive(Debug, Clone)]
pub(crate) enum Segmentation {
    /// SNIC ([`Snic`]).
    Snic(Snic),
    /// K-means ([`KMeans`]).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    KMeans(KMeans),
}

impl Default for Segmentation {
    fn default() -> Self {
        Self::Snic(Snic::default())
    }
}

impl Segmenter for Segmentation {
    fn label(&self, image: &ImageBuf<AlphaColor<Color>>) -> Vec<Option<SegmentId>> {
        match self {
            Self::Snic(snic) => snic.label(image),
            Self::KMeans(kmeans) => kmeans.label(image),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extract::segment::test_helpers::{self, BLACK};

    /// A 2x1 image of an opaque pixel and a transparent pixel.
    fn image() -> ImageBuf<AlphaColor<Color>> {
        test_helpers::image(&[[Some(BLACK), None]])
    }

    #[rstest]
    #[case::snic(Segmentation::Snic(Snic::default()), Snic::default().label(&image()))]
    #[case::kmeans(Segmentation::KMeans(KMeans::default()), KMeans::default().label(&image()))]
    fn test_label(#[case] segmentation: Segmentation, #[case] expected: Vec<Option<SegmentId>>) {
        // Act
        let actual = segmentation.label(&image());

        // Assert
        assert_eq!(actual, expected);
    }
}
