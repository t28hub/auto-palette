use std::{fmt::Display, marker::PhantomData};

use num_traits::clamp;

use crate::{
    color::{white_point::WhitePoint, LCHuv, D65, XYZ},
    math::FloatNumber,
};

/// CIE L*u*v* color space representation.
///
/// See the following for more details:
/// [CIELUV - Wikipedia](https://en.wikipedia.org/wiki/CIELUV)
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Fields
/// * `l` - The L* component.
/// * `u` - The u* component.
/// * `v` - The v* component.
///
/// # Examples
/// ```
/// use auto_palette::color::{LCHuv, Luv, D65};
///
/// let luv: Luv<_> = Luv::new(53.64, 165.87, 24.17);
/// assert_eq!(format!("{}", luv), "Luv(53.64, 165.87, 24.17)");
///
/// let lchuv: LCHuv<_> = (&luv).into();
/// assert_eq!(format!("{}", lchuv), "LCH(uv)(53.64, 167.62, 8.29)");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Luv<T, W = D65>
where
    T: FloatNumber,
    W: WhitePoint,
{
    pub l: T,
    pub u: T,
    pub v: T,
    _marker: PhantomData<W>,
}

impl<T, W> Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    /// Creates a new `Luv` instance.
    ///
    /// # Arguments
    /// * `l` - The L* component.
    /// * `u` - The u* component.
    /// * `v` - The v* component.
    ///
    /// # Returns
    /// A new `Luv` instance.
    #[must_use]
    pub fn new(l: T, u: T, v: T) -> Self {
        // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
        // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
        // The values are referenced from the following link:
        // https://docs.opencv.org/2.4/modules/imgproc/doc/miscellaneous_transformations.html?highlight=luv#cvtcolor
        Self {
            l: clamp(l, T::zero(), T::from_f32(100.0)),
            // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
            u: clamp(u, T::from_f32(-134.0), T::from_f32(220.0)),
            // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
            v: clamp(v, T::from_f32(-140.0), T::from_f32(122.0)),
            _marker: PhantomData,
        }
    }
}

impl<T, W> Display for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Luv({:.2}, {:.2}, {:.2})", self.l, self.u, self.v)
    }
}

impl<T, W> From<&XYZ<T>> for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(xyz: &XYZ<T>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
        let y = xyz.y / W::y();
        let cie_k = T::from_f64(903.296_296); // 24389 / 27
        let cie_e = T::from_f64(0.008_856); // 216 / 24389

        let l = if y > cie_e {
            T::from_u8(116) * y.powf(T::from_f32(1.0 / 3.0)) - T::from_u8(16)
        } else {
            cie_k * y
        };

        let denominator = xyz.x + T::from_f32(15.0) * xyz.y + T::from_f32(3.0) * xyz.z;
        let u_prime = if denominator.is_zero() {
            T::zero()
        } else {
            T::from_f32(4.0) * xyz.x / denominator
        };
        let v_prime = if denominator.is_zero() {
            T::zero()
        } else {
            T::from_f32(9.0) * xyz.y / denominator
        };

        let denominator =
            W::x::<T>() + T::from_f32(15.0) * W::y::<T>() + T::from_f32(3.0) * W::z::<T>();
        let u_prime_ref = T::from_f32(4.0) * W::x() / denominator;
        let v_prime_ref = T::from_f32(9.0) * W::y() / denominator;

        let u = T::from_f32(13.0) * l * (u_prime - u_prime_ref);
        let v = T::from_f32(13.0) * l * (v_prime - v_prime_ref);
        Luv::new(l, u, v)
    }
}

impl<T, W> From<&LCHuv<T, W>> for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(lch: &LCHuv<T, W>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_LCH_to_Luv.html
        let l = lch.l;
        let c = lch.c;
        let h = lch.h.to_radians();
        let u = c * h.cos();
        let v = c * h.sin();
        Luv::new(l, u, v)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::color::RGB;

    #[test]
    fn test_new() {
        // Act
        let actual: Luv<f32> = Luv::new(53.64, 165.86, 24.17);

        // Assert
        assert_eq!(
            actual,
            Luv {
                l: 53.64,
                u: 165.86,
                v: 24.17,
                _marker: PhantomData,
            }
        );
    }

    #[rstest]
    #[case((100.1, 220.1, 122.1), (100.0, 220.0, 122.0))]
    #[case((-0.1, -134.1, -140.1), (0.0, -134.0, -140.0))]
    fn test_new_with_out_of_bounds_values(
        #[case] input: (f32, f32, f32),
        #[case] expected: (f32, f32, f32),
    ) {
        // Act
        let actual: Luv<f32> = Luv::new(input.0, input.1, input.2);

        // Assert
        assert_eq!(actual.l, expected.0);
        assert_eq!(actual.u, expected.1);
        assert_eq!(actual.v, expected.2);
    }

    #[test]
    fn test_fmt() {
        // Act
        let luv = Luv::<_, D65>::new(53.64, 165.86, 24.17);
        let actual = format!("{}", luv);

        // Assert
        assert_eq!(actual, "Luv(53.64, 165.86, 24.17)");
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.00, 0.00, 0.00))]
    #[case::white((255, 255, 255), (100.0, 0.00, 0.01))]
    #[case::red((255, 0, 0), (53.24, 175.00, 37.75))]
    #[case::green((0, 255, 0), (87.74, -83.08, 107.40))]
    #[case::blue((0, 0, 255), (32.30, -9.41, -130.36))]
    #[case::yellow((255, 255, 0), (97.14, 7.69, 106.79))]
    #[case::cyan((0, 255, 255), (91.11, -70.48, -15.22))]
    #[case::magenta((255, 0, 255), (60.32, 84.05, -108.71))]
    fn test_from_xyz(#[case] rgb: (u8, u8, u8), #[case] expected: (f32, f32, f32)) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let xyz: XYZ<f32> = XYZ::from(&rgb);
        let actual = Luv::<_, D65>::from(&xyz);

        // Assert
        let (l, u, v) = expected;
        assert!((actual.l - l).abs() < 1e-2);
        assert!((actual.u - u).abs() < 1e-2);
        assert!((actual.v - v).abs() < 1e-2);
    }

    #[test]
    fn test_from_lchuv() {
        // Act
        let lchuv: LCHuv<f32> = LCHuv::new(53.640, 167.622, 8.291);
        let actual = Luv::from(&lchuv);

        println!("{:?}", actual);

        // Assert
        assert_eq!(actual.l, 53.640);
        assert!((actual.u - 165.870).abs() < 1e-3);
        assert!((actual.v - 24.171).abs() < 1e-3);
    }
}
