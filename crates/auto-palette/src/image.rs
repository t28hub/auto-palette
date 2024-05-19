use std::borrow::Cow;
#[cfg(feature = "image")]
use std::path::Path;

#[cfg(feature = "image")]
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::Error;

/// The image data representing the pixel data of an image.
///
/// Each pixel is represented by 4 bytes in RGBA (Red, Green, Blue, Alpha) format.
/// The pixel data is stored in a linear array of bytes, where each pixel is represented by 4 bytes.
///
/// # Example
/// ```
/// #[cfg(feature = "image")]
/// {
///     use auto_palette::ImageData;
///
///     let pixels = [
///         255, 0, 0, 255, // Red
///         0, 255, 0, 255, // Green
///         0, 0, 255, 255, // Blue
///         0, 0, 0, 255, // Black
///     ];
///     let image_data = ImageData::new(2, 2, &pixels).unwrap();
///     assert_eq!(image_data.width(), 2);
///     assert_eq!(image_data.height(), 2);
///     assert_eq!(image_data.data(), &pixels);
/// }
/// ```
#[derive(Debug)]
pub struct ImageData<'a> {
    width: u32,
    height: u32,
    data: Cow<'a, [u8]>,
}

impl<'a> ImageData<'a> {
    /// Creates a new `ImageData` with the given width, height, and pixel data.
    ///
    /// # Arguments
    /// * `width` - The width of the image data.
    /// * `height` - The height of the image data.
    /// * `data` - The pixel data of the image data.
    ///
    /// # Returns
    /// The `ImageData` with the given width, height, and pixel data.
    ///
    /// # Errors
    /// Returns an error if the length of the pixel data is not equal to `width * height * 4`.
    pub fn new(width: u32, height: u32, data: &'a [u8]) -> Result<Self, Error> {
        if data.len() as u32 != width * height * 4 {
            return Err(Error::InvalidImageData);
        }

        Ok(Self {
            width,
            height,
            data: Cow::Borrowed(data),
        })
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
        let image = image::open(path).map_err(|cause| Error::ImageLoadError { cause })?;
        Self::try_from(&image)
    }

    /// Returns the width of the image data.
    ///
    /// # Returns
    /// The width of the image data.
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image data.
    ///
    /// # Returns
    /// The height of the image data.
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
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
}

#[cfg(feature = "image")]
impl TryFrom<&DynamicImage> for ImageData<'_> {
    type Error = Error;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        match image {
            DynamicImage::ImageRgb8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgba8(image) => Ok(Self::from(image)),
            _ => Err(Error::UnsupportedImage),
        }
    }
}

#[cfg(feature = "image")]
impl From<&RgbImage> for ImageData<'_> {
    fn from(image: &RgbImage) -> Self {
        let (width, height) = image.dimensions();
        let size = (width * height) as usize;
        let data = image
            .pixels()
            .fold(Vec::with_capacity(size * 4), |mut pixels, pixel| {
                pixels.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
                pixels
            });
        Self {
            width,
            height,
            data: data.into(),
        }
    }
}

#[cfg(feature = "image")]
impl From<&RgbaImage> for ImageData<'_> {
    fn from(image: &RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let data = image.to_vec();
        Self {
            width,
            height,
            data: data.into(),
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
        let actual = ImageData::new(2, 2, &pixels).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 2);
        assert_eq!(actual.data(), &pixels);
    }

    #[test]
    fn test_new_empty_data() {
        // Arrange
        let pixels = [];

        // Act
        let actual = ImageData::new(0, 0, &pixels);

        // Assert
        assert!(actual.is_ok());
    }

    #[test]
    fn test_new_invalid_data() {
        // Arrange
        let pixels = [255, 255, 255, 255];

        // Act
        let actual = ImageData::new(2, 2, &pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_rgba_image() {
        // Act
        let actual = ImageData::load("../../gfx/olympic_logo.png").unwrap();

        // Assert
        assert_eq!(actual.width(), 320);
        assert_eq!(actual.height(), 213);
        assert_eq!(actual.data().len(), 320 * 213 * 4);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_rgb_image() {
        // Act
        let actual = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

        // Assert
        assert_eq!(actual.width(), 480);
        assert_eq!(actual.height(), 722);
        assert_eq!(actual.data().len(), 480 * 722 * 4);
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_invalid_path() {
        // Act
        let actual = ImageData::load("../../gfx/unknown.png");

        // Assert
        assert!(actual.is_err());
    }

    #[cfg(feature = "image")]
    #[test]
    fn test_load_invalid_file() {
        // Act
        let actual = ImageData::load("../../gfx/colors/invalid.jpg");

        // Assert
        assert!(actual.is_err());
    }
}
