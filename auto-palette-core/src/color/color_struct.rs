use crate::delta_e::DeltaE;
use crate::lab::Lab;
use crate::math::number::Float;
use crate::rgb::RGB;
use crate::white_point::{WhitePoint, D65};
use crate::xyz::XYZ;
use std::fmt::{Display, Formatter, Result};
use std::marker::PhantomData;

/// Struct representing a color.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # Examples
/// ```
/// use statrs::assert_almost_eq;
/// use auto_palette::color_struct::Color;
/// use auto_palette::rgb::RGB;
///
/// let yellow = RGB::new(255, 255, 0);
/// let color = Color::<f64>::from(&yellow);
/// assert_eq!(color.is_light(), true);
/// assert_eq!(color.is_dark(), false);
/// assert_almost_eq!(color.lightness(), 97.1385, 1e-4);
/// assert_almost_eq!(color.chroma(), 96.9126, 1e-4);
/// assert_almost_eq!(color.hue(), 102.8544, 1e-4);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Color<F: Float, WP = D65> {
    l: F,
    a: F,
    b: F,
    _marker: PhantomData<WP>,
}

impl<F, WP> Color<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    /// Creates a new `Color` instance.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[must_use]
    fn new(l: F, a: F, b: F) -> Self {
        Self {
            l,
            a,
            b,
            _marker: PhantomData::default(),
        }
    }

    /// Returns whether this color is light.
    ///
    /// # Returns
    /// `true` if this color is light, otherwise `false`.
    #[inline]
    #[must_use]
    pub fn is_light(&self) -> bool {
        self.l > F::from_f64(50.0)
    }

    /// Returns whether this color is dark.
    ///
    /// # Returns
    /// `true` if this color is dark, otherwise `false`.
    #[inline]
    #[must_use]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Returns the lightness of this color.
    /// The range of the lightness is [0, 100).
    ///
    /// # Returns
    /// The lightness of this color.
    #[inline]
    #[must_use]
    pub fn lightness(&self) -> F {
        self.l
    }

    /// Returns the chroma of this color.
    /// The range of the chroma is [0, 128).
    ///
    /// # Returns
    /// The chroma of this color.
    #[inline]
    #[must_use]
    pub fn chroma(&self) -> F {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the hue angle of this color.
    /// The range of the hue angle is [0, 360).
    ///
    /// # Returns
    /// The hue of this color.
    #[inline]
    #[must_use]
    pub fn hue(&self) -> F {
        let hue = self.b.atan2(self.a).to_degrees();
        if hue < F::zero() {
            hue + F::from_f64(360.0)
        } else {
            hue
        }
    }

    /// Mixes this color with another color with the given ratio.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `ratio` - The ratio of the other color.
    ///
    /// # Returns
    /// A mixed color.
    #[inline]
    #[must_use]
    pub fn mix(&self, other: &Color<F, WP>, ratio: F) -> Color<F, WP> {
        let l = self.l + (other.l - self.l) * ratio;
        let a = self.a + (other.a - self.a) * ratio;
        let b = self.b + (other.b - self.b) * ratio;
        let lab = Lab::<F, WP>::new(l, a, b);
        Self::new(lab.l, lab.a, lab.b)
    }

    /// Calculates the color difference between this color and another color.
    /// The color difference is calculated by the given delta E metric.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `metric` - The delta E metric.
    ///
    /// # Returns
    /// The color difference.
    #[inline]
    #[must_use]
    pub fn difference(&self, other: &Color<F, WP>, metric: &DeltaE) -> F {
        let lab1 = self.to_lab();
        let lab2 = other.to_lab();
        metric.measure(&lab1, &lab2)
    }

    /// Converts this color to an RGB color.
    ///
    /// # Returns
    /// A converted RGB color.
    #[inline]
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        RGB::from(&self.to_xyz())
    }

    /// Converts this color to an XYZ color.
    ///
    /// # Returns
    /// A converted XYZ color.
    #[inline]
    #[must_use]
    pub fn to_xyz(&self) -> XYZ<F, WP> {
        XYZ::<F, WP>::from(&self.to_lab())
    }

    /// Converts this color to a CIE L*a*b* color.
    ///
    /// # Returns
    /// A converted CIE L*a*b* color.
    #[inline]
    #[must_use]
    pub fn to_lab(&self) -> Lab<F, WP> {
        Lab::<F, WP>::new(self.l, self.a, self.b)
    }

    /// Converts this color to a hex string representation.
    ///
    /// # Returns
    /// A hex string representation.
    #[inline]
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }
}

impl<F, WP> Display for Color<F, WP>
where
    F: Float + Display,
    WP: WhitePoint<F>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Lab({l:.4}, {a:.4}, {b:.4})",
            l = self.l,
            a = self.a,
            b = self.b
        )
    }
}

impl<F, WP> From<&RGB> for Color<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[must_use]
    fn from(rgb: &RGB) -> Self {
        let xyz = XYZ::<F, WP>::from(rgb);
        let lab = Lab::<F, WP>::from(&xyz);
        Self::new(lab.l, lab.a, lab.b)
    }
}

impl<F, WP> From<&XYZ<F, WP>> for Color<F>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[must_use]
    fn from(xyz: &XYZ<F, WP>) -> Self {
        let lab = Lab::<F, WP>::from(xyz);
        Self::new(lab.l, lab.a, lab.b)
    }
}

impl<F, WP> From<&Lab<F, WP>> for Color<F>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[must_use]
    fn from(lab: &Lab<F, WP>) -> Self {
        Self::new(lab.l, lab.a, lab.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgb::RGB;
    use statrs::assert_almost_eq;

    #[test]
    fn test_new() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        assert_eq!(color.l, 97.138572);
        assert_eq!(color.a, -21.560719);
        assert_eq!(color.b, 94.483834);
    }

    #[test]
    fn test_is_light() {
        let black = Color::<f64>::new(0.0, 0.0, 0.0);
        assert!(!black.is_light());

        let gray = Color::<f64>::new(50.0, 0.0, 0.0);
        assert!(!gray.is_light());

        let white = Color::<f64>::new(100.0, 0.0, 0.0);
        assert!(white.is_light());
    }

    #[test]
    fn test_is_dark() {
        let black = Color::<f64>::new(0.0, 0.0, 0.0);
        assert!(black.is_dark());

        let gray = Color::<f64>::new(50.0, 0.0, 0.0);
        assert!(gray.is_dark());

        let white = Color::<f64>::new(100.0, 0.0, 0.0);
        assert!(!white.is_dark());
    }

    #[test]
    fn test_lightness() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        assert_eq!(color.lightness(), 97.138572);
    }

    #[test]
    fn test_chroma() {
        let black = Color::<f64>::new(0.0, 0.0, 0.0);
        assert_eq!(black.chroma(), 0.0);

        let white = Color::<f64>::new(100.0, 0.0, 0.0);
        assert_eq!(white.chroma(), 0.0);

        let green = Color::<f64>::new(87.75, -79.44, 81.26);
        assert_almost_eq!(green.chroma(), 113.6393, 1e-4);

        let yellow = Color::<f64>::new(97.60, -15.82, 93.95);
        assert_almost_eq!(yellow.chroma(), 95.2726, 1e-4);
    }

    #[test]
    fn test_hue() {
        let black = Color::<f64>::new(0.0, 0.0, 0.0);
        assert_eq!(black.hue(), 0.0);

        let white = Color::<f64>::new(100.0, 0.0, 0.0);
        assert_eq!(white.hue(), 0.0);

        let cyan = Color::<f64>::from(&RGB::new(0, 255, 255));
        assert_almost_eq!(cyan.hue(), 196.3761, 1e-4);

        let pink = Color::<f64>::from(&RGB::new(255, 0, 128));
        assert_almost_eq!(pink.hue(), 2.7591, 1e-4);
    }

    #[test]
    fn test_mix() {
        let red = Color::<f64>::from(&RGB::new(255, 0, 0));
        let yellow = Color::<f64>::from(&RGB::new(255, 255, 0));
        assert_eq!(red.mix(&yellow, 1.0).to_rgb(), RGB::new(255, 255, 0));
        assert_eq!(red.mix(&yellow, 0.8).to_rgb(), RGB::new(255, 219, 0));
        assert_eq!(red.mix(&yellow, 0.5).to_rgb(), RGB::new(255, 161, 0));
        assert_eq!(red.mix(&yellow, 0.0).to_rgb(), RGB::new(255, 0, 0));
    }

    #[test]
    fn test_difference() {
        let black = Color::<f64>::new(0.0, 0.0, 0.0);
        let white = Color::<f64>::new(100.0, 0.0, 0.0);
        let green = Color::<f64>::new(87.75, -79.44, 81.26);
        assert_almost_eq!(black.difference(&green, &DeltaE::CIE2000), 87.7154, 1e-4);
        assert_almost_eq!(white.difference(&green, &DeltaE::CIE2000), 32.7989, 1e-4);
    }

    #[test]
    fn test_to_rgb() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        let rgb = color.to_rgb();
        assert_eq!(rgb.r, 255);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_to_xyz() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        let xyz = color.to_xyz();
        assert_almost_eq!(xyz.x, 0.769975, 1e-6);
        assert_almost_eq!(xyz.y, 0.927808, 1e-6);
        assert_almost_eq!(xyz.z, 0.138525, 1e-6);
    }

    #[test]
    fn test_to_lab() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        let lab = color.to_lab();
        assert_almost_eq!(lab.l, 97.138572, 1e-6);
        assert_almost_eq!(lab.a, -21.560719, 1e-6);
        assert_almost_eq!(lab.b, 94.483834, 1e-6);
    }

    #[test]
    fn test_to_hex_string() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        assert_eq!(color.to_hex_string(), "#ffff00");
    }

    #[test]
    fn test_fmt() {
        let color = Color::<f64>::new(97.138572, -21.560719, 94.483834);
        assert_eq!(format!("{}", color), "Lab(97.1386, -21.5607, 94.4838)");
    }

    #[test]
    fn test_from_rgb() {
        let rgb = RGB::new(255, 255, 0);
        let color = Color::<f64>::from(&rgb);
        assert_almost_eq!(color.l, 97.138572, 1e-6);
        assert_almost_eq!(color.a, -21.560719, 1e-6);
        assert_almost_eq!(color.b, 94.483834, 1e-6);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::<f64>::new(0.769975, 0.927808, 0.138525);
        let color = Color::<f64>::from(&xyz);
        assert_almost_eq!(color.l, 97.138572, 1e-6);
        assert_almost_eq!(color.a, -21.560719, 1e-6);
        assert_almost_eq!(color.b, 94.484076, 1e-6);
    }

    #[test]
    fn test_from_lab() {
        let lab = Lab::<f64>::new(97.138572, -21.560719, 94.483834);
        let color = Color::<f64>::from(&lab);
        assert_eq!(color.l, 97.138572);
        assert_eq!(color.a, -21.560719);
        assert_eq!(color.b, 94.483834);
    }
}
