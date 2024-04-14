use crate::color::rgb::RGB;
use crate::color::white_point::WhitePoint;
use crate::color::D65;
use crate::Lab;
use std::fmt::Display;

/// Color representation in the CIE XYZ color space.
///
/// # Fields
/// * `x` - The X component.
/// * `y` - The Y component.
/// * `z` - The Z component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
}

impl XYZ {
    /// Minimum value of the X component.
    const MIN_X: f32 = 0.0;

    /// Maximum value of the X component.
    const MAX_X: f32 = 0.950456;

    /// Minimum value of the Y component.
    const MIN_Y: f32 = 0.0;

    /// Maximum value of the Y component.
    const MAX_Y: f32 = 1.0;

    /// Minimum value of the Z component.
    const MIN_Z: f32 = 0.0;

    /// Maximum value of the Z component.
    const MAX_Z: f32 = 1.088644;

    /// Creates a new `XYZ` instance.
    ///
    /// # Arguments
    /// * `x` - The X component.
    /// * `y` - The Y component.
    /// * `z` - The Z component.
    ///
    /// # Returns
    /// A new `XYZ` instance.
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x.clamp(XYZ::MIN_X, XYZ::MAX_X),
            y: y.clamp(XYZ::MIN_Y, XYZ::MAX_Y),
            z: z.clamp(XYZ::MIN_Z, XYZ::MAX_Z),
        }
    }
}

impl Display for XYZ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<&RGB> for XYZ {
    fn from(rgb: &RGB) -> Self {
        let (x, y, z) = rgb_to_xyz(rgb.r, rgb.g, rgb.b);
        XYZ::new(x, y, z)
    }
}

impl From<&Lab> for XYZ {
    fn from(lab: &Lab) -> Self {
        let (x, y, z) = lab_to_xyz::<D65>(lab.l, lab.a, lab.b);
        XYZ::new(x, y, z)
    }
}

/// Converts the RGB color space to the CIE XYZ color space.
///
/// # Arguments
/// * `r` - The red component of the RGB color.
/// * `g` - The green component of the RGB color.
/// * `b` - The blue component of the RGB color.
///
/// # Returns
/// The XYZ color space representation of the RGB color. The tuple contains the X, Y, and Z components.
#[inline]
#[must_use]
pub fn rgb_to_xyz(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let f = |t: f32| -> f32 {
        if t <= 0.04045 {
            t / 12.92
        } else {
            ((t + 0.055) / 1.055).powf(2.4)
        }
    };

    let r = f(r as f32 / 255.0);
    let g = f(g as f32 / 255.0);
    let b = f(b as f32 / 255.0);

    let x = 0.412_391 * r + 0.357_584 * g + 0.180_481 * b;
    let y = 0.212_639 * r + 0.715_169 * g + 0.072_192 * b;
    let z = 0.019_331 * r + 0.119_195 * g + 0.950_532 * b;
    (
        x.clamp(XYZ::MIN_X, XYZ::MAX_X),
        y.clamp(XYZ::MIN_Y, XYZ::MAX_Y),
        z.clamp(XYZ::MIN_Z, XYZ::MAX_Z),
    )
}

/// Converts the CIE L*a*b* color space to the CIE XYZ color space.
///
/// # Type Parameters
/// * `WP` - The white point.
///
/// # Arguments
/// * `l` - The L component of the L*a*b* color.
/// * `a` - The a component of the L*a*b* color.
/// * `b` - The b component of the L*a*b* color.
///
/// # Returns
/// The XYZ color space representation of the L*a*b* color. The tuple contains the X, Y, and Z components.
#[inline]
#[must_use]
pub fn lab_to_xyz<WP>(l: f32, a: f32, b: f32) -> (f32, f32, f32)
where
    WP: WhitePoint,
{
    let epsilon = 6.0 / 29.0;
    let kappa = 108.0 / 841.0; // 3.0 * ((6.0 / 29.0) ^ 2)
    let delta = 4.0 / 29.0;

    let f = |t: f32| -> f32 {
        if t > epsilon {
            t.powi(3)
        } else {
            kappa * (t - delta)
        }
    };

    let l2 = (l + 16.0) / 116.0;
    let fx = f(a / 500.0 + l2);
    let fy = f(l2);
    let fz = f(l2 - b / 200.0);

    let x = WP::x() * fx;
    let y = WP::y() * fy;
    let z = WP::z() * fz;
    (
        x.clamp(XYZ::MIN_X, XYZ::MAX_X),
        y.clamp(XYZ::MIN_Y, XYZ::MAX_Y),
        z.clamp(XYZ::MIN_Z, XYZ::MAX_Z),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::D65;
    use rstest::rstest;

    #[test]
    fn test_new_xyz() {
        // Act
        let xyz = XYZ::new(0.5928, 0.2848, 0.9699);

        // Assert
        assert_eq!(xyz.x, 0.5928);
        assert_eq!(xyz.y, 0.2848);
        assert_eq!(xyz.z, 0.9699);
    }

    #[test]
    fn test_fmt() {
        // Arrange
        let xyz = XYZ::new(0.5928, 0.2848, 0.9699);

        // Act
        let result = format!("{}", xyz);

        // Assert
        assert_eq!(result, "XYZ(0.5928, 0.2848, 0.9699)");
    }

    #[test]
    fn test_from_rgb() {
        // Act
        let xyz = XYZ::from(&RGB::new(255, 0, 255));

        // Assert
        assert!((xyz.x - 0.5928).abs() < 1e-3);
        assert!((xyz.y - 0.2848).abs() < 1e-3);
        assert!((xyz.z - 0.9699).abs() < 1e-3);
    }

    #[test]
    fn test_from_lab() {
        // Act
        let xyz = XYZ::from(&Lab::new(60.3199, 98.2302, -60.8496));

        // Assert
        assert!((xyz.x - 0.5928).abs() < 1e-3);
        assert!((xyz.y - 0.2848).abs() < 1e-3);
        assert!((xyz.z - 0.9699).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.9505, 1.0000, 1.0886))]
    #[case::red((255, 0, 0), (0.4125, 0.2127, 0.0193))]
    #[case::green((0, 255, 0), (0.3576, 0.7152, 0.1192))]
    #[case::blue((0, 0, 255), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((0, 255, 255), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((255, 0, 255), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((255, 255, 0), (0.7700, 0.9278, 0.1385))]
    fn test_rgb_to_xyz(#[case] rgb: (u8, u8, u8), #[case] xyz: (f32, f32, f32)) {
        // Act
        let (x, y, z) = rgb_to_xyz(rgb.0, rgb.1, rgb.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((100.0, 0.0052, 0.0141), (0.9505, 1.0000, 1.0886))]
    #[case::red((53.2437, 80.09315, 67.2388), (0.4125, 0.2127, 0.0193))]
    #[case::green((87.7376, - 86.1846, 83.1813), (0.3576, 0.7152, 0.1192))]
    #[case::blue((32.3026, 79.1436, - 107.8436), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((91.1120, - 48.0806, - 14.1521), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((60.3199, 98.2302, - 60.8496), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((97.1382, - 21.5551, 94.4825), (0.7700, 0.9278, 0.1385))]
    fn test_lab_to_xyz(#[case] lab: (f32, f32, f32), #[case] xyz: (f32, f32, f32)) {
        // Act
        let (x, y, z) = lab_to_xyz::<D65>(lab.0, lab.1, lab.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }
}
