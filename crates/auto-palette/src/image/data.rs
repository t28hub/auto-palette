#[cfg(feature = "image")]
use std::path::Path;

#[cfg(feature = "image")]
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::{
    error::Error,
    image::{RawImage, RGBA_CHANNELS},
};

/// The image data representing the pixel data of an image.
///
/// Each pixel is represented by 4 bytes in RGBA (Red, Green, Blue, Alpha) format.
/// The pixel data is stored in a linear array of bytes and owned by this
/// struct, so an `ImageData` can be freely sent across threads and language
/// boundaries (WebAssembly, Android, iOS).
///
/// Constructing an `ImageData` from a borrowed slice copies the buffer; pass
/// an owned `Vec<u8>` to avoid the copy, or use
/// [`PaletteBuilder::build_from_raw`](crate::PaletteBuilder::build_from_raw)
/// to extract a palette directly from a borrowed buffer without creating an
/// `ImageData` at all.
///
/// # Example
/// ```
/// use auto_palette::ImageData;
///
/// let pixels = [
///     255, 0, 0, 255, // Red
///     0, 255, 0, 255, // Green
///     0, 0, 255, 255, // Blue
///     0, 0, 0, 255, // Black
/// ];
/// let image_data = ImageData::new(2, 2, &pixels).unwrap();
/// assert_eq!(image_data.width(), 2);
/// assert_eq!(image_data.height(), 2);
/// assert_eq!(image_data.data(), &pixels);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl ImageData {
    /// Creates a new `ImageData` with the given width, height, and pixel data.
    ///
    /// # Arguments
    /// * `width` - The width of the image data.
    /// * `height` - The height of the image data.
    /// * `data` - The pixel data in RGBA order. Accepts any type convertible
    ///   into `Vec<u8>` (e.g. `Vec<u8>`, `&[u8]`, `&[u8; N]`). Passing an
    ///   owned `Vec<u8>` transfers ownership without copying; passing a
    ///   borrowed slice copies it.
    ///
    /// # Returns
    /// The `ImageData` with the given width, height, and pixel data.
    ///
    /// # Errors
    /// Returns [`Error::InvalidImageData`] if the length of the pixel data is
    /// not equal to `width * height * 4`.
    pub fn new(width: u32, height: u32, data: impl Into<Vec<u8>>) -> Result<Self, Error> {
        let data = data.into();
        let expected_length = (width * height) as usize * RGBA_CHANNELS;
        if data.len() != expected_length {
            return Err(Error::InvalidImageData {
                expected: expected_length,
                actual: data.len(),
            });
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }

    /// Creates a new `ImageData` from parts that already uphold the length
    /// invariant (`data.len() == width * height * 4`).
    pub(crate) fn from_parts(width: u32, height: u32, data: Vec<u8>) -> Self {
        debug_assert_eq!(data.len(), (width * height) as usize * RGBA_CHANNELS);
        Self {
            width,
            height,
            data,
        }
    }

    /// Loads the image data from the given path.
    /// The image data is loaded using the `image` crate.
    ///
    /// # Arguments
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    /// The image data loaded from the given path.
    ///
    /// # Errors
    /// Returns an error if the image loading process fails.
    /// Returns an error if the color type of the image is not supported.
    #[cfg(feature = "image")]
    pub fn load<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path)?;
        Self::try_from(&image)
    }

    /// Checks whether the image data is empty.
    ///
    /// # Returns
    /// `true` if the image data is empty, `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the width of the image data.
    ///
    /// # Returns
    /// The width of the image data.
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image data.
    ///
    /// # Returns
    /// The height of the image data.
    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the area of the image data.
    ///
    /// # Returns
    /// The area of the image data.
    #[must_use]
    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Returns the pixel data of the image data.
    ///
    /// Each pixel is represented by 4 bytes in RGBA (Red, Green, Blue, Alpha) format.
    ///
    /// # Returns
    /// The pixel data of the image data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns a borrowed [`RawImage`] view over this image data.
    ///
    /// # Returns
    /// A borrowed view of the pixel data.
    pub(crate) fn as_raw(&self) -> RawImage<'_> {
        RawImage::new_unchecked(self.width, self.height, &self.data)
    }
}

#[cfg(feature = "image")]
impl TryFrom<&DynamicImage> for ImageData {
    type Error = Error;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        match image {
            DynamicImage::ImageRgb8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgba8(image) => Ok(Self::from(image)),
            _ => Err(Error::UnsupportedImageFormat),
        }
    }
}

#[cfg(feature = "image")]
impl From<&RgbImage> for ImageData {
    fn from(image: &RgbImage) -> Self {
        let (width, height) = image.dimensions();
        let size = (width * height) as usize;
        let data = image.pixels().fold(
            Vec::with_capacity(size * RGBA_CHANNELS),
            |mut pixels, pixel| {
                pixels.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
                pixels
            },
        );
        Self {
            width,
            height,
            data,
        }
    }
}

#[cfg(feature = "image")]
impl From<&RgbaImage> for ImageData {
    fn from(image: &RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let data = image.to_vec();
        Self {
            width,
            height,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White
            255, 255, 255, 255, // White
            255, 255, 255, 255, // White
            255, 255, 255, 255, // White
        ];

        // Act
        let actual = ImageData::new(2, 2, pixels);

        // Assert
        assert!(actual.is_ok());

        let image_data = actual.unwrap();
        assert!(!image_data.is_empty());
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.area(), 4);
        assert_eq!(image_data.data(), &pixels);
    }

    #[test]
    fn test_new_empty_data() {
        // Arrange
        let pixels: [u8; 0] = [];

        // Act
        let actual = ImageData::new(0, 0, pixels);

        // Assert
        assert!(actual.is_ok());

        let image_data = actual.unwrap();
        assert!(image_data.is_empty());
        assert_eq!(image_data.width(), 0);
        assert_eq!(image_data.height(), 0);
        assert_eq!(image_data.area(), 0);
        assert_eq!(image_data.data(), &pixels);
    }

    #[test]
    fn test_new_unexpected_length() {
        // Arrange
        let pixels = [255, 255, 255, 255];

        // Act
        let actual = ImageData::new(2, 2, pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "unexpected data length: expected 16, got 4"
        );
    }

    #[test]
    fn test_as_raw() {
        // Arrange
        let pixels = [255u8; 2 * 2 * 4];
        let image_data = ImageData::new(2, 2, pixels).unwrap();

        // Act
        let raw = image_data.as_raw();

        // Assert
        assert_eq!(raw.width(), 2);
        assert_eq!(raw.height(), 2);
        assert_eq!(raw.area(), 4);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_supported_format() {
        // Act
        let actual = ImageData::load("../../gfx/parrots_rgba8.png");

        // Assert
        assert!(actual.is_ok());

        let image_data = actual.unwrap();
        assert!(!image_data.is_empty());
        assert_eq!(image_data.width(), 150);
        assert_eq!(image_data.height(), 150);
        assert_eq!(image_data.area(), 150 * 150);
        assert_eq!(image_data.data().len(), 150 * 150 * 4);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_unsupported_format() {
        // Act
        let actual = ImageData::load("../../gfx/parrots_la16.png");

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error.to_string(), "unsupported image format or color type");
    }

    #[cfg(all(feature = "image", not(target_os = "windows")))]
    #[test]
    fn test_load_unknown_path() {
        // Act
        let actual = ImageData::load("../../gfx/unknown.png");

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "failed to load image: No such file or directory (os error 2)"
        );
    }

    #[cfg(all(feature = "image", target_os = "windows"))]
    #[test]
    fn test_load_unknown_path_windows() {
        // Act
        let actual = ImageData::load("../../gfx/unknown.png");

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "failed to load image: The system cannot find the file specified. (os error 2)"
        );
    }
}
