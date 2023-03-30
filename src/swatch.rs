use crate::color::rgba::Rgba;

/// Struct representing a swatch that contains a color and its position.
///
/// # Examples
/// ```
/// use auto_palette::Swatch;
/// use auto_palette::rgba::Rgba;
///
/// let color = Rgba::new(255, 0, 64, 255);
/// let swatch = Swatch::new(color, (90, 120), 384);
/// assert_eq!(swatch.color(), (255, 0,64, 255));
/// assert_eq!(swatch.position(), (90, 120));
/// assert_eq!(swatch.size(), 384);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Swatch {
    pub(crate) color: Rgba,
    pub(crate) position: (u32, u32),
    pub(crate) size: usize,
}

impl Swatch {
    /// Creates a new `Swatch` instance.
    ///
    /// # Arguments
    /// * `color` - The RGBA color of the swatch.
    /// * `position` - The (x, y) position of the swatch.
    /// * `size` - The size of the swatch.
    ///
    /// # Returns
    /// A `Swatch` instance.
    #[must_use]
    pub fn new(color: Rgba, position: (u32, u32), size: usize) -> Self {
        Self {
            color,
            position,
            size,
        }
    }

    /// Returns the RGBA color of this swatch.
    ///
    /// # Returns
    /// A reference of RGBA color of this swatch.
    #[must_use]
    pub fn color(&self) -> &Rgba {
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

    /// Returns the size of this swatch.
    ///
    /// # Returns
    /// The size of this swatch.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for Swatch {
    #[must_use]
    fn default() -> Self {
        Self {
            color: Rgba::default(),
            position: (0, 0),
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swatch() {
        let color = Rgba::new(255, 0, 64, 255);
        let swatch = Swatch::new(color, (90, 120), 384);
        assert_eq!(swatch.color(), &Rgba::new(255, 0, 64, 255));
        assert_eq!(swatch.position(), (90, 120));
        assert_eq!(swatch.size(), 384);
    }

    #[test]
    fn test_defaults() {
        let swatch = Swatch::default();
        assert_eq!(swatch.color(), &Rgba::default());
        assert_eq!(swatch.position(), (0, 0));
        assert_eq!(swatch.size(), 0);
    }
}
