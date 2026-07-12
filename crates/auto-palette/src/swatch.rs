use std::fmt::{Display, Formatter};

use crate::color::Color;

/// A pixel position within an image.
///
/// # Examples
/// ```
/// use auto_palette::Position;
///
/// let position = Position::new(5, 10);
/// assert_eq!(position.x, 5);
/// assert_eq!(position.y, 10);
/// assert_eq!(position.to_string(), "(5, 10)");
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position {
    /// The x-coordinate (column) of the position.
    pub x: u32,
    /// The y-coordinate (row) of the position.
    pub y: u32,
}

impl Position {
    /// Creates a new `Position` instance.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate (column) of the position.
    /// * `y` - The y-coordinate (row) of the position.
    ///
    /// # Returns
    /// A new `Position` instance.
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// The swatch representation containing the color, position, and population.
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use auto_palette::{
///     color::{Color, Rgb},
///     Swatch,
/// };
///
/// let color = Color::from_str("#ff0040").unwrap();
/// let swatch = Swatch::new(color, (5, 10), 384, 0.25);
/// assert_eq!(swatch.color().to_rgb(), Rgb::new(255, 0, 64));
/// assert_eq!(swatch.position().x, 5);
/// assert_eq!(swatch.position().y, 10);
/// assert_eq!(swatch.population(), 384);
/// assert_eq!(swatch.ratio(), 0.25);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swatch {
    color: Color,
    position: Position,
    population: usize,
    ratio: f64,
}

impl Swatch {
    /// Creates a new `Swatch` instance with the given color and population.
    ///
    /// # Arguments
    /// * `color` - The color of the swatch.
    /// * `position` - The representative position of the swatch. Accepts a
    ///   [`Position`] or an `(x, y)` tuple.
    /// * `population` - The number of pixels represented by the swatch.
    /// * `ratio` - The fraction of the image covered by the swatch, in `0.0..=1.0`.
    ///
    /// # Returns
    /// A new `Swatch` instance.
    pub fn new(color: Color, position: impl Into<Position>, population: usize, ratio: f64) -> Self {
        Self {
            color,
            position: position.into(),
            population,
            ratio,
        }
    }

    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// The color of this swatch.
    #[inline]
    #[must_use]
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Returns the representative position of this swatch.
    ///
    /// # Returns
    /// The position of this swatch.
    #[inline]
    #[must_use]
    pub fn position(&self) -> Position {
        self.position
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The number of pixels represented by this swatch.
    #[inline]
    #[must_use]
    pub fn population(&self) -> usize {
        self.population
    }

    /// Returns the ratio of this swatch to the total population.
    ///
    /// # Returns
    /// The fraction of the image covered by this swatch, in `0.0..=1.0`.
    #[inline]
    #[must_use]
    pub fn ratio(&self) -> f64 {
        self.ratio
    }
}

impl Default for Swatch {
    fn default() -> Self {
        Self {
            color: Color::default(),
            position: Position::default(),
            population: 0,
            ratio: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        // Act
        let position = Position::new(5, 10);

        // Assert
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 10);
        assert_eq!(position, Position::from((5, 10)));
        assert_eq!(position.to_string(), "(5, 10)");
    }

    #[test]
    fn test_new_swatch() {
        // Act
        let color = Color::new(80.0, 0.0, 0.0);
        let swatch = Swatch::new(color, (5, 10), 384, 0.25);

        // Assert
        assert_eq!(swatch.color(), &color);
        assert_eq!(swatch.position(), Position::new(5, 10));
        assert_eq!(swatch.population(), 384);
        assert_eq!(swatch.ratio(), 0.25);
    }

    #[test]
    fn test_default() {
        // Act
        let swatch = Swatch::default();

        // Assert
        assert_eq!(
            swatch,
            Swatch {
                color: Color::default(),
                position: Position::default(),
                population: 0,
                ratio: 0.0,
            }
        )
    }
}
