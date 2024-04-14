use crate::color::white_point::WhitePoint;
use crate::color::D65;
use crate::XYZ;
use std::fmt::Display;

/// Color represented in the CIE L*a*b* color space.
///
/// # Fields
/// * `l` - The L component.
/// * `a` - The a component.
/// * `b` - The b component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl Lab {
    /// Minimum value of the L component.
    pub(crate) const MIN_L: f32 = 0.0;

    /// Maximum value of the L component.
    pub(crate) const MAX_L: f32 = 100.0;

    /// Minimum value of the a component.
    pub(crate) const MIN_A: f32 = -128.0;

    /// Maximum value of the a component.
    pub(crate) const MAX_A: f32 = 127.0;

    /// Minimum value of the b component.
    pub(crate) const MIN_B: f32 = -128.0;

    /// Maximum value of the b component.
    pub(crate) const MAX_B: f32 = 127.0;

    /// Creates a new `Lab` instance.
    ///
    /// # Arguments
    /// * `l` - The L component.
    /// * `a` - The a component.
    /// * `b` - The b component.
    pub fn new(l: f32, a: f32, b: f32) -> Self {
        Self {
            l: l.clamp(Self::MIN_L, Self::MAX_L),
            a: a.clamp(Self::MIN_A, Self::MAX_A),
            b: b.clamp(Self::MIN_B, Self::MAX_B),
        }
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab({}, {}, {})", self.l, self.a, self.b)
    }
}

impl From<&XYZ> for Lab {
    fn from(xyz: &XYZ) -> Self {
        let (l, a, b) = xyz_to_lab::<D65>(xyz.x, xyz.y, xyz.z);
        Lab::new(l, a, b)
    }
}

/// Converts the CIE XYZ color space to the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `WP` - The white point.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The L*a*b* color space representation of the XYZ color. The tuple contains the L, a, and b components.
#[inline]
#[must_use]
pub fn xyz_to_lab<WP>(x: f32, y: f32, z: f32) -> (f32, f32, f32)
where
    WP: WhitePoint,
{
    let epsilon = (6.0 / 29.0_f32).powi(3);
    let kappa = 841.0 / 108.0; // ((29.0 / 6.0) ^ 2) / 3.0
    let delta = 4.0 / 29.0;

    let f = |t: f32| -> f32 {
        if t > epsilon {
            t.cbrt()
        } else {
            kappa * t + delta
        }
    };

    let fx = f(x / WP::x());
    let fy = f(y / WP::y());
    let fz = f(z / WP::z());

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    (
        l.clamp(Lab::MIN_L, Lab::MAX_L),
        a.clamp(Lab::MIN_A, Lab::MAX_A),
        b.clamp(Lab::MIN_B, Lab::MAX_B),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::white_point::D65;
    use crate::XYZ;
    use rstest::rstest;

    #[test]
    fn test_new_lab() {
        // Act
        let lab = Lab::new(53.2437, 80.09315, 67.2388);

        // Assert
        assert_eq!(lab.l, 53.2437);
        assert_eq!(lab.a, 80.09315);
        assert_eq!(lab.b, 67.2388);
    }

    #[test]
    fn test_fmt() {
        // Act & Assert
        let lab = Lab::new(53.2437, 80.09315, 67.2388);
        assert_eq!(format!("{}", lab), "Lab(53.2437, 80.09315, 67.2388)");
    }

    #[test]
    fn test_from_xyz() {
        // Act
        let xyz = XYZ::new(0.3576, 0.7152, 0.1192);
        let lab = Lab::from(&xyz);

        // Assert
        assert!((lab.l - 87.7376).abs() < 1e-3);
        assert!((lab.a + 86.1846).abs() < 1e-3);
        assert!((lab.b - 83.1813).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((0.9505, 1.0000, 1.0886), (100.0, 0.0052, 0.0141))]
    #[case::red((0.4125, 0.2127, 0.0193), (53.2437, 80.09315, 67.2388))]
    #[case::green((0.3576, 0.7152, 0.1192), (87.7376, - 86.1846, 83.1813))]
    #[case::blue((0.1804, 0.0722, 0.9502), (32.3026, 79.1436, - 107.8436))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (91.1120, - 48.0806, - 14.1521))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (60.3199, 98.2302, - 60.8496))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (97.1382, - 21.5551, 94.4825))]
    fn test_xyz_to_lab(#[case] xyz: (f32, f32, f32), #[case] lab: (f32, f32, f32)) {
        // Act
        let (l, a, b) = xyz_to_lab::<D65>(xyz.0, xyz.1, xyz.2);

        // Assert
        assert!((l - lab.0).abs() < 1e-3);
        assert!((a - lab.1).abs() < 1e-3);
        assert!((b - lab.2).abs() < 1e-3);
    }
}
