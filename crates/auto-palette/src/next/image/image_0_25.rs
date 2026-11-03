//! Conversions from `image` 0.25 buffers, enabled by the `image-0_25` feature.

use image::{DynamicImage, GrayAlphaImage, GrayImage, ImageBuffer, Pixel, RgbImage, RgbaImage};

use super::{ImageRef, PixelFormat};
use crate::next::{error::ErrorDetail, Error};

impl<'a> ImageRef<'a> {
    /// Creates a view of the pixels of an 8-bit `image` buffer.
    ///
    /// # Arguments
    /// * `image` - The image buffer to borrow.
    /// * `format` - The pixel format that matches the pixel type of `image`.
    ///
    /// # Returns
    /// A view of the pixels of `image`.
    fn from_image_buffer<P>(image: &'a ImageBuffer<P, Vec<u8>>, format: PixelFormat) -> Self
    where
        P: Pixel<Subpixel = u8>,
    {
        let (width, height) = image.dimensions();
        // `ImageBuffer` guarantees that its buffer fits in `usize` and holds every pixel;
        // `with_stride` drops any extra bytes.
        let bytes_per_row = width as usize * format.bytes_per_pixel();
        Self::with_stride(image.as_raw(), width, height, bytes_per_row, format)
            .expect("an `ImageBuffer` always holds all of its pixels")
    }
}

impl<'a> From<&'a GrayImage> for ImageRef<'a> {
    fn from(image: &'a GrayImage) -> Self {
        Self::from_image_buffer(image, PixelFormat::Luma8)
    }
}

impl<'a> From<&'a GrayAlphaImage> for ImageRef<'a> {
    fn from(image: &'a GrayAlphaImage) -> Self {
        Self::from_image_buffer(image, PixelFormat::LumaA8)
    }
}

impl<'a> From<&'a RgbImage> for ImageRef<'a> {
    fn from(image: &'a RgbImage) -> Self {
        Self::from_image_buffer(image, PixelFormat::Rgb8)
    }
}

impl<'a> From<&'a RgbaImage> for ImageRef<'a> {
    fn from(image: &'a RgbaImage) -> Self {
        Self::from_image_buffer(image, PixelFormat::Rgba8)
    }
}

/// Borrows the pixels of an 8-bit image without copying them.
///
/// # Errors
/// Returns [`UnsupportedPixelFormat`](crate::next::ErrorKind::UnsupportedPixelFormat) for 16-bit
/// and floating-point images. Convert them with [`DynamicImage::to_rgba8`] first.
impl<'a> TryFrom<&'a DynamicImage> for ImageRef<'a> {
    type Error = Error;

    fn try_from(image: &'a DynamicImage) -> Result<Self, Self::Error> {
        match image {
            DynamicImage::ImageLuma8(image) => Ok(Self::from(image)),
            DynamicImage::ImageLumaA8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgb8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgba8(image) => Ok(Self::from(image)),
            _ => Err(ErrorDetail::UnsupportedColorType {
                color: format!("{:?}", image.color()),
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::ErrorKind;

    #[test]
    fn test_from_gray_image() {
        // Arrange
        let image = GrayImage::new(2, 3);

        // Act
        let actual = ImageRef::from(&image);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Luma8);
    }

    #[test]
    fn test_from_gray_alpha_image() {
        // Arrange
        let image = GrayAlphaImage::new(2, 3);

        // Act
        let actual = ImageRef::from(&image);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::LumaA8);
    }

    #[test]
    fn test_from_rgb_image() {
        // Arrange
        let image = RgbImage::new(2, 3);

        // Act
        let actual = ImageRef::from(&image);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Rgb8);
    }

    #[test]
    fn test_from_rgba_image() {
        // Arrange
        let image = RgbaImage::new(2, 3);

        // Act
        let actual = ImageRef::from(&image);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Rgba8);
    }

    #[test]
    fn test_from_rgba_image_longer_buffer() {
        // Arrange
        let image = RgbaImage::from_raw(2, 3, vec![0; 2 * 3 * 4 + 5]).unwrap();

        // Act
        let actual = ImageRef::from(&image);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Rgba8);
    }

    #[rstest]
    #[case::luma8(DynamicImage::new_luma8(2, 3), PixelFormat::Luma8)]
    #[case::luma_a8(DynamicImage::new_luma_a8(2, 3), PixelFormat::LumaA8)]
    #[case::rgb8(DynamicImage::new_rgb8(2, 3), PixelFormat::Rgb8)]
    #[case::rgba8(DynamicImage::new_rgba8(2, 3), PixelFormat::Rgba8)]
    fn test_try_from_dynamic_image(#[case] image: DynamicImage, #[case] expected: PixelFormat) {
        // Act
        let actual = ImageRef::try_from(&image).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), expected);
    }

    #[rstest]
    #[case::luma16(DynamicImage::new_luma16(2, 3), "L16")]
    #[case::luma_a16(DynamicImage::new_luma_a16(2, 3), "La16")]
    #[case::rgb16(DynamicImage::new_rgb16(2, 3), "Rgb16")]
    #[case::rgba16(DynamicImage::new_rgba16(2, 3), "Rgba16")]
    #[case::rgb32f(DynamicImage::new_rgb32f(2, 3), "Rgb32F")]
    #[case::rgba32f(DynamicImage::new_rgba32f(2, 3), "Rgba32F")]
    fn test_try_from_dynamic_image_unsupported_color_type(
        #[case] image: DynamicImage,
        #[case] color: &str,
    ) {
        // Act
        let actual = ImageRef::try_from(&image);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::UnsupportedPixelFormat);
        assert_eq!(
            error.to_string(),
            format!("unsupported pixel format {color}, convert the image with to_rgba8() first")
        );
    }
}
