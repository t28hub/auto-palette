use crate::color::white_point::WhitePoint;
use crate::color::xyz::XYZ;
use crate::color_trait::Color;
use crate::delta_e::DeltaE;
use crate::math::number::Float;
use crate::rgb::Rgb;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Struct representing a color in CIE L*a*b* color space.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # References
/// * [CIELAB color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Lab<F: Float, WP: WhitePoint<F>> {
    pub l: F,
    pub a: F,
    pub b: F,
    _marker: PhantomData<WP>,
}

impl<F, WP> Lab<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    /// Creates a new CIE L*a*b* color.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new CIE L*a*b* color.
    #[inline]
    #[must_use]
    pub fn new(l: F, a: F, b: F) -> Self {
        Self {
            l: Self::clamp_l(l),
            a: Self::clamp_a(a),
            b: Self::clamp_b(b),
            _marker: PhantomData::default(),
        }
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    #[inline]
    #[must_use]
    pub fn chroma(&self) -> F {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the min value of l.
    ///
    /// # Returns
    /// The min value of l.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn min_l<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Returns the max value of l.
    ///
    /// # Returns
    /// The max value of l.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn max_l<T: Float>() -> T {
        T::from_f64(100.0)
    }

    /// Returns the min value of a.
    ///
    /// # Returns
    /// The min value of a.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn min_a<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Returns the max value of a.
    ///
    /// # Returns
    /// The max value of a.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn max_a<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Returns max value of b.
    ///
    /// # Returns
    /// The max value of b.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn min_b<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Returns the max value of b.
    ///
    /// # Returns
    /// The max value of b.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn max_b<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Returns the min value of chroma.
    ///
    /// # Returns
    /// The min value of chroma.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn min_chroma<T: Float>() -> T {
        // sqrt(0^2 + 0^2) = 0
        T::from_f64(0.0)
    }

    /// Returns the max value of chroma.
    ///
    /// # Returns
    /// The max value of chroma.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    #[inline]
    #[must_use]
    pub(crate) fn max_chroma<T: Float>() -> T {
        // sqrt(127^2 + 127^2) = 179.605
        T::from_f64(128.0)
    }

    #[inline]
    #[must_use]
    fn clamp_l(value: F) -> F {
        value.clamp(Self::min_l(), Self::max_l())
    }

    #[inline]
    #[must_use]
    fn clamp_a(value: F) -> F {
        value.clamp(Self::min_a(), Self::max_a())
    }

    #[inline]
    #[must_use]
    fn clamp_b(value: F) -> F {
        value.clamp(Self::min_b(), Self::max_b())
    }
}

impl<F, WP> Display for Lab<F, WP>
where
    F: Float + Display,
    WP: WhitePoint<F>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Lab({l}, {a}, {b})", l = self.l, a = self.a, b = self.b)
    }
}

impl<F, W> From<&XYZ<F, W>> for Lab<F, W>
where
    F: Float + Default,
    W: WhitePoint<F>,
{
    #[inline]
    fn from(xyz: &XYZ<F, W>) -> Self {
        let epsilon = F::from_f64(6.0 / 29.0).powi(3);
        let kappa = F::from_f64(841.0 / 108.0); // ((29.0 / 6.0) ^ 2) / 3.0
        let delta = F::from_f64(4.0 / 29.0);
        let f = |t: F| -> F {
            if t > (epsilon) {
                t.cbrt()
            } else {
                kappa * t + delta
            }
        };

        let fx = f(xyz.x / W::x());
        let fy = f(xyz.y / W::y());
        let fz = f(xyz.z / W::z());

        let l = F::from_f64(116.0) * fy - F::from_f64(16.0);
        let a = F::from_f64(500.0) * (fx - fy);
        let b = F::from_f64(200.0) * (fy - fz);
        Lab::new(l, a, b)
    }
}

impl<F, WP> Color for Lab<F, WP>
where
    F: Float + Default,
    WP: WhitePoint<F>,
{
    type F = F;
    type WP = WP;

    #[must_use]
    fn delta_e(&self, other: &Self, metric: DeltaE) -> Self::F {
        metric.measure(self, other)
    }

    #[must_use]
    fn mix(&self, other: &Self, fraction: Self::F) -> Self {
        let l = self.l + (other.l - self.l) * fraction;
        let a = self.a + (other.a - self.a) * fraction;
        let b = self.b + (other.b - self.b) * fraction;
        Lab::new(l, a, b)
    }

    #[must_use]
    fn to_rgb(&self) -> Rgb {
        let xyz = self.to_xyz();
        Rgb::from(&xyz)
    }

    #[must_use]
    fn to_xyz(&self) -> XYZ<Self::F, Self::WP> {
        XYZ::<F, WP>::from(self)
    }

    #[must_use]
    fn to_lab(&self) -> Lab<Self::F, Self::WP> {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::white_point::D65;
    use rstest::rstest;
    use statrs::assert_almost_eq;

    #[test]
    fn test_lab() {
        let lab: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(lab.l, 53.23);
        assert_eq!(lab.a, 80.11);
        assert_eq!(lab.b, 67.22);

        let lab: Lab<f64, D65> = Lab::new(-4.0, -192.0, -192.0);
        assert_eq!(lab.l, 0.0);
        assert_eq!(lab.a, -128.0);
        assert_eq!(lab.b, -128.0);

        let lab: Lab<f64, D65> = Lab::new(108.0, 128.0, 128.0);
        assert_eq!(lab.l, 100.0);
        assert_eq!(lab.a, 127.0);
        assert_eq!(lab.b, 127.0);
    }

    #[test]
    fn test_chroma() {
        let lab: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        assert_almost_eq!(lab.chroma(), 104.576, 1e-3);
    }

    #[test]
    fn test_delta_e() {
        let lab1: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        let lab2: Lab<f64, D65> = Lab::new(-4.0, -192.0, -192.0);
        assert_almost_eq!(lab1.delta_e(&lab2, DeltaE::CIE76), 290.265, 1e-3);
    }

    #[test]
    fn test_to_string() {
        let lab: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(lab.to_string(), "Lab(53.23, 80.11, 67.22)");
    }

    #[rstest]
    #[case((0.0000, 0.0000, 0.0000), (0.0, 0.0, 0.0))] // Black
    #[case((0.9505, 1.0000, 1.0890), (100.0, 0.0, 0.0254))] // White
    #[case((0.4124, 0.2126, 0.0193), (53.2371, 80.1106, 67.2237))] // Red
    #[case((0.3576, 0.7152, 0.1192), (87.7355, - 86.1822, 83.1866))] // Green
    #[case((0.1805, 0.0722, 0.9505), (32.3008, 79.1952, - 107.8554))] // Blue
    #[case((0.5381, 0.7874, 1.0697), (91.1132, - 48.0875, - 14.1312))] // Cyan
    #[case((0.5929, 0.2848, 0.9698), (60.3242, 98.2557, - 60.8249))] // Magenta
    #[case((0.7700, 0.9278, 0.1385), (97.1393, - 21.5537, 94.4896))] // Yellow
    fn test_from_xyz(#[case] xyz: (f64, f64, f64), #[case] expected: (f64, f64, f64)) {
        let actual: Lab<_, D65> = Lab::from(&XYZ::new(xyz.0, xyz.1, xyz.2));
        let (l, a, b) = expected;
        assert_almost_eq!(actual.l, l, 0.01);
        assert_almost_eq!(actual.a, a, 0.01);
        assert_almost_eq!(actual.b, b, 0.01);
    }
}
