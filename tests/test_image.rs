use auto_palette::{ImageData, Pixel};
use image::{DynamicImage, GenericImageView};

/// Struct representing an image data wrapper for testing.
pub struct TestImageData {
    image: DynamicImage,
}

impl TestImageData {
    /// Creates a new `TestImageData`.
    ///
    /// # Arguments
    /// * `image` - A `DynamicImage` instance.
    ///
    /// # Returns
    /// A new `TestImageData`.
    pub fn new(image: DynamicImage) -> Self {
        Self { image }
    }
}

impl ImageData for TestImageData {
    fn width(&self) -> u32 {
        self.image.width()
    }

    fn height(&self) -> u32 {
        self.image.height()
    }

    #[inline]
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        if x <= self.image.width() && y <= self.image.height() {
            let pixel = self.image.get_pixel(x, y);
            Some(pixel.0)
        } else {
            None
        }
    }
}
