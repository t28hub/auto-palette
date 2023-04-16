use crate::color_trait::Color;

/// Struct representing a swatch that contains a color and its position.
///
/// # Type Parameters
/// * `C` - The color type.
///
/// # Examples
/// ```
/// use auto_palette::Swatch;
/// use auto_palette::rgb::Rgb;
///
/// let color = Rgb::new(255, 0, 64);
/// let swatch = Swatch::new(color, (90, 120), 384);
/// assert_eq!(swatch.color(), &Rgb::new(255, 0, 64));
/// assert_eq!(swatch.position(), (90, 120));
/// assert_eq!(swatch.population(), 384);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Swatch<C: Color> {
    color: C,
    position: (u32, u32),
    population: usize,
}

impl<C> Swatch<C>
where
    C: Color,
{
    /// Creates a new `Swatch` instance.
    ///
    /// # Arguments
    /// * `color` - The color of the swatch.
    /// * `position` - The (x, y) position of the swatch.
    /// * `population` - The population of the swatch.
    ///
    /// # Returns
    /// A `Swatch` instance.
    #[must_use]
    pub fn new(color: C, position: (u32, u32), population: usize) -> Self {
        Self {
            color,
            position,
            population,
        }
    }

    /// Returns the color of this swatch.
    ///
    /// # Returns
    /// A reference of color of this swatch.
    #[must_use]
    pub fn color(&self) -> &C {
        &self.color
    }

    /// Returns the (x, y) position of this swatch.
    ///
    /// # Returns
    /// The (x, y) position of this swatch.
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
    use crate::lab::Lab;
    use crate::rgb::Rgb;
    use crate::white_point::D65;

    #[test]
    fn test_swatch() {
        let color = Rgb::new(255, 0, 64);
        let swatch = Swatch::new(color, (90, 120), 384);
        assert_eq!(swatch.color(), &Rgb::new(255, 0, 64));
        assert_eq!(swatch.position(), (90, 120));
        assert_eq!(swatch.population(), 384);
    }

    #[test]
    fn test_default() {
        let swatch: Swatch<Lab<f64, D65>> = Swatch::default();
        assert_eq!(swatch.color(), &Lab::default());
        assert_eq!(swatch.position(), (0, 0));
        assert_eq!(swatch.population(), 0);
    }
}
