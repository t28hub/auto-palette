use super::position::Position;
use crate::next::color::{AlphaColor, Oklab};

/// The weighted mean color and position of a group of pixels, kept as running sums.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Centroid {
    /// The weighted sums of L, a, and b.
    color: [f64; 3],
    /// The weighted sums of x and y.
    position: [f64; 2],
    /// The sum of the weights.
    weight: f64,
}

impl Centroid {
    /// Adds a pixel to the group, weighted by its opacity.
    ///
    /// # Arguments
    /// * `pixel` - The color and opacity of the pixel.
    /// * `position` - The position of the pixel.
    #[inline]
    pub(crate) fn add(&mut self, pixel: AlphaColor<Oklab>, position: Position) {
        let (color, weight) = (pixel.color(), pixel.alpha());
        self.color[0] += color.l * weight;
        self.color[1] += color.a * weight;
        self.color[2] += color.b * weight;
        self.position[0] += position.x * weight;
        self.position[1] += position.y * weight;
        self.weight += weight;
    }

    /// Returns the weighted mean color.
    ///
    /// # Returns
    /// The mean color, or black if no pixel has been added.
    #[inline]
    pub(crate) fn color(&self) -> Oklab {
        let [l, a, b] = self.color.map(|sum| self.mean(sum));
        Oklab::new(l, a, b)
    }

    /// Returns the weighted mean position.
    ///
    /// # Returns
    /// The mean position, or the origin if no pixel has been added.
    #[inline]
    pub(crate) fn position(&self) -> Position {
        let [x, y] = self.position.map(|sum| self.mean(sum));
        Position::new(x, y)
    }

    /// Returns the sum of the weights.
    ///
    /// # Returns
    /// The total weight of the pixels added so far.
    #[inline]
    pub(crate) const fn weight(&self) -> f64 {
        self.weight
    }

    /// Divides a weighted sum by the total weight.
    ///
    /// # Arguments
    /// * `sum` - The weighted sum.
    ///
    /// # Returns
    /// The weighted mean, or 0.0 if no pixel has been added.
    #[inline]
    fn mean(&self, sum: f64) -> f64 {
        if self.weight > 0.0 {
            sum / self.weight
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    /// A centroid of a dark pixel with opacity 0.25 and a light pixel with opacity 0.75.
    fn two_pixels() -> Centroid {
        let dark = AlphaColor::new(Oklab::new(0.2, 0.0, 0.0), 0.25).unwrap();
        let light = AlphaColor::new(Oklab::new(0.6, 0.0, 0.0), 0.75).unwrap();
        let mut centroid = Centroid::default();
        centroid.add(dark, Position::new(0.0, 0.0));
        centroid.add(light, Position::new(0.4, 0.8));
        centroid
    }

    #[test]
    fn test_color() {
        // Act
        let actual = two_pixels().color();

        // Assert
        assert_approx_eq!(actual.l, 0.5);
    }

    #[test]
    fn test_position() {
        // Act
        let actual = two_pixels().position();

        // Assert
        assert_approx_eq!(actual.x, 0.3);
        assert_approx_eq!(actual.y, 0.6);
    }

    #[test]
    fn test_weight() {
        // Act
        let actual = two_pixels().weight();

        // Assert
        assert_eq!(actual, 1.0);
    }

    #[test]
    fn test_empty_weight() {
        // Act
        let actual = Centroid::default().weight();

        // Assert
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_empty_position() {
        // Act
        let actual = Centroid::default().position();

        // Assert: the origin, as no pixel has been added.
        assert_eq!(actual, Position::new(0.0, 0.0));
    }
}
