/// Swatch represents a color swatch with its position and population.
///
/// # Examples
/// ```
/// use auto_palette::Swatch;
///
/// let swatch = Swatch::new((255, 0, 64), (5, 10), 384);
/// assert_eq!(swatch.color(), (255, 0, 64));
/// assert_eq!(swatch.position(), (5, 10));
/// assert_eq!(swatch.population(), 384);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swatch {
    color: (u8, u8, u8),
    position: (u32, u32),
    population: usize,
}

impl Swatch {
    /// Creates a new `Swatch` instance with the given color and population.
    ///
    /// # Arguments
    /// * `color` - The color of the swatch.
    /// * `position` - The position of the swatch.
    /// * `population` - The population of the swatch.
    ///
    /// # Returns
    /// A new `Swatch` instance.
    pub fn new(color: (u8, u8, u8), position: (u32, u32), population: usize) -> Self {
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
    pub fn color(&self) -> (u8, u8, u8) {
        self.color
    }

    /// Returns the position of this swatch.
    ///
    /// # Returns
    /// The position of this swatch.
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
        let swatch = Swatch::new((255, 0, 64), (5, 10), 384);

        // Assert
        assert_eq!(swatch.color(), (255, 0, 64));
        assert_eq!(swatch.position(), (5, 10));
        assert_eq!(swatch.population(), 384);
    }
}
