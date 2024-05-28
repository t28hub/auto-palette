use std::fmt::Display;

use num_traits::clamp;

use crate::{
    color::{lab::Lab, luv::Luv, rgb::RGB, white_point::WhitePoint, Oklab},
    math::FloatNumber,
};

/// The CIE XYZ color representation.
///
/// See the following for more details:
/// [CIE 1931 color space - Wikipedia](https://en.wikipedia.org/wiki/CIE_1931_color_space)
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `x` - The X component.
/// * `y` - The Y component.
/// * `z` - The Z component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lab, Oklab, RGB, XYZ};
///
/// let rgb = RGB::new(255, 0, 255);
/// let xyz = XYZ::<f32>::from(&rgb);
/// assert_eq!(format!("{}", xyz), "XYZ(0.59, 0.28, 0.97)");
///
/// let lab: Lab<_> = (&xyz).into();
/// assert_eq!(format!("{}", lab), "Lab(60.32, 98.24, -60.84)");
///
/// let oklab: Oklab<_> = (&xyz).into();
/// assert_eq!(format!("{}", oklab), "Oklab(0.70, 0.27, -0.17)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XYZ<T>
where
    T: FloatNumber,
{
    pub x: T,
    pub y: T,
    pub z: T,
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
        write!(f, "XYZ({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl<T> From<&RGB> for XYZ<T>
where
    T: FloatNumber,
{
    fn from(rgb: &RGB) -> Self {
        let (x, y, z) = rgb_to_xyz(rgb.r, rgb.g, rgb.b);
        Self::new(x, y, z)
    }
}

impl<T, W> From<&Lab<T, W>> for XYZ<T>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(lab: &Lab<T, W>) -> Self {
        let (x, y, z) = lab_to_xyz::<T, W>(lab.l, lab.a, lab.b);
        Self::new(x, y, z)
    }
}

impl<T, W> From<&Luv<T, W>> for XYZ<T>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(luv: &Luv<T, W>) -> Self {
        // This implementation is based on the algorithm described in the following link:
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
        if luv.l.is_zero() {
            return XYZ::new(T::zero(), T::zero(), T::zero());
        }

        let denominator =
            W::x::<T>() + T::from_f32(15.0) * W::y::<T>() + T::from_f32(3.0) * W::z::<T>();
        let u0 = T::from_f32(4.0) * W::x::<T>() / denominator;
        let v0 = T::from_f32(9.0) * W::y::<T>() / denominator;

        let y = if luv.l > T::from_f32(8.0) {
            ((luv.l + T::from_f32(16.0)) / T::from_f32(116.0)).powi(3)
        } else {
            luv.l / T::from_f64(903.296_296)
        };

        let a = ((T::from_f32(52.0) * luv.l) / (luv.u + T::from_f32(13.0) * luv.l * u0) - T::one())
            * T::from_f32(1.0 / 3.0);
        let b = y * T::from_f32(-5.0);
        let c = T::from_f32(-1.0 / 3.0);
        let d = y
            * ((T::from_f32(39.0) * luv.l) / (luv.v + T::from_f32(13.0) * luv.l * v0)
                - T::from_f32(5.0));

        let x = (d - b) / (a - c);
        let z = x * a + b;
        Self::new(x, y, z)
    }
}

impl<T> From<&Oklab<T>> for XYZ<T>
where
    T: FloatNumber,
{
    fn from(oklab: &Oklab<T>) -> Self {
        // The inverse matrix of the conversion matrix M2 from LMS to L*a*b* is multiplied.
        let l_prime = T::from_f64(0.999_999_998_5) * oklab.l
            + T::from_f64(0.396_337_792_1) * oklab.a
            + T::from_f64(0.215_803_758_1) * oklab.b;
        let m_prime = T::from_f64(1.000_000_008_9) * oklab.l
            - T::from_f64(0.105_561_342_3) * oklab.a
            - T::from_f64(0.063_854_174_8) * oklab.b;
        let c_prime = T::from_f64(1.000_000_054_7) * oklab.l
            - T::from_f64(0.089_484_182_1) * oklab.a
            - T::from_f64(1.291_485_537_8) * oklab.b;

        let l = l_prime.powi(3);
        let m = m_prime.powi(3);
        let c = c_prime.powi(3);

        // The inverse matrix of the conversion matrix M1 from XYZ to LMS is multiplied.
        let x = T::from_f64(1.227_013_851_1) * l
            + T::from_f64(-0.557_799_980_7) * m
            + T::from_f64(0.281_256_149_0) * c;
        let y = T::from_f64(-0.040_580_178_4) * l
            + T::from_f64(1.112_256_869_6) * m
            + T::from_f64(-0.071_676_678_7) * c;
        let z = T::from_f64(-0.076_381_284_5) * l
            + T::from_f64(-0.421_481_978_4) * m
            + T::from_f64(1.586_163_220_4) * c;
        Self::new(x, y, z)
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
    // This implementation is based on the algorithm described in the following link:
    // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
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
    fn test_new() {
        // Act
        let actual = XYZ::new(0.5928, 0.2848, 0.9699);

        // Assert
        assert_eq!(actual.x, 0.5928);
        assert_eq!(actual.y, 0.2848);
        assert_eq!(actual.z, 0.9699);
    }

    #[test]
    fn test_fmt() {
        // Act
        let xyz = XYZ::new(0.5928, 0.2848, 0.9699);
        let actual = format!("{}", xyz);

        // Assert
        assert_eq!(actual, "XYZ(0.59, 0.28, 0.97)");
    }

    #[test]
    fn test_from_rgb() {
        // Act
        let rgb = RGB::new(255, 0, 255);
        let actual: XYZ<f32> = XYZ::from(&rgb);

        // Assert
        assert!((actual.x - 0.5928).abs() < 1e-3);
        assert!((actual.y - 0.2848).abs() < 1e-3);
        assert!((actual.z - 0.9699).abs() < 1e-3);
    }

    #[test]
    fn test_from_lab() {
        // Act
        let lab = Lab::<_>::new(60.3199, 98.2302, -60.8496);
        let actual: XYZ<f64> = XYZ::from(&lab);

        // Assert
        assert!((actual.x - 0.5928).abs() < 1e-3);
        assert!((actual.y - 0.2848).abs() < 1e-3);
        assert!((actual.z - 0.9699).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::dark_gray((4.5, 0.0, 0.0), (0.005, 0.005, 0.005))]
    #[case::white((100.0, 0.0, 0.0), (0.950, 1.000, 1.089))]
    #[case::red((53.241, 175.015, 37.756), (0.412, 0.213, 0.019))]
    #[case::green((87.735, -83.078, 107.399), (0.358, 0.715, 0.119))]
    #[case::blue((32.297, -9.405, -130.342), (0.180, 0.072, 0.950))]
    #[case::yellow((97.139, 7.706, 106.787), (0.770, 0.928, 0.138))]
    #[case::cyan((91.113, -70.477, -15.202), (0.538, 0.787, 1.070))]
    #[case::magenta((60.324, 84.071, -108.683), (0.593, 0.285, 0.970))]
    fn test_from_luv(#[case] luv: (f32, f32, f32), #[case] expected: (f32, f32, f32)) {
        // Act
        let luv: Luv<f32> = Luv::new(luv.0, luv.1, luv.2);
        let actual = XYZ::from(&luv);

        // Assert
        assert!((actual.x - expected.0).abs() < 1e-3);
        assert!((actual.y - expected.1).abs() < 1e-3);
        assert!((actual.z - expected.2).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.000, 0.0000, 0.0000), (0.000, 0.000, 0.000))]
    #[case::white((1.000, 0.0000, 0.0000), (0.950, 1.000, 1.089))]
    #[case::red((0.628, 0.225, 0.126), (0.412, 0.213, 0.019))]
    #[case::green((0.866, -0.234, 0.180), (0.357, 0.714, 0.117))]
    #[case::blue((0.452, -0.032, -0.312), (0.180, 0.072, 0.952))]
    #[case::yellow((0.968, -0.071, 0.199), (0.770, 0.928, 0.137))]
    #[case::cyan((0.905, -0.149, -0.039), (0.538, 0.786, 1.065))]
    #[case::magenta((0.702, 0.275, -0.169), (0.593, 0.285, 0.970))]
    fn test_from_oklab(#[case] oklab: (f32, f32, f32), #[case] expected: (f32, f32, f32)) {
        // Act
        let oklab: Oklab<f32> = Oklab::new(oklab.0, oklab.1, oklab.2);
        let actual = XYZ::from(&oklab);

        // Assert
        assert!((actual.x - expected.0).abs() < 1e-3);
        assert!((actual.y - expected.1).abs() < 1e-3);
        assert!((actual.z - expected.2).abs() < 1e-3);
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
