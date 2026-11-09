use super::{
    preprocessor::Preprocessor,
    segment::{Segmentation, Segmenter},
    Extraction,
};
use crate::next::{Error, ImageRef};

/// Extracts the prominent colors of an image.
#[derive(Debug, Clone, Default)]
pub struct Extractor {
    /// Prepares the image for extraction.
    preprocessor: Preprocessor,
    /// Splits the prepared image into segments.
    segmentation: Segmentation,
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
        let _segments = self.segmentation.segment(&image);
        todo!("select colors from the segments")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
