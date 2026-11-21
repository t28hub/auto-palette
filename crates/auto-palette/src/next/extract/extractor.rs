use super::{
    preprocessor::Preprocessor,
    segment::{Estimation, Segmentation, Segmenter, Segments},
    Extraction,
};
use crate::next::{
    color::{AlphaColor, Color},
    image::ImageBuf,
    Error,
    ImageRef,
};

/// Extracts the prominent colors of an image.
#[derive(Debug, Clone, Default)]
pub struct Extractor {
    /// Prepares the image for extraction.
    preprocessor: Preprocessor,
    /// Splits the prepared image into segments.
    segmentation: Segmentation,
    /// Chooses the color of each segment from its pixels.
    estimation: Estimation,
}

impl Extractor {
    /// Creates an extractor with the default options.
    ///
    /// # Returns
    /// An extractor with the default options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts the prominent colors of an image.
    ///
    /// # Arguments
    /// * `image` - The image to extract colors from, such as an [`ImageRef`] or an `image` crate
    ///   buffer (feature `image-0_25`).
    ///
    /// # Returns
    /// The result of extracting colors from `image`.
    ///
    /// # Errors
    /// Returns an error if `image` cannot be converted into an [`ImageRef`].
    pub fn extract<'a, I>(&self, image: I) -> Result<Extraction, Error>
    where
        I: TryInto<ImageRef<'a>>,
        Error: From<I::Error>,
    {
        self.extract_image(&image.try_into()?)
    }

    /// The non-generic body of [`Self::extract`].
    ///
    /// # Arguments
    /// * `image` - The image to extract colors from.
    ///
    /// # Returns
    /// The result of extracting colors from `image`.
    fn extract_image(&self, image: &ImageRef<'_>) -> Result<Extraction, Error> {
        let image = self.preprocessor.preprocess(image);
        let _segments = self.split_into_segments(&image);
        todo!("select colors from the segments")
    }

    /// Splits a prepared image into segments and summarizes each.
    ///
    /// # Arguments
    /// * `image` - The prepared image.
    ///
    /// # Returns
    /// The segments of `image`, or no segments if `image` has no opaque pixels.
    fn split_into_segments(&self, image: &ImageBuf<AlphaColor<Color>>) -> Segments {
        if image.pixels().iter().all(AlphaColor::is_transparent) {
            return Segments::default();
        }
        let labels = self.segmentation.label(image);
        Segments::of_labels(image, &labels, self.estimation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::{color::Oklab, image::Size};

    #[test]
    fn test_split_into_segments_single_pixel() {
        // Arrange
        let image = ImageBuf::new(Size::new(1, 1), vec![AlphaColor::opaque(Color::BLACK)]);

        // Act
        let actual = Extractor::new().split_into_segments(&image);

        // Assert
        assert_eq!(actual.segments().len(), 1);
    }

    #[test]
    fn test_split_into_segments_two_pixels() {
        // Arrange
        let white = Color::from_oklab(Oklab::new(1.0, 0.0, 0.0));
        let image = ImageBuf::new(
            Size::new(2, 1),
            vec![AlphaColor::opaque(Color::BLACK), AlphaColor::opaque(white)],
        );

        // Act
        let actual = Extractor::new().split_into_segments(&image);

        // Assert
        assert_eq!(actual.segments().len(), 2);
    }

    #[test]
    fn test_split_into_segments_ignores_transparent_pixels() {
        // Arrange
        let image = ImageBuf::new(
            Size::new(2, 1),
            vec![
                AlphaColor::opaque(Color::BLACK),
                AlphaColor::transparent(Color::BLACK),
            ],
        );

        // Act
        let actual = Extractor::new().split_into_segments(&image);

        // Assert
        assert_eq!(actual.segments().len(), 1);
    }

    #[test]
    fn test_split_into_segments_transparent_image() {
        // Arrange
        let image = ImageBuf::new(
            Size::new(2, 1),
            vec![AlphaColor::transparent(Color::BLACK); 2],
        );

        // Act
        let actual = Extractor::new().split_into_segments(&image);

        // Assert
        assert!(actual.segments().is_empty());
    }

    #[test]
    fn test_split_into_segments_empty() {
        // Arrange
        let image = ImageBuf::new(Size::new(0, 0), Vec::new());

        // Act
        let actual = Extractor::new().split_into_segments(&image);

        // Assert
        assert!(actual.segments().is_empty());
    }

    #[test]
    fn test_extract_accepts_image_inputs() {
        // Assert: these compile.
        let _ = Extractor::extract::<ImageRef<'_>>;
        #[cfg(feature = "image-0_25")]
        {
            let _ = Extractor::extract::<&image::RgbaImage>;
            let _ = Extractor::extract::<&image::DynamicImage>;
        }
    }

    #[cfg(feature = "image-0_25")]
    #[test]
    fn test_extract_unsupported_pixel_format() {
        // Arrange
        let extractor = Extractor::new();
        let image = image::DynamicImage::new_luma16(1, 1);

        // Act
        let actual = extractor.extract(&image);

        // Assert
        assert_eq!(
            actual.unwrap_err().kind(),
            crate::next::ErrorKind::UnsupportedPixelFormat
        );
    }

    #[test]
    fn test_extractor_is_send_sync() {
        // Assert: users can share an extractor across threads.
        fn assert_bounds<T: Send + Sync>() {}
        assert_bounds::<Extractor>();
    }
}
