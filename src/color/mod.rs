mod lab;
mod rgb;
mod white_point;
mod xyz;

pub use lab::{xyz_to_lab, Lab};
pub use rgb::RGB;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
pub use white_point::D65;
pub use xyz::{rgb_to_xyz, XYZ};

/// Struct representing a color.
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use auto_palette::Color;
///
/// let color = Color::from_str("#2c7de7").unwrap();
/// assert!(color.is_light());
/// assert_eq!(color.lightness(), 52.917793);
/// assert_eq!(color.chroma(), 61.9814870);
/// assert_eq!(color.hue(), 282.6622);
///```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub(super) l: f32,
    pub(super) a: f32,
    pub(super) b: f32,
}

impl Color {
    /// Creates a new `Color` instance.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[must_use]
    pub(crate) fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }

    /// Returns whether this color is light.
    ///
    /// # Returns
    /// `true` if the color is light, otherwise `false`.
    #[must_use]
    pub fn is_light(&self) -> bool {
        self.l > 50.0
    }

    /// Returns whether this color is dark.
    ///
    /// # Returns
    /// `true` if the color is dark, otherwise `false`.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Returns the lightness of this color.
    ///
    /// # Returns
    /// The lightness of this color.
    #[must_use]
    pub fn lightness(&self) -> f32 {
        self.l
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    #[must_use]
    pub fn chroma(&self) -> f32 {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the hue of this color.
    ///
    /// # Returns
    /// The hue of this color.
    #[must_use]
    pub fn hue(&self) -> f32 {
        let mut hue = self.b.atan2(self.a).to_degrees();
        if hue < 0.0 {
            hue += 360.0;
        }
        hue
    }

    /// Returns the difference between this color and another color.
    ///
    /// # Arguments
    /// * `other` - The other color.
    ///
    /// # Returns
    /// The difference between this color and the other color.
    pub fn difference(&self, other: &Self) -> f32 {
        let delta_l = self.l - other.l;
        let delta_a = self.a - other.a;
        let delta_b = self.b - other.b;
        (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt()
    }

    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        let xyz = self.to_xyz();
        RGB::from(&xyz)
    }

    #[must_use]
    pub fn to_xyz(&self) -> XYZ {
        XYZ::from(&self.to_lab())
    }

    #[must_use]
    pub fn to_lab(&self) -> Lab {
        Lab::new(self.l, self.a, self.b)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Color(l: {}, a: {}, b: {})", self.l, self.a, self.b)
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err("Invalid color format");
        }

        let r = u8::from_str_radix(&s[1..3], 16).unwrap();
        let g = u8::from_str_radix(&s[3..5], 16).unwrap();
        let b = u8::from_str_radix(&s[5..7], 16).unwrap();

        let (x, y, z) = rgb_to_xyz(r, g, b);
        let (l, a, b) = xyz_to_lab::<D65>(x, y, z);
        Ok(Self::new(l, a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new_color() {
        // Act
        let color = Color::new(80.0, 0.0, 0.0);

        // Assert
        assert_eq!(color.l, 80.0);
        assert_eq!(color.a, 0.0);
        assert_eq!(color.b, 0.0);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), false)]
    #[case((50.0, 0.0, 0.0), false)]
    #[case((50.1, 0.0, 0.0), true)]
    #[case((80.0, 0.0, 0.0), true)]
    fn test_color_is_light(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Arrange
        let color = Color::new(input.0, input.1, input.2);

        // Act
        let is_light = color.is_light();

        // Assert
        assert_eq!(is_light, expected);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), true)]
    #[case((50.0, 0.0, 0.0), true)]
    #[case((50.1, 0.0, 0.0), false)]
    #[case((80.0, 0.0, 0.0), false)]
    fn test_color_is_dark(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Arrange
        let color = Color::new(input.0, input.1, input.2);

        // Act
        let is_dark = color.is_dark();

        // Assert
        assert_eq!(is_dark, expected);
    }

    #[test]
    fn test_to_rgb() {
        // Arrange
        let color = Color::new(91.1120, -48.0806, -14.1521);

        // Act
        let rgb = color.to_rgb();

        // Assert
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn test_to_xyz() {
        // Arrange
        let color = Color::new(91.1120, -48.0806, -14.1521);

        // Act
        let xyz = color.to_xyz();

        // Assert
        assert!((xyz.x - 0.5380).abs() < 1e-3);
        assert!((xyz.y - 0.7873).abs() < 1e-3);
        assert!((xyz.z - 1.0690).abs() < 1e-3);
    }

    #[test]
    fn test_to_lab() {
        // Arrange
        let color = Color::new(91.1120, -48.0806, -14.1521);

        // Act
        let lab = color.to_lab();

        // Assert
        assert_eq!(lab.l, 91.1120);
        assert_eq!(lab.a, -48.0806);
        assert_eq!(lab.b, -14.1521);
    }

    #[test]
    fn test_fmt() {
        // Act & Assert
        let color = Color::new(80.0, 0.0, 0.0);
        assert_eq!(format!("{}", color), "Color(l: 80, a: 0, b: 0)");
    }
}
