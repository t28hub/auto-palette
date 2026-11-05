use super::Extraction;
use crate::next::{Error, ImageRef};

/// Extracts the prominent colors of an image.
#[derive(Debug, Clone, Default)]
pub struct Extractor {
    // Private so that options can be added without breaking users.
    _private: (),
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
        let _image = image.try_into()?;
        todo!("extract colors from the image")
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
