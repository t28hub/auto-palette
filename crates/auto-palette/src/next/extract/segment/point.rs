use super::position::Position;
use crate::next::{
    color::{AlphaColor, Oklab},
    image::ImageBuf,
    math::{DistanceMetric, Vector},
};

/// A color at a position with a weight: a pixel, or the mean of a group of pixels.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Point {
    /// The L, a, b, x, and y values.
    vector: Vector<5>,
    /// How much the point counts in a mean.
    weight: f64,
}

impl Point {
    /// Creates a point from a color, a position, and a weight.
    ///
    /// # Arguments
    /// * `color` - The color in OKLab.
    /// * `position` - The position in the image.
    /// * `weight` - How much the point counts in a mean.
    ///
    /// # Returns
    /// A point whose vector holds L, a, b, x, and y in this order.
    #[inline]
    pub(crate) fn new(color: Oklab, position: Position, weight: f64) -> Self {
        let vector = Vector([color.l, color.a, color.b, position.x, position.y]);
        Self::of_vector(vector, weight)
    }

    /// Creates a point from a vector of its color and position.
    ///
    /// # Arguments
    /// * `vector` - The L, a, b, x, and y values.
    /// * `weight` - How much the point counts in a mean.
    ///
    /// # Returns
    /// A point with the given vector and weight.
    #[inline]
    pub(crate) const fn of_vector(vector: Vector<5>, weight: f64) -> Self {
        Self { vector, weight }
    }

    /// Creates the point of a pixel, with its opacity as the weight.
    ///
    /// # Arguments
    /// * `image` - The image to read the pixel from.
    /// * `index` - The index of the pixel, row by row.
    ///
    /// # Returns
    /// The color, position, and opacity of the pixel.
    #[inline]
    pub(crate) fn at(image: &ImageBuf<AlphaColor<Oklab>>, index: usize) -> Self {
        let pixel = image.pixels()[index];
        Self::new(
            pixel.color(),
            Position::of_pixel(index, image.size()),
            pixel.alpha(),
        )
    }

    /// Creates the points of every pixel of an image, with positions scaled.
    ///
    /// # Arguments
    /// * `image` - The image to read the pixels from.
    /// * `scale` - The factor for positions, which weighs them against colors.
    ///
    /// # Returns
    /// The point of each pixel, row by row.
    pub(crate) fn of_pixels(image: &ImageBuf<AlphaColor<Oklab>>, scale: f64) -> Vec<Self> {
        (0..image.pixels().len())
            .map(|index| Self::at(image, index).scale_position(scale))
            .collect()
    }

    /// Returns whether the point counts for nothing in a mean.
    ///
    /// # Returns
    /// `true` if the weight is zero.
    #[inline]
    pub(crate) fn is_weightless(self) -> bool {
        self.weight == 0.0
    }

    /// Returns the point with its position scaled.
    ///
    /// # Arguments
    /// * `scale` - The factor for the position.
    ///
    /// # Returns
    /// The point with the same color and weight, and the position times `scale`.
    #[inline]
    fn scale_position(self, scale: f64) -> Self {
        let [l, a, b, x, y] = self.vector.0;
        Self::of_vector(Vector([l, a, b, x * scale, y * scale]), self.weight)
    }

    /// Returns the distance to another point over color and position.
    ///
    /// # Arguments
    /// * `other` - The point to measure the distance to.
    /// * `metric` - How to measure the distance.
    ///
    /// # Returns
    /// The distance between the two points.
    #[inline]
    pub(crate) fn distance(self, other: Self, metric: DistanceMetric) -> f64 {
        self.vector.distance(other.vector, metric)
    }

    /// Returns the color in OKLab.
    ///
    /// # Returns
    /// The L, a, and b values as a color.
    #[inline]
    pub(crate) fn color(self) -> Oklab {
        let [l, a, b, _, _] = self.vector.0;
        Oklab::new(l, a, b)
    }

    /// Returns the position, which segmenters scale with [`Self::of_pixels`].
    ///
    /// # Returns
    /// The x and y values as a position.
    #[inline]
    pub(crate) fn position(self) -> Position {
        let [_, _, _, x, y] = self.vector.0;
        Position::new(x, y)
    }

    /// Returns the color and position as one vector.
    ///
    /// # Returns
    /// The L, a, b, x, and y values.
    #[inline]
    pub(crate) const fn vector(self) -> Vector<5> {
        self.vector
    }

    /// Returns the weight with which the point counts in a mean.
    ///
    /// # Returns
    /// How much the point counts in a mean.
    #[inline]
    pub(crate) const fn weight(self) -> f64 {
        self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::extract::segment::test_helpers::{image, BLACK, WHITE},
    };

    #[test]
    fn test_at() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(WHITE)]]);

        // Act
        let actual = Point::at(&image, 1);

        // Assert
        assert_approx_eq!(actual.color().l, 1.0);
        assert_eq!(actual.position(), Position::new(0.75, 0.25));
        assert_eq!(actual.weight(), 1.0);
    }

    #[test]
    fn test_of_pixels() {
        // Arrange
        let image = image(&[[Some(BLACK), None]]);

        // Act
        let actual = Point::of_pixels(&image, 1.0);

        // Assert: one point per pixel, in row order.
        let weights: Vec<f64> = actual.iter().map(|point| point.weight()).collect();
        assert_eq!(weights, [1.0, 0.0]);
    }

    #[test]
    fn test_new() {
        // Act
        let actual = Point::new(Oklab::new(0.5, 0.1, -0.1), Position::new(0.3, 0.4), 1.0);

        // Assert: L, a, b, x, and y in order.
        assert_eq!(actual.vector(), Vector([0.5, 0.1, -0.1, 0.3, 0.4]));
    }

    #[test]
    fn test_is_weightless_transparent_pixel() {
        // Arrange
        let point = Point::at(&image(&[[None]]), 0);

        // Act
        let actual = point.is_weightless();

        // Assert
        assert!(actual);
    }

    #[test]
    fn test_is_weightless_opaque_pixel() {
        // Arrange
        let point = Point::at(&image(&[[Some(BLACK)]]), 0);

        // Act
        let actual = point.is_weightless();

        // Assert
        assert!(!actual);
    }

    #[test]
    fn test_scale_position() {
        // Arrange
        let point = Point::new(Oklab::new(0.5, 0.1, -0.1), Position::new(0.3, 0.4), 1.0);

        // Act
        let actual = point.scale_position(2.0);

        // Assert
        assert_eq!(actual.vector(), Vector([0.5, 0.1, -0.1, 0.6, 0.8]));
    }

    #[test]
    fn test_distance() {
        // Arrange
        let black = Point::new(BLACK, Position::new(0.0, 0.0), 1.0);
        let white = Point::new(WHITE, Position::new(0.3, 0.4), 1.0);

        // Act
        let actual = black.distance(white, DistanceMetric::SquaredEuclidean);

        // Assert: 1.0 in color plus 0.25 in position.
        assert_approx_eq!(actual, 1.25);
    }

    #[test]
    fn test_color() {
        // Arrange
        let point = Point::of_vector(Vector([0.5, 0.1, -0.1, 0.3, 0.4]), 1.0);

        // Act
        let actual = point.color();

        // Assert
        assert_eq!([actual.l, actual.a, actual.b], [0.5, 0.1, -0.1]);
    }

    #[test]
    fn test_position() {
        // Arrange
        let point = Point::of_vector(Vector([0.5, 0.1, -0.1, 0.3, 0.4]), 1.0);

        // Act
        let actual = point.position();

        // Assert
        assert_eq!(actual, Position::new(0.3, 0.4));
    }
}
