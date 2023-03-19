use crate::color::lab::Lab;
use crate::color::rgba::Rgba;
use crate::color::white_point::{WhitePoint, D65};
use crate::math::number::Float;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Color in CIE XYZ color space.
///
/// [CIE 1931 color space - Wikipedia](https://en.wikipedia.org/wiki/CIE_1931_color_space)
#[derive(Debug, Clone, PartialEq)]
pub struct XYZ<F: Float, W: WhitePoint<F> = D65> {
    pub x: F,
    pub y: F,
    pub z: F,
    _w: PhantomData<W>,
}

impl<F, W> XYZ<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    /// Create a color in CIE XYZ color space.
    #[inline]
    #[must_use]
    pub fn new(x: F, y: F, z: F) -> XYZ<F, W> {
        Self {
            x: Self::normalize_x(x),
            y: Self::normalize_y(y),
            z: Self::normalize_z(z),
            _w: PhantomData::default(),
        }
    }

    /// Return min value of x.
    #[inline]
    #[must_use]
    pub(crate) fn min_x<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Return max value of x.
    #[inline]
    #[must_use]
    pub(crate) fn max_x<T: Float>() -> T {
        T::from_f64(0.950456)
    }

    /// Return min value of y.
    #[inline]
    #[must_use]
    pub(crate) fn min_y<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Return max value of y.
    #[inline]
    #[must_use]
    pub(crate) fn max_y<T: Float>() -> T {
        T::from_f64(1.0)
    }

    /// Return min value of z.
    #[inline]
    #[must_use]
    pub(crate) fn min_z<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Return max value of z.
    #[inline]
    #[must_use]
    pub(crate) fn max_z<T: Float>() -> T {
        T::from_f64(1.088644)
    }

    #[inline]
    #[must_use]
    fn normalize_x(value: F) -> F {
        value.clamp(Self::min_x(), Self::max_x())
    }

    #[inline]
    #[must_use]
    fn normalize_y(value: F) -> F {
        value.clamp(Self::min_y(), Self::max_y())
    }

    #[inline]
    #[must_use]
    fn normalize_z(value: F) -> F {
        value.clamp(Self::min_z(), Self::max_z())
    }
}

impl<F, W> Display for XYZ<F, W>
where
    F: Float + Display,
    W: WhitePoint<F>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "XYZ({x}, {y}, {z})", x = self.x, y = self.y, z = self.z)
    }
}

impl<F, W> From<&Rgba> for XYZ<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    #[inline]
    fn from(rgba: &Rgba) -> Self {
        let f = |value: F| -> F {
            if value <= F::from_f64(0.04045) {
                value / F::from_f64(12.92)
            } else {
                ((value + F::from_f64(0.055)) / F::from_f64(1.055)).powf(F::from_f64(2.4))
            }
        };

        let max_value: F = Rgba::max_value();
        let r = f(rgba.r::<F>() / max_value);
        let g = f(rgba.g::<F>() / max_value);
        let b = f(rgba.b::<F>() / max_value);

        let x = F::from_f64(0.412391) * r + F::from_f64(0.357584) * g + F::from_f64(0.180481) * b;
        let y = F::from_f64(0.212639) * r + F::from_f64(0.715169) * g + F::from_f64(0.072192) * b;
        let z = F::from_f64(0.019331) * r + F::from_f64(0.119195) * g + F::from_f64(0.950532) * b;
        XYZ::new(x, y, z)
    }
}

impl<F, W> From<&Lab<F>> for XYZ<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    #[inline]
    fn from(lab: &Lab<F>) -> Self {
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

        let x = W::x() * f(l2 + a2);
        let y = W::y() * f(l2);
        let z = W::z() * f(l2 - b2);
        XYZ::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;

    #[test]
    fn new_should_create_xyz_color() {
        let xyz: XYZ<f64, D65> = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(xyz.x, 0.256394);
        assert_eq!(xyz.y, 0.223987);
        assert_eq!(xyz.z, 0.975798);

        let xyz: XYZ<f64, D65> = XYZ::new(-1.0, -1.0, -1.0);
        assert_eq!(xyz.x, 0.0);
        assert_eq!(xyz.y, 0.0);
        assert_eq!(xyz.z, 0.0);

        let xyz: XYZ<f64, D65> = XYZ::new(1.0, 1.1, 1.2);
        assert_eq!(xyz.x, 0.950456);
        assert_eq!(xyz.y, 1.0);
        assert_eq!(xyz.z, 1.088644);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let xyz: XYZ<f64, D65> = XYZ::new(0.256394, 0.223987, 0.975798);
        assert_eq!(xyz.to_string(), "XYZ(0.256394, 0.223987, 0.975798)");
    }

    #[test]
    fn from_rgba_should_convert_to_xyz() {
        let black = Rgba::black();
        let actual: XYZ<f64, D65> = XYZ::from(&black);
        assert_eq!(actual, XYZ::<f64, D65>::new(0.0, 0.0, 0.0));

        let white = Rgba::white();
        let actual: XYZ<f64, D65> = XYZ::from(&white);
        assert_close_to!(actual.x, 0.950456);
        assert_close_to!(actual.y, 1.0);
        assert_close_to!(actual.z, 1.088644);

        let red = Rgba::red();
        let actual: XYZ<f64, D65> = XYZ::from(&red);
        assert_close_to!(actual.x, 0.412391);
        assert_close_to!(actual.y, 0.212639);
        assert_close_to!(actual.z, 0.019331);

        let green = Rgba::green();
        let actual: XYZ<f64, D65> = XYZ::from(&green);
        assert_close_to!(actual.x, 0.357584);
        assert_close_to!(actual.y, 0.715169);
        assert_close_to!(actual.z, 0.119195);

        let blue = Rgba::blue();
        let actual: XYZ<f64, D65> = XYZ::from(&blue);
        assert_close_to!(actual.x, 0.180481);
        assert_close_to!(actual.y, 0.072192);
        assert_close_to!(actual.z, 0.950532);

        let transparent = Rgba::transparent();
        let actual: XYZ<f64, D65> = XYZ::from(&transparent);
        assert_eq!(actual, XYZ::<f64, D65>::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn from_lab_should_convert_to_xyz() {
        let black = Lab::new(0.0, 0.0, 0.0);
        let actual: XYZ<f64, D65> = XYZ::from(&black);
        assert_eq!(actual, XYZ::<f64, D65>::new(0.0, 0.0, 0.0));

        let white = Lab::new(100.0, 0.0, 0.0);
        let actual: XYZ<f64, D65> = XYZ::from(&white);
        assert_close_to!(actual.x, 0.950456);
        assert_close_to!(actual.y, 1.0);
        assert_close_to!(actual.z, 1.088644);

        let red = Lab::new(53.237114, 80.089636, 67.203135);
        let actual: XYZ<f64, D65> = XYZ::from(&red);
        assert_close_to!(actual.x, 0.412391);
        assert_close_to!(actual.y, 0.212639);
        assert_close_to!(actual.z, 0.019331);

        let green = Lab::new(87.735534, -86.182293, 83.186653);
        let actual: XYZ<f64, D65> = XYZ::from(&green);
        assert_close_to!(actual.x, 0.357584);
        assert_close_to!(actual.y, 0.715169);
        assert_close_to!(actual.z, 0.119195);

        let blue = Lab::new(32.300802, 79.195275, -107.855445);
        let actual: XYZ<f64, D65> = XYZ::from(&blue);
        assert_close_to!(actual.x, 0.180481);
        assert_close_to!(actual.y, 0.072192);
        assert_close_to!(actual.z, 0.950532);
    }
}
