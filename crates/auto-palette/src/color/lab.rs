use std::{fmt::Display, marker::PhantomData};

use num_traits::clamp;

use crate::{
    color::{white_point::WhitePoint, LCHab, D65, XYZ},
    math::FloatNumber,
};

/// Color represented in the CIE L*a*b* color space.
///
/// See the following for more details:
/// [CIELAB color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Fields
/// * `l` - The L component.
/// * `a` - The a component.
/// * `b` - The b component.
///
/// # Examples
/// ```
/// use auto_palette::color::{LCHab, Lab, D65, XYZ};
///
/// let xyz = XYZ::new(0.3576, 0.7152, 0.1192);
/// let lab = Lab::<_>::from(&xyz);
/// assert_eq!(format!("{}", lab), "Lab(87.74, -86.18, 83.18)");
///
/// let lchab: LCHab<_> = (&lab).into();
/// assert_eq!(format!("{}", lchab), "LCH(ab)(87.74, 119.78, 136.02)");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Lab<T, W = D65>
where
    T: FloatNumber,
    W: WhitePoint,
{
    pub l: T,
    pub a: T,
    pub b: T,
    _marker: PhantomData<W>,
}

impl<T, W> Lab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    /// Creates a new `Lab` instance.
    ///
    /// # Arguments
    /// * `l` - The L component.
    /// * `a` - The a component.
    /// * `b` - The b component.
    ///
    /// # Returns
    /// A new `Lab` instance.
    #[must_use]
    pub fn new(l: T, a: T, b: T) -> Self {
        Self {
            l: clamp(l, Lab::<T, W>::min_l(), Lab::<T, W>::max_l()),
            a: clamp(a, Lab::<T, W>::min_a(), Lab::<T, W>::max_a()),
            b: clamp(b, Lab::<T, W>::min_b(), Lab::<T, W>::max_b()),
            _marker: PhantomData,
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
        write!(f, "Lab({:.2}, {:.2}, {:.2})", self.l, self.a, self.b)
    }
}

impl<T, W> From<&XYZ<T>> for Lab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(xyz: &XYZ<T>) -> Self {
        let (l, a, b) = xyz_to_lab::<T, W>(xyz.x, xyz.y, xyz.z);
        Lab::new(l, a, b)
    }
}

impl<T, W> From<&LCHab<T, W>> for Lab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(lch: &LCHab<T, W>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let l = lch.l;
        let c = lch.c;
        let h = lch.h.to_radians();
        let a = c * h.cos();
        let b = c * h.sin();
        Lab::new(l, a, b)
    }
}

/// Converts the CIE XYZ color space to the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
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
pub fn xyz_to_lab<T, W>(x: T, y: T, z: T) -> (T, T, T)
where
    T: FloatNumber,
    W: WhitePoint,
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

    let fx = f(x / W::x());
    let fy = f(y / W::y());
    let fz = f(z / W::z());

    let l = T::from_f32(116.0) * fy - T::from_f32(16.0);
    let a = T::from_f32(500.0) * (fx - fy);
    let b = T::from_f32(200.0) * (fy - fz);
    (
        clamp(l, Lab::<T, W>::min_l(), Lab::<T, W>::max_l()),
        clamp(a, Lab::<T, W>::min_a(), Lab::<T, W>::max_a()),
        clamp(b, Lab::<T, W>::min_b(), Lab::<T, W>::max_b()),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::color::white_point::D65;

    #[test]
    fn test_new() {
        // Act
        let actual = Lab::<_>::new(53.2437, 80.09315, 67.2388);

        // Assert
        assert_eq!(actual.l, 53.2437);
        assert_eq!(actual.a, 80.09315);
        assert_eq!(actual.b, 67.2388);
    }

    #[test]
    fn test_fmt() {
        // Act
        let lab = Lab::<_>::new(53.2437, 80.09315, 67.2388);
        let actual = format!("{}", lab);

        // Assert
        assert_eq!(actual, "Lab(53.24, 80.09, 67.24)");
    }

    #[test]
    fn test_from_xyz() {
        // Act
        let xyz: XYZ<f64> = XYZ::new(0.3576, 0.7152, 0.1192);
        let actual: Lab<f64> = Lab::<_>::from(&xyz);

        // Assert
        assert!((actual.l - 87.7376).abs() < 1e-3);
        assert!((actual.a + 86.1846).abs() < 1e-3);
        assert!((actual.b - 83.1813).abs() < 1e-3);
    }

    #[test]
    fn test_from_lchab() {
        // Act
        let lchab: LCHab<f64> = LCHab::new(54.617, 92.151, 27.756);
        let actual = Lab::from(&lchab);

        // Assert
        assert_eq!(actual.l, 54.617);
        assert!((actual.a - 81.549).abs() < 1e-3);
        assert!((actual.b - 42.915).abs() < 1e-3);
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
