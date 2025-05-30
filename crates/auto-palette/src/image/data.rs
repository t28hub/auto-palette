use std::borrow::Cow;
#[cfg(feature = "image")]
use std::path::Path;

#[cfg(feature = "image")]
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::{
    color::{rgb_to_xyz, xyz_to_lab, Lab, D65},
    image::{error::ImageError, Pixel, RGBA_CHANNELS},
    math::normalize,
    Filter,
    FloatNumber,
    ImageResult,
};

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
    pub fn new(width: u32, height: u32, data: &'a [u8]) -> ImageResult<Self> {
        let expected_length = (width * height) as usize * RGBA_CHANNELS;
        if data.len() != expected_length {
            return Err(ImageError::UnexpectedLength {
                expected: expected_length,
                actual: data.len(),
            });
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
    pub fn load<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = image::open(path).map_err(ImageError::from)?;
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

    /// Returns an iterator over the pixels of the image data.
    ///
    /// # Returns
    /// An iterator over the pixels of the image data.
    #[allow(dead_code)]
    pub(crate) fn pixels<'b, T>(&'b self) -> impl Iterator<Item = Pixel<T>> + 'b
    where
        T: FloatNumber + 'b,
    {
        self.data
            .chunks_exact(RGBA_CHANNELS)
            .enumerate()
            .map(move |(index, rgba)| self.chunk_to_pixel(index, rgba))
    }

    /// Returns an iterator over the pixels of the image data together with the result of applying the `filter`.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `F` - The filter type.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the pixels.
    ///
    /// # Returns
    /// An iterator over the pixels of the image data and the result of applying the filter.
    pub(crate) fn pixels_with_filter<'b, T, F>(
        &'b self,
        filter: &'b F,
    ) -> impl Iterator<Item = (Pixel<T>, bool)> + 'b
    where
        T: FloatNumber + 'b,
        F: Filter,
    {
        self.data
            .chunks_exact(RGBA_CHANNELS)
            .enumerate()
            .map(move |(index, chunk)| {
                (
                    self.chunk_to_pixel::<T>(index, chunk),
                    filter.test(&[chunk[0], chunk[1], chunk[2], chunk[3]]),
                )
            })
    }

    /// Converts a chunk of pixel data to a pixel representation.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Arguments
    /// * `index` - The index of the pixel in the image data.
    /// * `chunk` - The chunk of pixel data.
    ///
    /// # Returns
    /// The pixel representation of the chunk of pixel data.
    #[inline(always)]
    #[must_use]
    fn chunk_to_pixel<T>(&self, index: usize, chunk: &[u8]) -> Pixel<T>
    where
        T: FloatNumber,
    {
        let (x, y, z) = rgb_to_xyz::<T>(chunk[0], chunk[1], chunk[2]);
        let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);

        let coord_x = T::from_usize((index % self.width as usize) + 1);
        let coord_y = T::from_usize((index / self.width as usize) + 1);

        let width_f = T::from_u32(self.width);
        let height_f = T::from_u32(self.height);

        [
            Lab::<T>::normalize_l(l),
            Lab::<T>::normalize_a(a),
            Lab::<T>::normalize_b(b),
            normalize(coord_x, T::zero(), width_f),
            normalize(coord_y, T::zero(), height_f),
        ]
    }
}

#[cfg(feature = "image")]
impl TryFrom<&DynamicImage> for ImageData<'_> {
    type Error = ImageError;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        match image {
            DynamicImage::ImageRgb8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgba8(image) => Ok(Self::from(image)),
            _ => Err(ImageError::UnsupportedFormat),
        }
    }
}

#[cfg(feature = "image")]
impl From<&RgbImage> for ImageData<'_> {
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
    use crate::{assert_approx_eq, Rgba};

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
        let actual = ImageData::new(2, 2, &pixels);

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
        let pixels = [];

        // Act
        let actual = ImageData::new(0, 0, &pixels);

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
        let actual = ImageData::new(2, 2, &pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Unexpected data length - expected 16, got 4"
        );
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
        assert_eq!(error.to_string(), "Unsupported image format or color type");
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
            "Failed to load image from file: No such file or directory (os error 2)"
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
            "Failed to load image from file: The system cannot find the file specified. (os error 2)"
        );
    }

    #[cfg(all(feature = "image", not(target_os = "windows")))]
    #[test]
    fn test_load_invalid_file() {
        // Act
        let actual = ImageData::load("../../gfx/colors/invalid.jpg");

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Failed to load image from file: No such file or directory (os error 2)"
        );
    }

    #[cfg(all(feature = "image", target_os = "windows"))]
    #[test]
    fn test_load_invalid_file_windows() {
        // Act
        let actual = ImageData::load("../../gfx/colors/invalid.jpg");

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Failed to load image from file: The system cannot find the file specified. (os error 2)"
        );
    }

    #[test]
    fn test_pixels_iter() {
        // Arrange
        let pixels = [
            255, 0, 0, 255, // Red
            0, 0, 0, 0, // Transparent
            255, 255, 0, 255, // Yellow
            0, 0, 0, 0, // Transparent
        ];
        let image_data = ImageData::new(2, 2, &pixels).unwrap();

        // Act
        let actual: Vec<_> = image_data.pixels::<f64>().collect();

        // Assert
        assert_eq!(actual.len(), 4);

        let pixel = actual[0];
        assert_approx_eq!(pixel[0], 0.532371);
        assert_approx_eq!(pixel[1], 0.816032);
        assert_approx_eq!(pixel[2], 0.765488);
        assert_approx_eq!(pixel[3], 0.5);
        assert_approx_eq!(pixel[4], 0.5);

        let pixel = actual[1];
        assert_approx_eq!(pixel[0], 0.0);
        assert_approx_eq!(pixel[1], 0.501960);
        assert_approx_eq!(pixel[2], 0.501960);
        assert_approx_eq!(pixel[3], 1.0);
        assert_approx_eq!(pixel[4], 0.5);

        let pixel = actual[2];
        assert_approx_eq!(pixel[0], 0.971385);
        assert_approx_eq!(pixel[1], 0.417402);
        assert_approx_eq!(pixel[2], 0.872457);
        assert_approx_eq!(pixel[3], 0.5);
        assert_approx_eq!(pixel[4], 1.0);

        let pixel = actual[3];
        assert_approx_eq!(pixel[0], 0.0);
        assert_approx_eq!(pixel[1], 0.501960);
        assert_approx_eq!(pixel[2], 0.501960);
        assert_approx_eq!(pixel[3], 1.0);
        assert_approx_eq!(pixel[4], 1.0);
    }

    #[test]
    fn test_pixels_with_filter() {
        // Arrange
        let data = [
            255, 0, 0, 255, // Red
            0, 0, 0, 0, // Transparent
            255, 255, 0, 255, // Yellow
            0, 0, 0, 0, // Transparent
        ];
        let image_data = ImageData::new(2, 2, &data).unwrap();

        // Act
        let (pixels, mask) = image_data
            .pixels_with_filter::<f64, _>(&|rgba: &Rgba| rgba[3] != 0)
            .fold(
                (Vec::new(), Vec::new()),
                |(mut pixels, mut mask), (pixel, m)| {
                    pixels.push(pixel);
                    mask.push(m);
                    (pixels, mask)
                },
            );

        // Assert
        assert_eq!(pixels.len(), 4);
        assert_eq!(mask.len(), 4);
        assert_eq!(mask, vec![true, false, true, false]);
    }
}
