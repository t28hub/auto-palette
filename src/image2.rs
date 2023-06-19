use image::{RgbImage, RgbaImage};
use ndarray::{Array, ArrayView, Ix3};

/// Struct representing an image data.
pub struct ImageData {
    width: u32,
    height: u32,
    channels: u8,
    data: Array<u8, Ix3>,
}

impl ImageData {
    /// Returns the width of the image data.
    ///
    /// # Returns
    /// The width of the image data.
    #[allow(unused)]
    #[must_use]
    fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image data.
    ///
    /// # Returns
    /// The height of the image data.
    #[allow(unused)]
    #[must_use]
    fn height(&self) -> u32 {
        self.height
    }

    /// Returns the number of channels of the image data.
    ///
    /// # Returns
    /// The number of channels of the image data.
    #[allow(unused)]
    #[must_use]
    fn channels(&self) -> u8 {
        self.channels
    }

    /// Returns the raw data of the image data.
    ///
    /// # Returns
    /// The raw data of the image data.
    #[allow(unused)]
    #[must_use]
    fn data(&self) -> ArrayView<u8, Ix3> {
        self.data.view()
    }
}

impl From<&RgbImage> for ImageData {
    #[must_use]
    fn from(value: &RgbImage) -> Self {
        let (width, height) = value.dimensions();
        let samples = value.as_flat_samples();
        let array = ArrayView::from_shape((height as usize, width as usize, 3), samples.as_slice())
            .unwrap();
        Self {
            width,
            height,
            channels: 3,
            data: array.to_owned(),
        }
    }
}

impl From<&RgbaImage> for ImageData {
    #[must_use]
    fn from(value: &RgbaImage) -> Self {
        let (width, height) = value.dimensions();
        let samples = value.as_flat_samples();
        let array = ArrayView::from_shape((height as usize, width as usize, 4), samples.as_slice())
            .unwrap();
        Self {
            width,
            height,
            channels: 4,
            data: array.to_owned(),
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
        let image_data = ImageData::from(&RgbImage::from_raw(2, 2, buffer).unwrap());
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.channels(), 3);
        assert_eq!(image_data.data().shape(), &[2, 2, 3]);
    }

    #[test]
    fn test_image_data_with_rgba_image() {
        let buffer: Vec<u8> = vec![
            0, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 0, 255, 255, 255, 255, 0, 0, 255, 255, 0,
            255, 255, 255, 255, 0, 255, 255, 255, 255, 255,
        ];
        let image_data = ImageData::from(&RgbaImage::from_raw(2, 2, buffer).unwrap());
        assert_eq!(image_data.width(), 2);
        assert_eq!(image_data.height(), 2);
        assert_eq!(image_data.channels(), 4);
        assert_eq!(image_data.data().shape(), &[2, 2, 4]);
    }
}
