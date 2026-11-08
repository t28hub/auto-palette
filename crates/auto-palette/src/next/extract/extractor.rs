use std::num::NonZeroU32;

use super::{
    resample::{Resample, Resampling},
    Extraction,
};
use crate::next::{
    color::{AlphaColor, Oklab},
    image::ImageBuf,
    Error,
    ImageRef,
};

/// Extracts the prominent colors of an image.
#[derive(Debug, Clone)]
pub struct Extractor {
    /// The largest number of pixels to keep. Larger images are scaled down.
    max_pixels: NonZeroU32,
    /// The algorithm that scales the image down.
    resampling: Resampling,
}

impl Extractor {
    /// The default for `max_pixels`: 256 × 256 pixels.
    // Evaluated at compile time, so a zero fails the build instead of panicking.
    const DEFAULT_MAX_PIXELS: NonZeroU32 = NonZeroU32::new(256 * 256).unwrap();

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
        let _image = self.preprocess(image);
        todo!("extract colors from the preprocessed image")
    }

    /// Prepares an [`ImageRef`] for extraction.
    ///
    /// 1. Scales the image down to at most `max_pixels` pixels with `resampling`.
    /// 2. Converts each pixel to OKLab and keeps its alpha. Fully transparent pixels become black.
    ///
    /// # Arguments
    /// * `image` - The image to prepare.
    ///
    /// # Returns
    /// The image in OKLab, or an empty image if `image` has no pixels.
    fn preprocess(&self, image: &ImageRef<'_>) -> ImageBuf<AlphaColor<Oklab>> {
        let size = image.size().fit(self.max_pixels);
        if size.is_empty() {
            return ImageBuf::new(size, Vec::new());
        }
        let scaled = self.resampling.resample(image, size);
        scaled.map(|pixel| {
            if pixel.is_transparent() {
                AlphaColor::transparent(Oklab::new(0.0, 0.0, 0.0))
            } else {
                pixel.map_color(Oklab::from)
            }
        })
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            max_pixels: Self::DEFAULT_MAX_PIXELS,
            resampling: Resampling::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::{image::Size, PixelFormat};

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
    fn test_preprocess_keeps_alpha() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White
            255, 0, 0, 128, // Half-transparent red
            0, 0, 0, 0, // Transparent
        ];
        let image = ImageRef::from_rgba8(&pixels, 3, 1).unwrap();

        // Act
        let actual = Extractor::new().preprocess(&image);

        // Assert
        let alphas: Vec<f64> = actual.pixels().iter().map(|pixel| pixel.alpha()).collect();
        assert_eq!(alphas, [1.0, 128.0 / 255.0, 0.0]);
    }

    #[test]
    fn test_preprocess_scales_down() {
        // Arrange
        let pixels = vec![0; 60 * 40];
        let image = ImageRef::new(&pixels, 60, 40, PixelFormat::Luma8).unwrap();
        let extractor = Extractor {
            max_pixels: NonZeroU32::new(6).unwrap(),
            ..Extractor::default()
        };

        // Act
        let actual = extractor.preprocess(&image);

        // Assert
        assert_eq!(actual.size(), Size::new(3, 2));
    }

    #[rstest]
    #[case::area(Resampling::Area)]
    #[case::nearest(Resampling::Nearest)]
    fn test_preprocess_transparent_is_black(#[case] resampling: Resampling) {
        // Arrange: transparent white, which `Nearest` passes through with its color.
        let pixels = [255, 255, 255, 0];
        let image = ImageRef::from_rgba8(&pixels, 1, 1).unwrap();
        let extractor = Extractor {
            resampling,
            ..Extractor::default()
        };

        // Act
        let actual = extractor.preprocess(&image);

        // Assert
        let [pixel] = actual.pixels() else {
            panic!("expected one pixel");
        };
        let color = pixel.color();
        assert_eq!((color.l, color.a, color.b), (0.0, 0.0, 0.0));
        assert!(pixel.is_transparent());
    }

    #[rstest]
    #[case::empty(0, 0)]
    #[case::zero_width(0, 3)]
    #[case::zero_height(2, 0)]
    fn test_preprocess_empty(#[case] width: u32, #[case] height: u32) {
        // Arrange
        let image = ImageRef::from_rgba8(&[], width, height).unwrap();

        // Act
        let actual = Extractor::new().preprocess(&image);

        // Assert
        assert_eq!(actual.size(), Size::new(width, height));
        assert!(actual.pixels().is_empty());
    }

    #[test]
    fn test_extractor_is_send_sync() {
        // Assert: users can share an extractor across threads.
        fn assert_bounds<T: Send + Sync>() {}
        assert_bounds::<Extractor>();
    }
}
