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
