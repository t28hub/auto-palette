use std::path::Path;

use image::ImageError::{IoError, Unsupported};
use image::{DynamicImage, RgbImage, RgbaImage};

use crate::image::error::ImageError;
use crate::image::ImageError::InvalidParameter;

/// ImageData represents the raw data of an image.
#[derive(Debug)]
pub struct ImageData {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl ImageData {
    /// Creates a new ImageData with the given width, height, and pixels.
    ///
    /// # Arguments
    /// * `width` - The width of the image data.
    /// * `height` - The height of the image data.
    /// * `pixels` - The raw data of the image data. The data should be in RGBA format.
    ///
    /// # Returns
    /// A new ImageData.
    ///
    /// # Errors
    /// Returns an error if the pixels are empty or the length of the pixels is not a multiple of the width and height.
    ///
    /// # Examples
    /// ```
    /// use auto_palette::image::ImageData;
    ///
    /// let image_data = ImageData::new(1, 2, vec![0, 0, 0, 255, 255, 255, 255, 255]).unwrap();
    /// assert_eq!(image_data.width(), 1);
    /// assert_eq!(image_data.height(), 2);
    /// assert_eq!(image_data.pixels(), &[0, 0, 0, 255, 255, 255, 255, 255]);
    /// ```
    pub fn new(width: u32, height: u32, pixels: Vec<u8>) -> Result<Self, ImageError> {
        if pixels.is_empty() || pixels.len() % (width * height * 4) as usize != 0 {
            return Err(InvalidParameter);
        }
        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    /// Opens an image file and returns the image data.
    ///
    /// # Arguments
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    /// The result of the image data.
    ///
    /// # Errors
    /// Returns an error if the image file is not supported or an I/O error occurred.
    ///
    /// # Examples
    /// ```
    /// use auto_palette::image::ImageData;
    ///
    /// let image_data = ImageData::open("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();    ///
    /// assert_eq!(image_data.width(), 640);
    /// assert_eq!(image_data.height(), 962);
    /// assert_eq!(image_data.pixels().len(), 2462720);
    /// ```
    pub fn open<P>(path: P) -> Result<Self, ImageError>
    where
        P: AsRef<Path>,
    {
        let image = image::open(&path).map_err(|error| match error {
            Unsupported(error) => ImageError::UnsupportedFile(error),
            IoError(error) => ImageError::IoError(error),
            error => ImageError::Unknown(error),
        })?;
        Self::try_from(&image)
    }

    /// Returns the width of the image data.
    ///
    /// # Returns
    /// The width of the image data.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image data.
    ///
    /// # Returns
    /// The height of the image data.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the raw data of the image data.
    ///
    /// # Returns
    /// The raw data of the image data. The data is in RGBA format.
    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

impl TryFrom<&DynamicImage> for ImageData {
    type Error = ImageError;

    fn try_from(image: &DynamicImage) -> Result<Self, Self::Error> {
        match image {
            DynamicImage::ImageRgb8(image) => Ok(Self::from(image)),
            DynamicImage::ImageRgba8(image) => Ok(Self::from(image)),
            _ => Err(ImageError::UnsupportedType(image.color())),
        }
    }
}

impl From<&RgbImage> for ImageData {
    fn from(image: &RgbImage) -> Self {
        let (width, height) = image.dimensions();
        let size = (width * height) as usize;
        let pixels = image
            .pixels()
            .fold(Vec::with_capacity(size * 4), |mut pixels, pixel| {
                pixels.extend_from_slice(&[pixel[0], pixel[1], pixel[2], 255]);
                pixels
            });
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl From<&RgbaImage> for ImageData {
    fn from(image: &RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let pixels = image.to_vec();
        Self {
            width,
            height,
            pixels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_new() {
        let pixels = vec![
            0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let image_data = ImageData::new(2, 2, pixels.clone()).unwrap();
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.pixels(), &pixels);
    }

    #[test]
    fn test_new_with_empty_pixels() {
        let image_data = ImageData::new(2, 2, vec![]);
        assert!(image_data.is_err());
    }

    #[test]
    fn test_new_with_invalid_parameters() {
        let image_data = ImageData::new(2, 2, vec![0, 0, 0, 255, 255, 255, 255]);
        assert!(image_data.is_err());
    }

    #[test]
    fn test_open() {
        let image_data =
            ImageData::open("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
        assert_eq!(image_data.width(), 640);
        assert_eq!(image_data.height(), 962);
        assert_eq!(image_data.pixels().len(), 640 * 962 * 4);
    }

    #[test]
    fn test_open_with_rgba_image() {
        let image_data = ImageData::open("./tests/assets/flag_np.png").unwrap();
        assert_eq!(image_data.width(), 394);
        assert_eq!(image_data.height(), 480);
        assert_eq!(image_data.pixels().len(), 394 * 480 * 4);
    }

    #[test]
    fn test_open_with_invalid_path() {
        let image_data = ImageData::open("./tests/assets/invalid.jpg");
        assert!(image_data.is_err());
    }

    #[test]
    fn test_open_with_invalid_file() {
        let image_data = ImageData::open("../../tests/assets/empty.txt");
        assert!(image_data.is_err());
    }
}
