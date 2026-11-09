use super::Size;

/// An image that owns its pixels, which are colors.
pub(crate) struct ImageBuf<P> {
    /// The size of the image.
    size: Size,
    /// The pixels, row by row. There are always `size.width * size.height` of them.
    pixels: Vec<P>,
}

impl<P> ImageBuf<P> {
    /// Creates an image from its pixels.
    ///
    /// # Arguments
    /// * `size` - The size of the image.
    /// * `pixels` - The pixels, row by row.
    ///
    /// # Returns
    /// An image of `size` with `pixels`.
    ///
    /// # Panics
    /// Panics if the number of `pixels` does not match `size`.
    pub(crate) fn new(size: Size, pixels: Vec<P>) -> Self {
        assert_eq!(
            pixels.len(),
            size.pixel_count(),
            "the number of pixels does not match the size"
        );
        Self { size, pixels }
    }

    /// Returns the size of the image.
    ///
    /// # Returns
    /// The width and height of the image in pixels.
    pub(crate) fn size(&self) -> Size {
        self.size
    }

    /// Returns the pixels of the image.
    ///
    /// # Returns
    /// The pixels, row by row.
    pub(crate) fn pixels(&self) -> &[P] {
        &self.pixels
    }

    /// Converts each pixel and keeps the size.
    ///
    /// # Arguments
    /// * `f` - The function that converts a pixel.
    ///
    /// # Returns
    /// An image of the same size with the converted pixels.
    pub(crate) fn map<Q>(self, f: impl FnMut(P) -> Q) -> ImageBuf<Q> {
        ImageBuf {
            size: self.size,
            pixels: self.pixels.into_iter().map(f).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::empty(Size::new(0, 0), vec![])]
    #[case::zero_width(Size::new(0, 3), vec![])]
    #[case::pixels(Size::new(2, 1), vec![1, 2])]
    fn test_new(#[case] size: Size, #[case] pixels: Vec<u8>) {
        // Act
        let actual = ImageBuf::new(size, pixels.clone());

        // Assert
        assert_eq!(actual.size(), size);
        assert_eq!(actual.pixels(), pixels);
    }

    #[test]
    #[should_panic(expected = "the number of pixels does not match the size")]
    fn test_new_pixel_count_mismatch() {
        // Act
        let _ = ImageBuf::new(Size::new(2, 2), vec![0; 3]);
    }

    #[test]
    fn test_map() {
        // Arrange
        let image = ImageBuf::new(Size::new(2, 1), vec![1, 2]);

        // Act
        let actual = image.map(|pixel| pixel * 10);

        // Assert
        assert_eq!(actual.size(), Size::new(2, 1));
        assert_eq!(actual.pixels(), [10, 20]);
    }
}
