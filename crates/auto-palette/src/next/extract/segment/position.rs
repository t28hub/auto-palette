use crate::next::image::Size;

/// A position in an image, scaled so that the longer side spans 0.0 to 1.0.
///
/// Both axes use the same scale, so one pixel is the same distance across and down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Position {
    /// The horizontal position, from 0.0 (left).
    pub(crate) x: f64,
    /// The vertical position, from 0.0 (top).
    pub(crate) y: f64,
}

impl Position {
    /// Creates a position from its horizontal and vertical values.
    ///
    /// # Arguments
    /// * `x` - The horizontal position.
    /// * `y` - The vertical position.
    ///
    /// # Returns
    /// A position with the given `x` and `y` values.
    #[inline]
    pub(crate) const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Returns the position of the center of a pixel.
    ///
    /// # Arguments
    /// * `index` - The index of the pixel, row by row.
    /// * `size` - The size of the image, with at least one pixel.
    ///
    /// # Returns
    /// The position of the center of the pixel.
    #[inline]
    pub(crate) fn of_pixel(index: usize, size: Size) -> Self {
        let width = size.width as usize;
        let pixel = Self::pixel_length(size);
        let (column, row) = (index % width, index / width);
        Self::new((column as f64 + 0.5) * pixel, (row as f64 + 0.5) * pixel)
    }

    /// Returns the length of one pixel.
    ///
    /// # Arguments
    /// * `size` - The size of the image, with at least one pixel.
    ///
    /// # Returns
    /// The distance between the centers of two adjacent pixels.
    #[inline]
    pub(crate) fn pixel_length(size: Size) -> f64 {
        1.0 / f64::from(size.width.max(size.height))
    }

    /// Returns the length of one segment when `count` square segments tile an image.
    ///
    /// # Arguments
    /// * `size` - The size of the image, with at least one pixel.
    /// * `count` - The number of segments, at least one.
    ///
    /// # Returns
    /// The distance between the centers of adjacent segments.
    #[inline]
    pub(crate) fn segment_length(size: Size, count: usize) -> f64 {
        let pixels_per_segment = size.pixel_count() as f64 / count as f64;
        let pixels_per_side = pixels_per_segment.sqrt();
        pixels_per_side * Self::pixel_length(size)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::first(0, Position::new(0.125, 0.125))]
    #[case::last_column(3, Position::new(0.875, 0.125))]
    #[case::second_row(4, Position::new(0.125, 0.375))]
    fn test_of_pixel(#[case] index: usize, #[case] expected: Position) {
        // Act
        let actual = Position::of_pixel(index, Size::new(4, 2));

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_of_pixel_uses_one_scale_for_both_axes() {
        // Arrange: the image is 4 pixels wide and 2 high, so both axes are divided by 4.
        let size = Size::new(4, 2);

        // Act
        let right = Position::of_pixel(1, size);
        let below = Position::of_pixel(4, size);

        // Assert
        let origin = Position::of_pixel(0, size);
        assert_eq!(right.x - origin.x, below.y - origin.y);
    }

    #[rstest]
    #[case::landscape(Size::new(4, 2))]
    #[case::portrait(Size::new(2, 4))]
    fn test_pixel_length(#[case] size: Size) {
        // Act
        let actual = Position::pixel_length(size);

        // Assert: one pixel of the longer side.
        assert_eq!(actual, 0.25);
    }

    #[test]
    fn test_segment_length() {
        // Act: two segments of 2x2 pixels in a 4x2 image.
        let actual = Position::segment_length(Size::new(4, 2), 2);

        // Assert: two pixels of 0.25 each.
        assert_eq!(actual, 0.5);
    }
}
