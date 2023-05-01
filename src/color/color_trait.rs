use crate::delta_e::DeltaE;
use crate::lab::Lab;
use crate::math::number::Float;
use crate::rgb::Rgb;
use crate::white_point::WhitePoint;
use crate::xyz::XYZ;

/// Trait representing a color.
pub trait Color: Clone + Default + PartialEq {
    type F: Float + Default;
    type WP: WhitePoint<Self::F>;

    /// Returns the brightness of this color.
    ///
    /// # Returns
    /// The brightness of this color.
    ///
    /// # References
    /// * [Techniques For Accessibility Evaluation And Repair Tools](https://www.w3.org/TR/AERT/#color-contrast)
    #[must_use]
    fn darkness(&self) -> Self::F {
        let rgb = self.to_rgb();
        let r = Self::F::from_f64(299.0) * rgb.r();
        let g = Self::F::from_f64(587.0) * rgb.g();
        let b = Self::F::from_f64(114.0) * rgb.b();
        (r + g + b) / Self::F::from_f64(1000.0) / Rgb::max_value()
    }

    /// Returns `true` if this color is light, `false` otherwise.
    ///
    /// # Returns
    /// `true` if this color is light, `false` otherwise.
    #[must_use]
    fn is_light(&self) -> bool {
        self.darkness() > Self::F::from_f64(0.5)
    }

    /// Returns `true` if this color is dark, `false` otherwise.
    ///
    /// # Returns
    /// `true` if this color is dark, `false` otherwise.
    #[must_use]
    fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Computes the delta E between this color and another color.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `metric` - The metric to use.
    ///
    /// # Returns
    /// The delta E between this color and another color.
    #[must_use]
    fn delta_e(&self, other: &Self, metric: DeltaE) -> Self::F {
        let lab1 = self.to_lab();
        let lab2 = other.to_lab();
        metric.measure(&lab1, &lab2)
    }

    /// Mixes this color with another color.
    ///
    /// # Arguments
    /// * `other` - The other color.
    /// * `fraction` - The fraction of the other color to mix into this color.
    ///
    /// # Returns
    /// The mixed color.
    #[must_use]
    fn mix(&self, other: &Self, fraction: Self::F) -> Self;

    /// Returns an RGB representation of this color.
    ///
    /// # Returns
    /// An RGB representation of this color.
    #[must_use]
    fn to_rgb(&self) -> Rgb;

    /// Returns a CIE XYZ representation of this color.
    ///
    /// # Returns
    /// A CIE XYZ representation of this color.
    #[must_use]
    fn to_xyz(&self) -> XYZ<Self::F, Self::WP>;

    /// Returns a CIE L*a*b* representation of this color.
    ///
    /// # Returns
    /// A CIE L*a*b* representation of this color.
    #[must_use]
    fn to_lab(&self) -> Lab<Self::F, Self::WP>;

    /// Returns a hex string representation of this color.
    ///
    /// # Returns
    /// A hex string representation of this color.
    #[must_use]
    fn to_hex_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b)
    }

    /// Returns an RGB string representation of this color.
    ///
    /// # Returns
    /// An RGB string representation of this color.
    #[must_use]
    fn to_rgb_string(&self) -> String {
        let rgb = self.to_rgb();
        format!("rgb({} {} {})", rgb.r, rgb.g, rgb.b)
    }
}
