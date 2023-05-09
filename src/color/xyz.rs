use crate::color::lab::Lab;
use crate::color::rgb::RGB;
use crate::color::white_point::WhitePoint;
use crate::math::number::Float;
use crate::white_point::D65;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Struct representing a color in CIE XYZ color space.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # References
/// * [CIE 1931 color space - Wikipedia](https://en.wikipedia.org/wiki/CIE_1931_color_space)
#[derive(Debug, Clone, PartialEq)]
pub struct XYZ<F: Float, WP: WhitePoint<F> = D65> {
    pub x: F,
    pub y: F,
    pub z: F,
    _marker: PhantomData<WP>,
}

impl<F, WP> XYZ<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    /// Creates a new CIE XYZ color.
    ///
    /// # Arguments
    /// * `x` - The value of X.
    /// * `y` - The value of Y.
    /// * `z` - The value of Z.
    ///
    /// # Returns
    /// A new XYZ color.
    #[inline]
    #[must_use]
    pub fn new(x: F, y: F, z: F) -> XYZ<F, WP> {
        Self {
            x: Self::clamp_x(x),
            y: Self::clamp_y(y),
            z: Self::clamp_z(z),
            _marker: PhantomData::default(),
        }
    }

    /// Returns min value of x.
    ///
    /// # Returns
    /// The min value of x.
    #[inline]
    #[must_use]
    pub(crate) fn min_x<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Returns the max value of x.
    ///
    /// # Returns
    /// The max value of x.
    #[inline]
    #[must_use]
    pub(crate) fn max_x<T: Float>() -> T {
        T::from_f64(0.950456)
    }

    /// Returns the min value of y.
    ///
    /// # Returns
    /// The min value of y.
    #[inline]
    #[must_use]
    pub(crate) fn min_y<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Returns the max value of y.
    ///
    /// # Returns
    /// The max value of y.
    #[inline]
    #[must_use]
    pub(crate) fn max_y<T: Float>() -> T {
        T::from_f64(1.0)
    }

    /// Returns the min value of z.
    ///
    /// # Returns
    /// The min value of z.
    #[inline]
    #[must_use]
    pub(crate) fn min_z<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Returns the max value of z.
    ///
    /// # Returns
    /// The max value of z.
    #[inline]
    #[must_use]
    pub(crate) fn max_z<T: Float>() -> T {
        T::from_f64(1.088644)
    }

    #[inline]
    #[must_use]
    fn clamp_x(value: F) -> F {
        value.clamp(Self::min_x(), Self::max_x())
    }

    #[inline]
    #[must_use]
    fn clamp_y(value: F) -> F {
        value.clamp(Self::min_y(), Self::max_y())
    }

    #[inline]
    #[must_use]
    fn clamp_z(value: F) -> F {
        value.clamp(Self::min_z(), Self::max_z())
    }
}

impl<F, WP> Display for XYZ<F, WP>
where
    F: Float + Default + Display,
    WP: WhitePoint<F>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "XYZ({x:.4}, {y:.4}, {z:.4})",
            x = self.x,
            y = self.y,
            z = self.z
        )
    }
}

impl<F, WP> From<&RGB> for XYZ<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[inline]
    #[must_use]
    fn from(rgb: &RGB) -> Self {
        let f = |value: F| -> F {
            if value <= F::from_f64(0.04045) {
                value / F::from_f64(12.92)
            } else {
                ((value + F::from_f64(0.055)) / F::from_f64(1.055)).powf(F::from_f64(2.4))
            }
        };

        let max_value: F = RGB::max_value();
        let r = f(rgb.r::<F>() / max_value);
        let g = f(rgb.g::<F>() / max_value);
        let b = f(rgb.b::<F>() / max_value);

        let x = F::from_f64(0.412391) * r + F::from_f64(0.357584) * g + F::from_f64(0.180481) * b;
        let y = F::from_f64(0.212639) * r + F::from_f64(0.715169) * g + F::from_f64(0.072192) * b;
        let z = F::from_f64(0.019331) * r + F::from_f64(0.119195) * g + F::from_f64(0.950532) * b;
        XYZ::new(x, y, z)
    }
}

impl<F, WP> From<&Lab<F, WP>> for XYZ<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[inline]
    #[must_use]
    fn from(lab: &Lab<F, WP>) -> Self {
        let epsilon = F::from_f64(6.0 / 29.0);
        let kappa = F::from_f64(108.0 / 841.0); // 3.0 * ((6.0 / 29.0) ^ 2)
        let delta = F::from_f64(4.0 / 29.0);
        let f = |t: F| -> F {
            if t > epsilon {
                t.powi(3)
            } else {
                kappa * (t - delta)
            }
        };

        let l2 = (lab.l + F::from_f64(16.0)) / F::from_f64(116.0);
        let a2 = lab.a / F::from_f64(500.0);
        let b2 = lab.b / F::from_f64(200.0);

        let x = WP::x() * f(l2 + a2);
        let y = WP::y() * f(l2);
        let z = WP::z() * f(l2 - b2);
        XYZ::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::white_point::D65;
    use rstest::rstest;
    use statrs::assert_almost_eq;

    #[test]
    fn test_xyz() {
        let xyz: XYZ<_, D65> = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(xyz.x, 0.256394);
        assert_eq!(xyz.y, 0.223987);
        assert_eq!(xyz.z, 0.975798);

        let xyz: XYZ<_, D65> = XYZ::new(-1.0, -1.0, -1.0);
        assert_eq!(xyz.x, 0.0);
        assert_eq!(xyz.y, 0.0);
        assert_eq!(xyz.z, 0.0);

        let xyz: XYZ<_, D65> = XYZ::new(1.0, 1.1, 1.2);
        assert_eq!(xyz.x, 0.950456);
        assert_eq!(xyz.y, 1.0);
        assert_eq!(xyz.z, 1.088644);
    }

    #[test]
    fn test_fmt() {
        let xyz: XYZ<_, D65> = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(format!("{}", xyz), "XYZ(0.2564, 0.2240, 0.9758)");
    }

    #[rstest]
    #[case((0, 0, 0), (0.0000, 0.0000, 0.0000))] // Black
    #[case((255, 255, 255), (0.9505, 1.0000, 1.0890))] // White
    #[case((255, 0, 0), (0.4124, 0.2126, 0.0193))] // Red
    #[case((0, 255, 0), (0.3576, 0.7152, 0.1192))] // Green
    #[case((0, 0, 255), (0.1805, 0.0722, 0.9505))] // Blue
    #[case((0, 255, 255), (0.5381, 0.7874, 1.0697))] // Cyan
    #[case((255, 0, 255), (0.5929, 0.2848, 0.9698))] // Magenta
    #[case((255, 255, 0), (0.7700, 0.9278, 0.1385))] // Yellow
    fn test_from_rgba(#[case] rgba: (u8, u8, u8), #[case] expected: (f64, f64, f64)) {
        let actual: XYZ<_, D65> = XYZ::from(&RGB::new(rgba.0, rgba.1, rgba.2));
        let (x, y, z) = expected;
        assert_almost_eq!(actual.x, x, 1e-3);
        assert_almost_eq!(actual.y, y, 1e-3);
        assert_almost_eq!(actual.z, z, 1e-3);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), (0.0000, 0.0000, 0.0000))] // Black
    #[case((100.0, 0.0, 0.0254), (0.9505, 1.0000, 1.0890))] // White
    #[case((53.2371, 80.1106, 67.2237), (0.4124, 0.2126, 0.0193))] // Red
    #[case((87.7355, - 86.1822, 83.1866), (0.3576, 0.7152, 0.1192))] // Green
    #[case((32.3008, 79.1952, - 107.8554), (0.1805, 0.0722, 0.9505))] // Blue
    #[case((91.1132, - 48.0875, - 14.1312), (0.5381, 0.7874, 1.0697))] // Cyan
    #[case((60.3242, 98.2557, - 60.8249), (0.5929, 0.2848, 0.9698))] // Magenta
    #[case((97.1393, - 21.5537, 94.4896), (0.7700, 0.9278, 0.1385))] // Yellow
    fn test_from_lab(#[case] lab: (f64, f64, f64), #[case] expected: (f64, f64, f64)) {
        let actual: XYZ<_, D65> = XYZ::from(&Lab::new(lab.0, lab.1, lab.2));
        let (x, y, z) = expected;
        assert_almost_eq!(actual.x, x, 1e-3);
        assert_almost_eq!(actual.y, y, 1e-3);
        assert_almost_eq!(actual.z, z, 1e-3);
    }
}
