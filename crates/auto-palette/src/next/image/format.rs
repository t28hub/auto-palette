/// The pixel layout of 8-bit image data.
///
/// Each channel takes one byte, in the order the name lists them. Color values are
/// gamma-encoded sRGB. Alpha is straight (not premultiplied), the same as the HTML canvas
/// `ImageData`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PixelFormat {
    /// Grayscale, opaque.
    Luma8,
    /// Grayscale and alpha.
    LumaA8,
    /// Red, green, and blue, opaque.
    Rgb8,
    /// Red, green, blue, and alpha.
    Rgba8,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel.
    ///
    /// # Returns
    /// The number of bytes per pixel.
    #[inline]
    #[must_use]
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Luma8 => 1,
            Self::LumaA8 => 2,
            Self::Rgb8 => 3,
            Self::Rgba8 => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::luma8(PixelFormat::Luma8, 1)]
    #[case::luma_a8(PixelFormat::LumaA8, 2)]
    #[case::rgb8(PixelFormat::Rgb8, 3)]
    #[case::rgba8(PixelFormat::Rgba8, 4)]
    fn test_bytes_per_pixel(#[case] format: PixelFormat, #[case] expected: usize) {
        // Act
        let actual = format.bytes_per_pixel();

        // Assert
        assert_eq!(actual, expected);
    }
}
