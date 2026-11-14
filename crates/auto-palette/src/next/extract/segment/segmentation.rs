use super::{segments::SegmentId, snic::Snic, Segments};
use crate::next::{
    color::{AlphaColor, Oklab},
    image::ImageBuf,
};

/// An algorithm that splits an image into segments.
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
    fn label(&self, image: &ImageBuf<AlphaColor<Oklab>>) -> Vec<Option<SegmentId>>;

    /// Splits `image` into segments.
    ///
    /// # Arguments
    /// * `image` - The image to split.
    ///
    /// # Returns
    /// The segments of `image`, or no segments if `image` has no opaque pixels.
    fn segment(&self, image: &ImageBuf<AlphaColor<Oklab>>) -> Segments {
        if image.pixels().iter().all(AlphaColor::is_transparent) {
            return Segments::default();
        }
        Segments::new(image, &self.label(image))
    }
}

/// The segmentation algorithm to use.
#[derive(Debug, Clone)]
pub(crate) enum Segmentation {
    /// SNIC ([`Snic`]).
    Snic(Snic),
}

impl Default for Segmentation {
    fn default() -> Self {
        Self::Snic(Snic::default())
    }
}

impl Segmenter for Segmentation {
    fn label(&self, image: &ImageBuf<AlphaColor<Oklab>>) -> Vec<Option<SegmentId>> {
        match self {
            Self::Snic(snic) => snic.label(image),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::{
        extract::segment::test_helpers::{self, BLACK},
        image::Size,
    };

    /// A 2x1 image of an opaque pixel and a transparent pixel.
    fn image() -> ImageBuf<AlphaColor<Oklab>> {
        test_helpers::image(&[[Some(BLACK), None]])
    }

    #[rstest]
    #[case::snic(Segmentation::Snic(Snic::default()), Snic::default().label(&image()))]
    fn test_label(#[case] segmentation: Segmentation, #[case] expected: Vec<Option<SegmentId>>) {
        // Act
        let actual = segmentation.label(&image());

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_segment() {
        // Act
        let actual = Segmentation::default().segment(&image());

        // Assert: the transparent pixel is in no segment.
        assert_eq!(actual.segments().len(), 1);
    }

    #[test]
    fn test_segment_transparent() {
        // Arrange
        let image = test_helpers::image(&[[None, None]]);

        // Act
        let actual = Segmentation::default().segment(&image);

        // Assert
        assert!(actual.segments().is_empty());
    }

    #[test]
    fn test_segment_empty() {
        // Arrange
        let image = ImageBuf::new(Size::new(0, 0), Vec::new());

        // Act
        let actual = Segmentation::default().segment(&image);

        // Assert
        assert!(actual.segments().is_empty());
    }
}
