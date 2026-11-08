/// A color in gamma-encoded 8-bit sRGB.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Srgb8 {
    /// The red value, from 0 to 255.
    pub(crate) r: u8,
    /// The green value, from 0 to 255.
    pub(crate) g: u8,
    /// The blue value, from 0 to 255.
    pub(crate) b: u8,
}

impl Srgb8 {
    /// Creates a color in gamma-encoded sRGB from its 8-bit red, green, and blue values.
    ///
    /// # Arguments
    /// * `r` - The red value, from 0 to 255.
    /// * `g` - The green value, from 0 to 255.
    /// * `b` - The blue value, from 0 to 255.
    ///
    /// # Returns
    /// An 8-bit sRGB color with the given `r`, `g`, and `b` values.
    #[inline]
    pub(crate) const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}
