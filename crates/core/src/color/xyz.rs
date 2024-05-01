use std::fmt::Display;

use num_traits::clamp;

use crate::{
    color::{rgb::RGB, white_point::WhitePoint, D65},
    math::FloatNumber,
    Lab,
};

/// Color representation in the CIE XYZ color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `x` - The X component.
/// * `y` - The Y component.
/// * `z` - The Z component.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ<T>
where
    T: FloatNumber,
{
    pub(super) x: T,
    pub(super) y: T,
    pub(super) z: T,
}

impl<T> XYZ<T>
where
    T: FloatNumber,
{
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
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: clamp(x, Self::min_x(), Self::max_x()),
            y: clamp(y, Self::min_y(), Self::max_y()),
            z: clamp(z, Self::min_z(), Self::max_z()),
        }
    }

    /// Returns the minimum value of the X component.
    ///
    /// # Returns
    /// The minimum value of the X component.
    #[inline]
    #[must_use]
    pub(crate) fn min_x() -> T {
        T::zero()
    }

    /// Returns the minimum value of the X component.
    ///
    /// # Returns
    /// The minimum value of the X component.
    #[inline]
    #[must_use]
    pub(crate) fn max_x() -> T {
        T::from_f32(0.950_456)
    }

    /// Returns the minimum value of the Y component.
    ///
    /// # Returns
    /// The minimum value of the Y component.
    #[inline]
    #[must_use]
    pub(crate) fn min_y() -> T {
        T::zero()
    }

    /// Returns the maximum value of the Y component.
    ///
    /// # Returns
    /// The maximum value of the Y component.
    #[inline]
    #[must_use]
    pub(crate) fn max_y() -> T {
        T::one()
    }

    /// Returns the minimum value of the Z component.
    ///
    /// # Returns
    /// The minimum value of the Z component.
    #[inline]
    #[must_use]
    pub(crate) fn min_z() -> T {
        T::zero()
    }

    /// Returns the maximum value of the Z component.
    ///
    /// # Returns
    /// The maximum value of the Z component.
    #[inline]
    #[must_use]
    pub(crate) fn max_z() -> T {
        T::from_f32(1.088_644)
    }
}

impl<T> Display for XYZ<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XYZ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T> From<&RGB> for XYZ<T>
where
    T: FloatNumber,
{
    fn from(rgb: &RGB) -> Self {
        let (x, y, z) = rgb_to_xyz(rgb.r, rgb.g, rgb.b);
        XYZ::new(x, y, z)
    }
}

impl<T> From<&Lab<T>> for XYZ<T>
where
    T: FloatNumber,
{
    fn from(lab: &Lab<T>) -> Self {
        let (x, y, z) = lab_to_xyz::<T, D65>(lab.l, lab.a, lab.b);
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
pub fn rgb_to_xyz<T>(r: u8, g: u8, b: u8) -> (T, T, T)
where
    T: FloatNumber,
{
    let f = |t: T| -> T {
        if t <= T::from_f32(0.04045) {
            t / T::from_f32(12.92)
        } else {
            ((t + T::from_f32(0.055)) / T::from_f32(1.055)).powf(T::from_f32(2.4))
        }
    };

    let r = f(T::from_u8(r) / RGB::max_value());
    let g = f(T::from_u8(g) / RGB::max_value());
    let b = f(T::from_u8(b) / RGB::max_value());

    let x = T::from_f32(0.412_391) * r + T::from_f32(0.357_584) * g + T::from_f32(0.180_481) * b;
    let y = T::from_f32(0.212_639) * r + T::from_f32(0.715_169) * g + T::from_f32(0.072_192) * b;
    let z = T::from_f32(0.019_331) * r + T::from_f32(0.119_195) * g + T::from_f32(0.950_532) * b;

    (
        clamp(x, XYZ::min_x(), XYZ::max_x()),
        clamp(y, XYZ::min_y(), XYZ::max_y()),
        clamp(z, XYZ::min_z(), XYZ::max_z()),
    )
}

/// Converts the CIE L*a*b* color space to the CIE XYZ color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
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
pub fn lab_to_xyz<T, WP>(l: T, a: T, b: T) -> (T, T, T)
where
    T: FloatNumber,
    WP: WhitePoint,
{
    let epsilon = T::from_f64(6.0 / 29.0);
    let kappa = T::from_f64(108.0 / 841.0); // 3.0 * ((6.0 / 29.0) ^ 2)
    let delta = T::from_f64(4.0 / 29.0);

    let f = |t: T| -> T {
        if t > epsilon {
            t.powi(3)
        } else {
            kappa * (t - delta)
        }
    };

    let l2 = (l + T::from_f32(16.0)) / T::from_f32(116.0);
    let fx = f(a / T::from_f32(500.0) + l2);
    let fy = f(l2);
    let fz = f(l2 - b / T::from_f32(200.0));

    let x = WP::x::<T>() * fx;
    let y = WP::y::<T>() * fy;
    let z = WP::z::<T>() * fz;
    (
        clamp(x, XYZ::min_x(), XYZ::max_x()),
        clamp(y, XYZ::min_y(), XYZ::max_y()),
        clamp(z, XYZ::min_z(), XYZ::max_z()),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::color::D65;

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
        let xyz: XYZ<f32> = XYZ::from(&RGB::new(255, 0, 255));

        // Assert
        assert!((xyz.x - 0.5928).abs() < 1e-3);
        assert!((xyz.y - 0.2848).abs() < 1e-3);
        assert!((xyz.z - 0.9699).abs() < 1e-3);
    }

    #[test]
    fn test_from_lab() {
        // Act
        let xyz: XYZ<f64> = XYZ::from(&Lab::new(60.3199, 98.2302, -60.8496));

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
        let (x, y, z) = rgb_to_xyz::<f32>(rgb.0, rgb.1, rgb.2);

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
        let (x, y, z) = lab_to_xyz::<f32, D65>(lab.0, lab.1, lab.2);

        // Assert
        assert!((x - xyz.0).abs() < 1e-3);
        assert!((y - xyz.1).abs() < 1e-3);
        assert!((z - xyz.2).abs() < 1e-3);
    }
}
