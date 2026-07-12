#[cfg(feature = "image")]
use std::path::Path;

#[cfg(feature = "image")]
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::{
    color::{rgb_to_xyz, xyz_to_lab, Lab, D65},
    error::Error,
    image::{Pixel, RGBA_CHANNELS},
    math::{normalize, FloatNumber},
    Filter,
};

/// The image data representing the pixel data of an image.
///
/// Each pixel is represented by 4 bytes in RGBA (Red, Green, Blue, Alpha) format.
/// The pixel data is stored in a linear array of bytes and owned by this
/// struct, so an `ImageData` can be freely sent across threads and language
/// boundaries (WebAssembly, Android, iOS).
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
    ///   into `Vec<u8>` (e.g. `Vec<u8>`, `&[u8]`, `&[u8; N]`).
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

    /// Downsamples the image with nearest-neighbor sampling so that the
    /// result contains at most `max_pixels` pixels.
    ///
    /// The aspect ratio is approximately preserved. If the image already
    /// contains `max_pixels` pixels or fewer, a clone of the image is
    /// returned.
    ///
    /// # Arguments
    /// * `max_pixels` - The maximum number of pixels in the result.
    ///
    /// # Returns
    /// The downsampled image data.
    pub(crate) fn downsample(&self, max_pixels: usize) -> Self {
        let area = self.area();
        if max_pixels == 0 || area <= max_pixels {
            return self.clone();
        }

        let stride = ((area as f64 / max_pixels as f64).sqrt().ceil() as u32).max(1);
        let width = self.width.div_ceil(stride);
        let height = self.height.div_ceil(stride);

        let mut data = Vec::with_capacity((width * height) as usize * RGBA_CHANNELS);
        for row in 0..height {
            let src_row = (row * stride) as usize;
            for col in 0..width {
                let src_col = (col * stride) as usize;
                let offset = (src_row * self.width as usize + src_col) * RGBA_CHANNELS;
                data.extend_from_slice(&self.data[offset..offset + RGBA_CHANNELS]);
            }
        }
        Self {
            width,
            height,
            data,
        }
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
        let (x, y, z) = rgb_to_xyz(chunk[0], chunk[1], chunk[2]);
        let (l, a, b) = xyz_to_lab::<D65>(x, y, z);

        let coord_x = T::from_usize((index % self.width as usize) + 1);
        let coord_y = T::from_usize((index / self.width as usize) + 1);

        let width_f = T::from_u32(self.width);
        let height_f = T::from_u32(self.height);

        [
            T::from_f64(Lab::normalize_l(l)),
            T::from_f64(Lab::normalize_a(a)),
            T::from_f64(Lab::normalize_b(b)),
            normalize(coord_x, T::zero(), width_f),
            normalize(coord_y, T::zero(), height_f),
        ]
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
    fn test_downsample() {
        // Arrange
        let mut pixels = Vec::with_capacity(8 * 8 * 4);
        for index in 0..64u8 {
            pixels.extend_from_slice(&[index, index, index, 255]);
        }
        let image_data = ImageData::new(8, 8, pixels).unwrap();

        // Act
        let actual = image_data.downsample(16);

        // Assert
        assert_eq!(actual.width(), 4);
        assert_eq!(actual.height(), 4);
        assert_eq!(actual.area(), 16);
        // Nearest-neighbor sampling picks every 2nd pixel of every 2nd row.
        assert_eq!(actual.data()[0], 0);
        assert_eq!(actual.data()[4], 2);
        assert_eq!(actual.data()[16], 16);
    }

    #[test]
    fn test_downsample_no_op() {
        // Arrange
        let pixels = vec![255; 2 * 2 * 4];
        let image_data = ImageData::new(2, 2, pixels).unwrap();

        // Act
        let actual = image_data.downsample(4);

        // Assert
        assert_eq!(actual, image_data);
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

    #[test]
    fn test_pixels() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White
            0, 0, 0, 255, // Black
        ];
        let image_data = ImageData::new(2, 1, pixels).unwrap();

        // Act
        let actual: Vec<_> = image_data.pixels::<f64>().collect();

        // Assert
        assert_eq!(actual.len(), 2);
        // White pixel: L = 100 normalized to 1.0
        assert_approx_eq!(actual[0][0], 1.0, 1e-3);
        // Black pixel: L = 0 normalized to 0.0
        assert_approx_eq!(actual[1][0], 0.0, 1e-3);
        // 1-indexed normalized coordinates
        assert_approx_eq!(actual[0][3], 0.5);
        assert_approx_eq!(actual[1][3], 1.0);
    }

    #[test]
    fn test_pixels_with_filter() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White, opaque
            0, 0, 0, 0, // Black, transparent
        ];
        let image_data = ImageData::new(2, 1, pixels).unwrap();
        let filter = |rgba: &Rgba| rgba[3] != 0;

        // Act
        let actual: Vec<_> = image_data.pixels_with_filter::<f64, _>(&filter).collect();

        // Assert
        assert_eq!(actual.len(), 2);
        assert!(actual[0].1);
        assert!(!actual[1].1);
    }
}
