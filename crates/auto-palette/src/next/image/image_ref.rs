use std::fmt;

use super::PixelFormat;
use crate::next::{error::ErrorDetail, Error};

/// A borrowed view of 8-bit image data.
///
/// The view never copies the pixel data. Rows are stored top to bottom, and the pixels in a
/// row left to right.
///
/// A view converted from an `image` buffer (feature `image-0_25`) ignores the buffer's color space
/// metadata and reads the pixels as sRGB.
///
/// # Examples
/// ```
/// use auto_palette::next::{ImageRef, PixelFormat};
///
/// let pixels = [
///     255, 0, 0, 255, // Red
///     0, 0, 255, 255, // Blue
/// ];
/// let image = ImageRef::from_rgba8(&pixels, 2, 1).unwrap();
/// assert_eq!(image.width(), 2);
/// assert_eq!(image.height(), 1);
/// assert_eq!(image.format(), PixelFormat::Rgba8);
/// ```
#[derive(Clone, Copy)]
pub struct ImageRef<'a> {
    #[expect(
        dead_code,
        reason = "read by the color extractor, which is not implemented yet"
    )]
    data: &'a [u8],
    width: u32,
    height: u32,
    bytes_per_row: usize,
    format: PixelFormat,
}

impl<'a> ImageRef<'a> {
    /// Creates a view of tightly packed RGB pixel data.
    ///
    /// # Arguments
    /// * `data` - The pixel data, exactly `width * height * 3` bytes long.
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    ///
    /// # Returns
    /// A view of the pixel data.
    ///
    /// # Errors
    /// Returns an error if the length of `data` does not match the image size, or if the image
    /// size overflows `usize`.
    #[inline]
    pub fn from_rgb8(data: &'a [u8], width: u32, height: u32) -> Result<Self, Error> {
        Self::new(data, width, height, PixelFormat::Rgb8)
    }

    /// Creates a view of tightly packed RGBA pixel data.
    ///
    /// # Arguments
    /// * `data` - The pixel data, exactly `width * height * 4` bytes long.
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    ///
    /// # Returns
    /// A view of the pixel data.
    ///
    /// # Errors
    /// Returns an error if the length of `data` does not match the image size, or if the image
    /// size overflows `usize`.
    #[inline]
    pub fn from_rgba8(data: &'a [u8], width: u32, height: u32) -> Result<Self, Error> {
        Self::new(data, width, height, PixelFormat::Rgba8)
    }

    /// Creates a view of tightly packed pixel data in the given format.
    ///
    /// # Arguments
    /// * `data` - The pixel data, exactly `width * height * format.bytes_per_pixel()` bytes long.
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    /// * `format` - The pixel format of the data.
    ///
    /// # Returns
    /// A view of the pixel data.
    ///
    /// # Errors
    /// Returns an error if the length of `data` does not match the image size, or if the image
    /// size overflows `usize`.
    pub fn new(
        data: &'a [u8],
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<Self, Error> {
        let bytes_per_row = (width as usize)
            .checked_mul(format.bytes_per_pixel())
            .ok_or(ErrorDetail::ImageTooLarge {
                width,
                height,
                format,
            })?;
        let expected =
            bytes_per_row
                .checked_mul(height as usize)
                .ok_or(ErrorDetail::ImageTooLarge {
                    width,
                    height,
                    format,
                })?;
        if data.len() != expected {
            return Err(ErrorDetail::DataLengthMismatch {
                width,
                height,
                format,
                expected,
                actual: data.len(),
            }
            .into());
        }

        Self::with_stride(data, width, height, bytes_per_row, format)
    }

    /// Creates a view of pixel data whose rows are `bytes_per_row` bytes apart.
    ///
    /// Use this for rows padded for alignment, or for a sub-region of a larger image. In that
    /// case `data` starts at the first pixel of the region and `bytes_per_row` is the row length
    /// of the larger image. The last row does not need padding, and `data` may be longer than
    /// the image requires.
    ///
    /// # Arguments
    /// * `data` - The pixel data.
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    /// * `bytes_per_row` - The distance between the starts of two adjacent rows, in bytes.
    /// * `format` - The pixel format of the data.
    ///
    /// # Returns
    /// A view of the pixel data.
    ///
    /// # Errors
    /// Returns an error if `bytes_per_row` is shorter than one row of pixels, if `data` is
    /// shorter than the image requires, or if the image size overflows `usize`.
    pub fn with_stride(
        data: &'a [u8],
        width: u32,
        height: u32,
        bytes_per_row: usize,
        format: PixelFormat,
    ) -> Result<Self, Error> {
        let min = (width as usize)
            .checked_mul(format.bytes_per_pixel())
            .ok_or(ErrorDetail::ImageTooLarge {
                width,
                height,
                format,
            })?;
        if bytes_per_row < min {
            return Err(ErrorDetail::RowStrideTooShort { bytes_per_row, min }.into());
        }

        let required = match (height as usize).checked_sub(1) {
            None => 0,
            Some(rows_before_last) => bytes_per_row
                .checked_mul(rows_before_last)
                .and_then(|length| length.checked_add(min))
                .ok_or(ErrorDetail::ImageTooLarge {
                    width,
                    height,
                    format,
                })?,
        };
        if data.len() < required {
            return Err(ErrorDetail::DataTooShort {
                width,
                height,
                format,
                required,
                actual: data.len(),
            }
            .into());
        }

        Ok(Self {
            data: &data[..required],
            width,
            height,
            bytes_per_row,
            format,
        })
    }

    /// Returns the width of the image in pixels.
    ///
    /// # Returns
    /// The width of the image in pixels.
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image in pixels.
    ///
    /// # Returns
    /// The height of the image in pixels.
    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the pixel format of the image.
    ///
    /// # Returns
    /// The pixel format of the image.
    #[inline]
    #[must_use]
    pub fn format(&self) -> PixelFormat {
        self.format
    }
}

impl fmt::Debug for ImageRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The pixel data is left out: it is large and says nothing about the image's shape.
        f.debug_struct("ImageRef")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("format", &self.format)
            .field("bytes_per_row", &self.bytes_per_row)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::ErrorKind;

    #[test]
    fn test_from_rgb8() {
        // Arrange
        let pixels = [0; 2 * 3 * 3];

        // Act
        let actual = ImageRef::from_rgb8(&pixels, 2, 3).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Rgb8);
    }

    #[test]
    fn test_from_rgba8() {
        // Arrange
        let pixels = [0; 2 * 3 * 4];

        // Act
        let actual = ImageRef::from_rgba8(&pixels, 2, 3).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), PixelFormat::Rgba8);
    }

    #[rstest]
    #[case::luma8(PixelFormat::Luma8)]
    #[case::luma_a8(PixelFormat::LumaA8)]
    #[case::rgb8(PixelFormat::Rgb8)]
    #[case::rgba8(PixelFormat::Rgba8)]
    fn test_new(#[case] format: PixelFormat) {
        // Arrange
        let pixels = vec![0; 2 * 3 * format.bytes_per_pixel()];

        // Act
        let actual = ImageRef::new(&pixels, 2, 3, format).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 3);
        assert_eq!(actual.format(), format);
    }

    #[rstest]
    #[case::empty(0, 0)]
    #[case::zero_width(0, 3)]
    #[case::zero_height(2, 0)]
    fn test_new_empty(#[case] width: u32, #[case] height: u32) {
        // Act
        let actual = ImageRef::new(&[], width, height, PixelFormat::Rgba8).unwrap();

        // Assert
        assert_eq!(actual.width(), width);
        assert_eq!(actual.height(), height);
    }

    #[rstest]
    #[case::too_short(15, "expected 16 bytes of pixel data for a 2x2 Rgba8 image, found 15")]
    #[case::too_long(17, "expected 16 bytes of pixel data for a 2x2 Rgba8 image, found 17")]
    fn test_new_data_length_mismatch(#[case] length: usize, #[case] message: &str) {
        // Arrange
        let pixels = vec![0; length];

        // Act
        let actual = ImageRef::new(&pixels, 2, 2, PixelFormat::Rgba8);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidImage);
        assert_eq!(error.to_string(), message);
    }

    #[test]
    fn test_new_image_too_large() {
        // Act
        let actual = ImageRef::new(&[], u32::MAX, u32::MAX, PixelFormat::Rgba8);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidImage);
        assert_eq!(
            error.to_string(),
            "a 4294967295x4294967295 Rgba8 image is too large for this platform"
        );
    }

    #[rstest]
    #[case::exact(2, 3, 8 * 2 + 6)]
    #[case::padded_last_row(2, 3, 8 * 3)]
    #[case::sub_region(2, 3, 100)]
    #[case::zero_height(2, 0, 0)]
    #[case::zero_width(0, 3, 8 * 2)]
    fn test_with_stride(#[case] width: u32, #[case] height: u32, #[case] length: usize) {
        // Arrange: RGB pixels (3 bytes each), rows 8 bytes apart.
        let pixels = vec![0; length];

        // Act
        let actual = ImageRef::with_stride(&pixels, width, height, 8, PixelFormat::Rgb8).unwrap();

        // Assert
        assert_eq!(actual.width(), width);
        assert_eq!(actual.height(), height);
        assert_eq!(actual.format(), PixelFormat::Rgb8);
    }

    #[test]
    fn test_with_stride_row_stride_too_short() {
        // Arrange
        let pixels = [0; 15];

        // Act
        let actual = ImageRef::with_stride(&pixels, 2, 3, 5, PixelFormat::Rgb8);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidImage);
        assert_eq!(
            error.to_string(),
            "bytes per row (5) is less than one row of pixels (6 bytes)"
        );
    }

    #[test]
    fn test_with_stride_data_too_short() {
        // Arrange
        let pixels = [0; 8 * 2 + 5];

        // Act
        let actual = ImageRef::with_stride(&pixels, 2, 3, 8, PixelFormat::Rgb8);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidImage);
        assert_eq!(
            error.to_string(),
            "expected at least 22 bytes of pixel data for a 2x3 Rgb8 image, found 21"
        );
    }

    #[rstest]
    #[case::multiplication_overflow(3, "a 1x3 Rgb8 image is too large for this platform")]
    #[case::addition_overflow(2, "a 1x2 Rgb8 image is too large for this platform")]
    fn test_with_stride_image_too_large(#[case] height: u32, #[case] message: &str) {
        // Act: `usize::MAX * (height - 1)` overflows for height 3; for height 2 it does not,
        // but adding the last row does.
        let actual = ImageRef::with_stride(&[], 1, height, usize::MAX, PixelFormat::Rgb8);

        // Assert
        let error = actual.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::InvalidImage);
        assert_eq!(error.to_string(), message);
    }

    #[test]
    fn test_debug_omits_pixels() {
        // Arrange
        let pixels = [12, 34, 56, 78];
        let image = ImageRef::from_rgba8(&pixels, 1, 1).unwrap();

        // Act
        let actual = format!("{image:?}");

        // Assert
        assert_eq!(
            actual,
            "ImageRef { width: 1, height: 1, format: Rgba8, bytes_per_row: 4, .. }"
        );
    }
}
