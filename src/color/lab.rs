use crate::color::white_point::WhitePoint;
use crate::color::D65;
use crate::math::FloatNumber;
use crate::XYZ;
use num_traits::clamp;
use std::fmt::Display;

/// Color represented in the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `l` - The L component.
/// * `a` - The a component.
/// * `b` - The b component.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lab<T>
where
    T: FloatNumber,
{
    pub l: T,
    pub a: T,
    pub b: T,
}

impl<T> Lab<T>
where
    T: FloatNumber,
{
    /// Creates a new `Lab` instance.
    ///
    /// # Arguments
    /// * `l` - The L component.
    /// * `a` - The a component.
    /// * `b` - The b component.
    #[must_use]
    pub fn new(l: T, a: T, b: T) -> Self {
        Self {
            l: clamp(l, Lab::min_l(), Lab::max_l()),
            a: clamp(a, Lab::min_a(), Lab::max_a()),
            b: clamp(b, Lab::min_b(), Lab::max_b()),
        }
    }

    /// Returns the minimum value of the L component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The minimum value of the L component.
    #[inline]
    #[must_use]
    pub(crate) fn min_l() -> T {
        T::zero()
    }

    /// Returns the maximum value of the L component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The maximum value of the L component.
    #[inline]
    #[must_use]
    pub(crate) fn max_l() -> T {
        T::from_f32(100.0)
    }

    /// Returns the minimum value of the a component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The minimum value of the a component.
    #[inline]
    #[must_use]
    pub(crate) fn min_a() -> T {
        T::from_f32(-128.0)
    }

    /// Returns the maximum value of the a component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The maximum value of the a component.
    #[inline]
    #[must_use]
    pub(crate) fn max_a() -> T {
        T::from_f32(127.0)
    }

    /// Returns the minimum value of the b component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The minimum value of the b component.
    #[inline]
    #[must_use]
    pub(crate) fn min_b() -> T {
        T::from_f32(-128.0)
    }

    /// Returns the maximum value of the b component.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The maximum value of the b component.
    #[inline]
    #[must_use]
    pub(crate) fn max_b() -> T {
        T::from_f32(127.0)
    }
}

impl<T> Display for Lab<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab({}, {}, {})", self.l, self.a, self.b)
    }
}

impl<T> From<&XYZ<T>> for Lab<T>
where
    T: FloatNumber,
{
    fn from(xyz: &XYZ<T>) -> Self {
        let (l, a, b) = xyz_to_lab::<T, D65>(xyz.x, xyz.y, xyz.z);
        Lab::new(l, a, b)
    }
}

/// Converts the CIE XYZ color space to the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
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
pub fn xyz_to_lab<T, WP>(x: T, y: T, z: T) -> (T, T, T)
where
    T: FloatNumber,
    WP: WhitePoint,
{
    let epsilon = T::from_f64(6.0 / 29.0).powi(3);
    let kappa = T::from_f64(841.0 / 108.0); // ((29.0 / 6.0) ^ 2) / 3.0
    let delta = T::from_f64(4.0 / 29.0);

    let f = |t: T| -> T {
        if t > epsilon {
            t.cbrt()
        } else {
            kappa * t + delta
        }
    };

    let fx = f(x / WP::x());
    let fy = f(y / WP::y());
    let fz = f(z / WP::z());

    let l = T::from_f32(116.0) * fy - T::from_f32(16.0);
    let a = T::from_f32(500.0) * (fx - fy);
    let b = T::from_f32(200.0) * (fy - fz);
    (
        clamp(l, Lab::min_l(), Lab::max_l()),
        clamp(a, Lab::min_a(), Lab::max_a()),
        clamp(b, Lab::min_b(), Lab::max_b()),
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
        let xyz: XYZ<f64> = XYZ::new(0.3576, 0.7152, 0.1192);
        let lab: Lab<f64> = Lab::from(&xyz);

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
        let (l, a, b) = xyz_to_lab::<f32, D65>(xyz.0, xyz.1, xyz.2);

        // Assert
        assert!((l - lab.0).abs() < 1e-3);
        assert!((a - lab.1).abs() < 1e-3);
        assert!((b - lab.2).abs() < 1e-3);
    }
}
