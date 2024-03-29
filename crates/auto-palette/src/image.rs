use image::{RgbImage, RgbaImage};

/// Struct representing an image data.
#[derive(Debug)]
pub struct ImageData {
    width: u32,
    height: u32,
    channels: u8,
    data: Vec<u8>,
}

impl ImageData {
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

    /// Returns the number of channels of the image data.
    ///
    /// # Returns
    /// The number of channels of the image data.
    #[must_use]
    pub fn channels(&self) -> u8 {
        self.channels
    }

    /// Returns the raw data of the image data.
    ///
    /// # Returns
    /// The raw data of the image data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl From<&RgbImage> for ImageData {
    #[must_use]
    fn from(value: &RgbImage) -> Self {
        let (width, height) = value.dimensions();
        Self {
            width,
            height,
            channels: 3,
            data: value.to_vec(),
        }
    }
}

impl From<&RgbaImage> for ImageData {
    #[must_use]
    fn from(value: &RgbaImage) -> Self {
        let (width, height) = value.dimensions();
        Self {
            width,
            height,
            channels: 4,
            data: value.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_data_with_rgb_image() {
        let buffer: Vec<u8> = vec![
            0, 0, 0, 0, 0, 255, 0, 255, 0, 0, 255, 255, 255, 0, 0, 255, 0, 255, 255, 255, 0, 255,
            255, 255,
        ];
        let image_data = ImageData::from(&RgbImage::from_raw(2, 2, buffer.clone()).unwrap());
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.channels(), 3);
        assert_eq!(image_data.data(), &buffer);
    }

    #[test]
    fn test_image_data_with_rgba_image() {
        let buffer: Vec<u8> = vec![
            0, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 255, 255, 0,
            255, 255, 255, 255, 0, 255, 255, 255, 255, 255,
        ];
        let image_data = ImageData::from(&RgbaImage::from_raw(2, 2, buffer.clone()).unwrap());
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.channels(), 4);
        assert_eq!(image_data.data(), &buffer);
    }
}
