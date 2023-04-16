use crate::image::error::ImageError;

/// Trait representing an image data.
pub trait ImageData {
    /// Returns the width of this image.
    ///
    /// # Returns
    /// The width of this image.
    #[must_use]
    fn width(&self) -> u32;

    /// Returns the height of this image.
    ///
    /// # Returns
    /// The height of this image.
    #[must_use]
    fn height(&self) -> u32;

    /// Returns the raw data of this image.
    ///
    /// # Returns
    /// The raw data of this image.
    fn data(&self) -> &[u8];
}

/// Struct representing simple image data implementation.
///
/// # Examples
/// ```no_run
/// extern crate image;
/// use auto_palette::{ImageData, SimpleImageData};
///
/// let img = image::open("/path/to/image.png").unwrap();
/// let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();
/// ```
#[derive(Debug, PartialEq)]
pub struct SimpleImageData<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> SimpleImageData<'a> {
    /// Creates a new `SimpleImageData` instance.
    ///
    /// # Arguments
    /// * `width` - The width of this image.
    /// * `height` - The height of this image.
    /// * `data` - The raw data of this image.
    ///
    /// # Returns
    /// A new `SimpleImageData` instance.
    pub fn new(width: u32, height: u32, data: &'a [u8]) -> Result<Self, ImageError> {
        let size = (width * height * 4) as usize;
        if data.len() == size {
            Ok(Self {
                data,
                width,
                height,
            })
        } else {
            Err(ImageError::InvalidDataSize(data.len()))
        }
    }
}

impl<'a> ImageData for SimpleImageData<'a> {
    #[must_use]
    fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    fn data(&self) -> &'a [u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_image_data() {
        let data = vec![0, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255];
        let image_data = SimpleImageData::new(2, 2, &data);
        assert_eq!(
            image_data,
            Ok(SimpleImageData {
                data: &data,
                width: 2,
                height: 2,
            })
        );

        let image_data = SimpleImageData::new(3, 1, &data);
        assert_eq!(image_data, Err(ImageError::InvalidDataSize(16)));
    }
}
