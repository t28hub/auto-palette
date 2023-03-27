use crate::image::error::ImageError;

/// Type alias for RGBA pixel.
pub type Pixel = [u8; 4];

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

    /// Returns the RGBA pixel located at the coordinates (x, y).
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    ///
    /// # Returns
    /// An `Option` containing a pixel if the specified coordinates are within the bounds of the image.
    #[must_use]
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel>;
}

/// Struct representing an `ImageData` stored as bytes.
#[derive(Debug, PartialEq)]
pub struct SimpleImageData {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl SimpleImageData {
    /// Creates a new `BytesImageData` instance.
    ///
    /// # Arguments
    /// * `data` - The byte vector containing this image data.
    /// * `width` - The width of this image.
    /// * `height` - The height of this image.
    ///
    /// # Returns
    /// A new `BytesImageData` instance.
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Result<Self, ImageError> {
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

impl ImageData for SimpleImageData {
    #[must_use]
    fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    #[must_use]
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = 4 * (x + y * self.width) as usize;
        Some([
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_simple_image_data() {
        let data = vec![0, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255];
        let image_data = SimpleImageData::new(data.clone(), 2, 2);
        assert_eq!(
            image_data.unwrap(),
            SimpleImageData {
                data: data.clone(),
                width: 2,
                height: 2,
            }
        );

        let image_data = SimpleImageData::new(data.clone(), 3, 1);
        assert_eq!(image_data, Err(ImageError::InvalidDataSize(16)));
    }
}
