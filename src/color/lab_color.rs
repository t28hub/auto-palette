use crate::color_trait::Color;
use crate::lab::Lab;
use crate::math::number::Float;
use crate::rgb::Rgb;
use crate::white_point::{WhitePoint, D65};
use crate::xyz::XYZ;

/// Struct representing a color in CIE L*a*b* color space.
///
/// # Type Parameters
/// * `F` - The floating point type.
/// * `WP` - The white point.
///
/// # References
/// * [CIELAB color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
#[derive(Debug, Clone, PartialEq)]
pub struct LabColor<F: Float, WP: WhitePoint<F> = D65> {
    lab: Lab<F, WP>,
}

impl<F, WP> LabColor<F, WP>
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
            lab: Lab::new(l, a, b),
        }
    }
}

impl<F, WP> From<&Lab<F, WP>> for LabColor<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[inline]
    #[must_use]
    fn from(value: &Lab<F, WP>) -> Self {
        LabColor::new(value.l, value.a, value.b)
    }
}

impl<F, WP> Color for LabColor<F, WP>
where
    F: Float,
    WP: WhitePoint<F>,
{
    type F = F;
    type WP = WP;

    #[must_use]
    fn darkness(&self) -> F {
        let rgb = self.to_rgb();
        (F::from_f64(299.0) * rgb.r() + F::from_f64(587.0) * rgb.g() + F::from_f64(114.0) * rgb.b())
            / F::from_f64(1000.0)
            / Rgb::max_value()
    }

    #[inline]
    #[must_use]
    fn is_light(&self) -> bool {
        self.darkness() > F::from_f64(0.5)
    }

    #[must_use]
    fn to_rgb(&self) -> Rgb {
        let xyz = self.to_xyz();
        Rgb::from(&xyz)
    }

    #[must_use]
    fn to_xyz(&self) -> XYZ<F, WP> {
        XYZ::from(&self.lab)
    }

    #[must_use]
    fn to_lab(&self) -> Lab<F, WP> {
        self.lab.clone()
    }

    #[must_use]
    fn to_hex_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }

    #[must_use]
    fn to_rgb_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("rgb({} {} {})", rgb.r, rgb.g, rgb.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use statrs::assert_almost_eq;

    #[test]
    fn test_lab_color() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        assert_eq!(color.to_rgb(), Rgb::new(246, 191, 0));
    }

    #[test]
    fn test_from_lab() {
        let lab = Lab::<f64, D65>::new(80.098, 6.535, 81.957);
        let color = LabColor::<f64, D65>::from(&lab);
        assert_eq!(color.to_rgb(), Rgb::new(246, 191, 0));
    }

    #[test]
    fn test_darkness() {
        let color = LabColor::<f64, D65>::new(0.0, 0.0, 0.0);
        assert_eq!(color.darkness(), 0.0);

        let color = LabColor::<f64, D65>::new(50.0, 0.0, 0.0);
        assert_almost_eq!(color.darkness(), 0.46, 0.01);

        let color = LabColor::<f64, D65>::new(100.0, 0.0, 0.0);
        assert_eq!(color.darkness(), 1.0);
    }

    #[test]
    fn test_is_light() {
        let color = LabColor::<f64, D65>::new(0.0, 0.0, 0.0);
        assert!(!color.is_light());

        let color = LabColor::<f64, D65>::new(50.0, 0.0, 0.0);
        assert!(!color.is_light());

        let color = LabColor::<f64, D65>::new(100.0, 0.0, 0.0);
        assert!(color.is_light());
    }

    #[test]
    fn test_to_rgb() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        assert_eq!(color.to_rgb(), Rgb::new(246, 191, 0));

        let color = LabColor::<f64, D65>::new(64.253, 57.011, 14.351);
        assert_eq!(color.to_rgb(), Rgb::new(252, 108, 133));
    }

    #[test]
    fn test_to_xyz() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        let actual = color.to_xyz();
        assert_almost_eq!(actual.x, 0.56637, 0.0001);
        assert_almost_eq!(actual.y, 0.56854, 0.0001);
        assert_almost_eq!(actual.z, 0.07989, 0.0001);

        let color = LabColor::<f64, D65>::new(64.253, 57.011, 14.351);
        let actual = color.to_xyz();
        assert_almost_eq!(actual.x, 0.49741, 0.0001);
        assert_almost_eq!(actual.y, 0.33114, 0.0001);
        assert_almost_eq!(actual.z, 0.25960, 0.0001);
    }

    #[test]
    fn test_to_lab() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        assert_eq!(color.to_lab(), Lab::new(80.098, 6.535, 81.957));

        let color = LabColor::<f64, D65>::new(64.253, 57.011, 14.351);
        assert_eq!(color.to_lab(), Lab::new(64.253, 57.011, 14.351));
    }

    #[test]
    fn test_to_hex_string() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        assert_eq!(color.to_hex_string(), "#f6bf00");

        let color = LabColor::<f64, D65>::new(64.253, 57.011, 14.351);
        assert_eq!(color.to_hex_string(), "#fc6c85");
    }

    #[test]
    fn test_to_rgb_string() {
        let color = LabColor::<f64, D65>::new(80.098, 6.535, 81.957);
        assert_eq!(color.to_rgb_string(), "rgb(246 191 0)");

        let color = LabColor::<f64, D65>::new(64.253, 57.011, 14.351);
        assert_eq!(color.to_rgb_string(), "rgb(252 108 133)");
    }
}
