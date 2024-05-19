use crate::{color::Color, math::FloatNumber};

/// The swatch representation containing the color, position, and population.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use auto_palette::{
///     color::{Color, RGB},
///     Swatch,
/// };
///
/// let color: Color<f32> = Color::from_str("#ff0040").unwrap();
/// let swatch = Swatch::new(color, (5, 10), 384);
/// assert_eq!(swatch.color().to_rgb(), RGB::new(255, 0, 64));
/// assert_eq!(swatch.position(), (5, 10));
/// assert_eq!(swatch.population(), 384);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swatch<T>
where
    T: FloatNumber,
{
    color: Color<T>,
    position: (u32, u32),
    population: usize,
}

impl<T> Swatch<T>
where
    T: FloatNumber,
{
    /// Creates a new `Swatch` instance with the given color and population.
    ///
    /// # Arguments
    /// * `color` - The color of the swatch.
    /// * `position` - The position of the swatch.
    /// * `population` - The population of the swatch.
    ///
    /// # Returns
    /// A new `Swatch` instance.
    pub fn new(color: Color<T>, position: (u32, u32), population: usize) -> Self {
        Self {
            color,
            position,
            population,
        }
    }

    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// The color of this swatch.
    #[must_use]
    pub fn color(&self) -> &Color<T> {
        &self.color
    }

    /// Returns the position of this swatch.
    ///
    /// # Returns
    /// The position of this swatch.
    /// The position is a tuple of the x and y coordinates.
    #[must_use]
    pub fn position(&self) -> (u32, u32) {
        self.position
    }

    /// Returns the population of this swatch.
    ///
    /// # Returns
    /// The population of this swatch.
    #[must_use]
    pub fn population(&self) -> usize {
        self.population
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_swatch() {
        // Act
        let color = Color::new(80.0, 0.0, 0.0);
        let swatch = Swatch::new(color.clone(), (5, 10), 384);

        // Assert
        assert_eq!(swatch.color(), &color);
        assert_eq!(swatch.position(), (5, 10));
        assert_eq!(swatch.population(), 384);
    }
}
