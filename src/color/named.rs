use crate::rgba::Rgba;

/// Struct representing a named color.
///
/// # Examples
/// ```
/// use auto_palette::named::NamedColor;
/// use auto_palette::rgba::Rgba;
///
/// let red = NamedColor::new("red", Rgba::new(255, 0, 0, 255));
/// assert_eq!(red.name, "red");
/// assert_eq!(red.color, Rgba::new(255, 0, 0, 255));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NamedColor {
    pub name: &'static str,
    pub color: Rgba,
}

impl NamedColor {
    /// Creates a new `NamedColor` instance.
    ///
    /// # Arguments
    /// * `name` - The name of the color.
    /// * `color` - The color.
    ///
    /// # Returns
    /// A new `NamedColor` instance.
    #[must_use]
    pub const fn new(name: &'static str, color: Rgba) -> Self {
        Self { name, color }
    }

    /// Returns the name of this color.
    ///
    /// # Returns
    /// The name of this color.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns a reference to the color.
    ///
    /// # Returns
    /// A reference to the color.
    #[inline]
    #[must_use]
    pub fn color(&self) -> &Rgba {
        &self.color
    }
}

/// Creates a new `NamedColor` instance.
///
/// # Arguments
/// * `name` - The name of the color.
/// * `r` - The red component of the color.
/// * `g` - The green component of the color.
/// * `b` - The blue component of the color.
///
/// # Returns
/// A new `NamedColor` instance.
#[inline]
#[must_use]
const fn named(name: &'static str, r: u8, g: u8, b: u8) -> NamedColor {
    NamedColor::new(name, Rgba::new(r, g, b, 255))
}

/// The list of named colors defined in the CSS3 specification.
///
/// # References
/// * [6.1. Named Colors - CSS3 Color Module Level 4](https://www.w3.org/TR/css-color-4/#named-colors)
pub const COLORS: [NamedColor; 140] = [
    named("AliceBlue", 240, 248, 255),
    named("AntiqueWhite", 250, 235, 215),
    named("Aqua", 0, 255, 255),
    named("Aquamarine", 127, 255, 212),
    named("Azure", 240, 255, 255),
    named("Beige", 245, 245, 220),
    named("Bisque", 255, 228, 196),
    named("Black", 0, 0, 0),
    named("BlanchedAlmond", 255, 235, 205),
    named("Blue", 0, 0, 255),
    named("BlueViolet", 138, 43, 226),
    named("Brown", 165, 42, 42),
    named("BurlyWood", 222, 184, 135),
    named("CadetBlue", 95, 158, 160),
    named("Chartreuse", 127, 255, 0),
    named("Chocolate", 210, 105, 30),
    named("Coral", 255, 127, 80),
    named("CornflowerBlue", 100, 149, 237),
    named("Cornsilk", 255, 248, 220),
    named("Crimson", 220, 20, 60),
    named("Cyan", 0, 255, 255),
    named("DarkBlue", 0, 0, 139),
    named("DarkCyan", 0, 139, 139),
    named("DarkGoldenrod", 184, 134, 11),
    named("DarkGray", 169, 169, 169),
    named("DarkGreen", 0, 100, 0),
    named("DarkKhaki", 189, 183, 107),
    named("DarkMagenta", 139, 0, 139),
    named("DarkOliveGreen", 85, 107, 47),
    named("DarkOrange", 255, 140, 0),
    named("DarkOrchid", 153, 50, 204),
    named("DarkRed", 139, 0, 0),
    named("DarkSalmon", 233, 150, 122),
    named("DarkSeaGreen", 143, 188, 143),
    named("DarkSlateBlue", 72, 61, 139),
    named("DarkSlateGray", 47, 79, 79),
    named("DarkTurquoise", 0, 206, 209),
    named("DarkViolet", 148, 0, 211),
    named("DeepPink", 255, 20, 147),
    named("DeepSkyBlue", 0, 191, 255),
    named("DimGray", 105, 105, 105),
    named("DodgerBlue", 30, 144, 255),
    named("Firebrick", 178, 34, 34),
    named("FloralWhite", 255, 250, 240),
    named("ForestGreen", 34, 139, 34),
    named("Fuchsia", 255, 0, 255),
    named("Gainsboro", 220, 220, 220),
    named("GhostWhite", 248, 248, 255),
    named("Gold", 255, 215, 0),
    named("Goldenrod", 218, 165, 32),
    named("Gray", 128, 128, 128),
    named("Green", 0, 128, 0),
    named("GreenYellow", 173, 255, 47),
    named("Honeydew", 240, 255, 240),
    named("HotPink", 255, 105, 180),
    named("IndianRed", 205, 92, 92),
    named("Indigo", 75, 0, 130),
    named("Ivory", 255, 255, 240),
    named("Khaki", 240, 230, 140),
    named("Lavender", 230, 230, 250),
    named("LavenderBlush", 255, 240, 245),
    named("LawnGreen", 124, 252, 0),
    named("LemonChiffon", 255, 250, 205),
    named("LightBlue", 173, 216, 230),
    named("LightCoral", 240, 128, 128),
    named("LightCyan", 224, 255, 255),
    named("LightGoldenrodYellow", 250, 250, 210),
    named("LightGray", 211, 211, 211),
    named("LightGreen", 144, 238, 144),
    named("LightPink", 255, 182, 193),
    named("LightSalmon", 255, 160, 122),
    named("LightSeaGreen", 32, 178, 170),
    named("LightSkyBlue", 135, 206, 250),
    named("LightSlateGray", 119, 136, 153),
    named("LightSteelBlue", 176, 196, 222),
    named("LightYellow", 255, 255, 224),
    named("Lime", 0, 255, 0),
    named("LimeGreen", 50, 205, 50),
    named("Linen", 250, 240, 230),
    named("Magenta", 255, 0, 255),
    named("Maroon", 128, 0, 0),
    named("MediumAquamarine", 102, 205, 170),
    named("MediumBlue", 0, 0, 205),
    named("MediumOrchid", 186, 85, 211),
    named("MediumPurple", 147, 112, 219),
    named("MediumSeaGreen", 60, 179, 113),
    named("MediumSlateBlue", 123, 104, 238),
    named("MediumSpringGreen", 0, 250, 154),
    named("MediumTurquoise", 72, 209, 204),
    named("MediumVioletRed", 199, 21, 133),
    named("MidnightBlue", 25, 25, 112),
    named("MintCream", 245, 255, 250),
    named("MistyRose", 255, 228, 225),
    named("Moccasin", 255, 228, 181),
    named("NavajoWhite", 255, 222, 173),
    named("Navy", 0, 0, 128),
    named("OldLace", 253, 245, 230),
    named("Olive", 128, 128, 0),
    named("OliveDrab", 107, 142, 35),
    named("Orange", 255, 165, 0),
    named("OrangeRed", 255, 69, 0),
    named("Orchid", 218, 112, 214),
    named("PaleGoldenrod", 238, 232, 170),
    named("PaleGreen", 152, 251, 152),
    named("PaleTurquoise", 175, 238, 238),
    named("PaleVioletRed", 219, 112, 147),
    named("PapayaWhip", 255, 239, 213),
    named("PeachPuff", 255, 218, 185),
    named("Peru", 205, 133, 63),
    named("Pink", 255, 192, 203),
    named("Plum", 221, 160, 221),
    named("PowderBlue", 176, 224, 230),
    named("Purple", 128, 0, 128),
    named("Red", 255, 0, 0),
    named("RosyBrown", 188, 143, 143),
    named("RoyalBlue", 65, 105, 225),
    named("SaddleBrown", 139, 69, 19),
    named("Salmon", 250, 128, 114),
    named("SandyBrown", 244, 164, 96),
    named("SeaGreen", 46, 139, 87),
    named("SeaShell", 255, 245, 238),
    named("Sienna", 160, 82, 45),
    named("Silver", 192, 192, 192),
    named("SkyBlue", 135, 206, 235),
    named("SlateBlue", 106, 90, 205),
    named("SlateGray", 112, 128, 144),
    named("Snow", 255, 250, 250),
    named("SpringGreen", 0, 255, 127),
    named("SteelBlue", 70, 130, 180),
    named("Tan", 210, 180, 140),
    named("Teal", 0, 128, 128),
    named("Thistle", 216, 191, 216),
    named("Tomato", 255, 99, 71),
    named("Turquoise", 64, 224, 208),
    named("Violet", 238, 130, 238),
    named("Wheat", 245, 222, 179),
    named("White", 255, 255, 255),
    named("WhiteSmoke", 245, 245, 245),
    named("Yellow", 255, 255, 0),
    named("YellowGreen", 154, 205, 50),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color() {
        let name = "Red";
        let color = Rgba::new(255, 0, 0, 255);
        let actual = NamedColor::new(name, color.clone());

        assert_eq!(actual.name, name);
        assert_eq!(actual.color, color);
    }
}
