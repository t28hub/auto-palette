use crate::lab::Lab;
use crate::math::number::Float;
use crate::rgb::Rgb;
use crate::white_point::WhitePoint;
use crate::xyz::XYZ;

/// Trait representing a color.
pub trait Color {
    type F: Float;
    type WP: WhitePoint<Self::F>;

    /// Returns the brightness of this color.
    ///
    /// # Returns
    /// The brightness of this color.
    ///
    /// # References
    /// * [Techniques For Accessibility Evaluation And Repair Tools](https://www.w3.org/TR/AERT/#color-contrast)
    fn darkness(&self) -> Self::F;

    /// Returns `true` if this color is light, `false` otherwise.
    ///
    /// # Returns
    /// `true` if this color is light, `false` otherwise.
    #[must_use]
    fn is_light(&self) -> bool;

    /// Returns `true` if this color is dark, `false` otherwise.
    ///
    /// # Returns
    /// `true` if this color is dark, `false` otherwise.
    #[must_use]
    fn is_dark(&self) -> bool {
        !self.is_light()
    }

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
    fn to_hex_string(&self) -> String;

    /// Returns an RGB string representation of this color.
    ///
    /// # Returns
    /// An RGB string representation of this color.
    #[must_use]
    fn to_rgb_string(&self) -> String;
}
