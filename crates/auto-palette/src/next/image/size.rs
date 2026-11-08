use std::num::NonZeroU32;

/// The size of an image in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Size {
    /// The width in pixels.
    pub(crate) width: u32,
    /// The height in pixels.
    pub(crate) height: u32,
}

impl Size {
    /// Creates a size from a width and a height in pixels.
    ///
    /// # Arguments
    /// * `width` - The width in pixels.
    /// * `height` - The height in pixels.
    ///
    /// # Returns
    /// A size of `width` × `height` pixels.
    pub(crate) const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Returns the number of pixels.
    ///
    /// # Returns
    /// `width * height`.
    pub(crate) const fn pixel_count(self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Returns whether the size has no pixels.
    ///
    /// # Returns
    /// `true` if the width or the height is zero.
    pub(crate) const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Returns the size that fits in `max_pixels` and keeps the aspect ratio.
    ///
    /// # Arguments
    /// * `max_pixels` - The largest number of pixels allowed.
    ///
    /// # Returns
    /// The scaled size, or this size if it already fits.
    pub(crate) fn fit(self, max_pixels: NonZeroU32) -> Self {
        let max = u64::from(max_pixels.get());
        let area = u64::from(self.width) * u64::from(self.height);
        if area <= max {
            return self;
        }

        let scale = (max as f64 / area as f64).sqrt();
        let scaled = |side: u32| ((f64::from(side) * scale).round() as u32).clamp(1, side);
        let mut width = scaled(self.width);
        let mut height = scaled(self.height);
        // Rounding can go over the limit, and so can a side raised to one pixel.
        if u64::from(width) * u64::from(height) > max {
            if width >= height {
                width = (max / u64::from(height)) as u32;
            } else {
                height = (max / u64::from(width)) as u32;
            }
        }
        Self::new(width, height)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::empty(Size::new(0, 0), 0)]
    #[case::zero_width(Size::new(0, 3), 0)]
    #[case::zero_height(Size::new(2, 0), 0)]
    #[case::pixels(Size::new(2, 3), 6)]
    fn test_pixel_count(#[case] size: Size, #[case] expected: usize) {
        // Act
        let actual = size.pixel_count();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty(Size::new(0, 0), true)]
    #[case::zero_width(Size::new(0, 3), true)]
    #[case::zero_height(Size::new(2, 0), true)]
    #[case::pixels(Size::new(2, 3), false)]
    fn test_is_empty(#[case] size: Size, #[case] expected: bool) {
        // Act
        let actual = size.is_empty();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty(Size::new(0, 0), Size::new(0, 0))]
    #[case::zero_width(Size::new(0, 300), Size::new(0, 300))]
    #[case::zero_height(Size::new(300, 0), Size::new(300, 0))]
    #[case::smaller(Size::new(100, 100), Size::new(100, 100))]
    #[case::exact(Size::new(256, 256), Size::new(256, 256))]
    #[case::square(Size::new(1000, 1000), Size::new(256, 256))]
    #[case::landscape(Size::new(2000, 1000), Size::new(362, 181))]
    #[case::portrait(Size::new(1000, 2000), Size::new(181, 362))]
    #[case::panorama(Size::new(4000, 100), Size::new(1619, 40))]
    #[case::one_row(Size::new(1_000_000, 1), Size::new(65_536, 1))]
    #[case::one_column(Size::new(1, 1_000_000), Size::new(1, 65_536))]
    fn test_fit(#[case] size: Size, #[case] expected: Size) {
        // Act
        let actual = size.fit(NonZeroU32::new(65_536).unwrap());

        // Assert
        assert_eq!(actual, expected);
    }
}
