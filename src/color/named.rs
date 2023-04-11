use crate::color_trait::Color;
use crate::lab::Lab;
use crate::rgb::Rgb;
use crate::white_point::D65;
use crate::xyz::XYZ;
use std::fmt::{Display, Formatter, Result};

/// Struct representing a named color.
///
/// # Examples
/// ```
/// use auto_palette::color_trait::Color;
/// use auto_palette::named::NamedColor;
/// use auto_palette::rgb::Rgb;
///
/// let orange = NamedColor::new("orange", 255, 165, 0);
/// assert_eq!(orange.name(), "orange");
/// assert_eq!(orange.to_rgb(), Rgb::new(255, 165, 0));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedColor {
    name: &'static str,
    r: u8,
    g: u8,
    b: u8,
}

impl NamedColor {
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
    #[must_use]
    pub const fn new(name: &'static str, r: u8, g: u8, b: u8) -> Self {
        Self { name, r, g, b }
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
}

impl Display for NamedColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.name)
    }
}

impl Color for NamedColor {
    type F = f64;
    type WP = D65;

    #[inline]
    #[must_use]
    fn to_rgb(&self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
    }

    #[must_use]
    fn to_xyz(&self) -> XYZ<Self::F, Self::WP> {
        XYZ::from(&self.to_rgb())
    }

    #[must_use]
    fn to_lab(&self) -> Lab<Self::F, Self::WP> {
        Lab::from(&self.to_xyz())
    }
}

/// The list of 16 basic colors.
///
/// # References
/// * [CSS/Properties/color/keywords](https://www.w3.org/wiki/CSS/Properties/color/keywords#Basic_Colors)
pub const BASIC_COLORS: [NamedColor; 16] = [
    NamedColor::new("black", 0, 0, 0),
    NamedColor::new("silver", 192, 192, 192),
    NamedColor::new("gray", 128, 128, 128),
    NamedColor::new("white", 255, 255, 255),
    NamedColor::new("maroon", 128, 0, 0),
    NamedColor::new("red", 255, 0, 0),
    NamedColor::new("purple", 128, 0, 128),
    NamedColor::new("fuchsia", 255, 0, 255),
    NamedColor::new("green", 0, 128, 0),
    NamedColor::new("lime", 0, 255, 0),
    NamedColor::new("olive", 128, 128, 0),
    NamedColor::new("yellow", 255, 255, 0),
    NamedColor::new("navy", 0, 0, 128),
    NamedColor::new("blue", 0, 0, 255),
    NamedColor::new("teal", 0, 128, 128),
    NamedColor::new("aqua", 0, 255, 255),
];

/// The list of extended named colors defined in the CSS3 specification.
///
/// # References
/// * [CSS/Properties/color/keywords](https://www.w3.org/wiki/CSS/Properties/color/keywords#Extended_colors)
pub const EXTENDED_COLORS: [NamedColor; 148] = [
    NamedColor::new("aliceblue", 240, 248, 255),
    NamedColor::new("antiquewhite", 250, 235, 215),
    NamedColor::new("aqua", 0, 255, 255),
    NamedColor::new("aquamarine", 127, 255, 212),
    NamedColor::new("azure", 240, 255, 255),
    NamedColor::new("beige", 245, 245, 220),
    NamedColor::new("bisque", 255, 228, 196),
    NamedColor::new("black", 0, 0, 0),
    NamedColor::new("blanchedalmond", 255, 235, 205),
    NamedColor::new("blue", 0, 0, 255),
    NamedColor::new("blueviolet", 138, 43, 226),
    NamedColor::new("brown", 165, 42, 42),
    NamedColor::new("burlywood", 222, 184, 135),
    NamedColor::new("cadetblue", 95, 158, 160),
    NamedColor::new("chartreuse", 127, 255, 0),
    NamedColor::new("chocolate", 210, 105, 30),
    NamedColor::new("coral", 255, 127, 80),
    NamedColor::new("cornflowerblue", 100, 149, 237),
    NamedColor::new("cornsilk", 255, 248, 220),
    NamedColor::new("crimson", 220, 20, 60),
    NamedColor::new("cyan", 0, 255, 255),
    NamedColor::new("darkblue", 0, 0, 139),
    NamedColor::new("darkcyan", 0, 139, 139),
    NamedColor::new("darkgoldenrod", 184, 134, 11),
    NamedColor::new("darkgray", 169, 169, 169),
    NamedColor::new("darkgreen", 0, 100, 0),
    NamedColor::new("darkgrey", 169, 169, 169),
    NamedColor::new("darkkhaki", 189, 183, 107),
    NamedColor::new("darkmagenta", 139, 0, 139),
    NamedColor::new("darkolivegreen", 85, 107, 47),
    NamedColor::new("darkorange", 255, 140, 0),
    NamedColor::new("darkorchid", 153, 50, 204),
    NamedColor::new("darkred", 139, 0, 0),
    NamedColor::new("darksalmon", 233, 150, 122),
    NamedColor::new("darkseagreen", 143, 188, 143),
    NamedColor::new("darkslateblue", 72, 61, 139),
    NamedColor::new("darkslategray", 47, 79, 79),
    NamedColor::new("darkslategrey", 47, 79, 79),
    NamedColor::new("darkturquoise", 0, 206, 209),
    NamedColor::new("darkviolet", 148, 0, 211),
    NamedColor::new("deeppink", 255, 20, 147),
    NamedColor::new("deepskyblue", 0, 191, 255),
    NamedColor::new("dimgray", 105, 105, 105),
    NamedColor::new("dimgrey", 105, 105, 105),
    NamedColor::new("dodgerblue", 30, 144, 255),
    NamedColor::new("firebrick", 178, 34, 34),
    NamedColor::new("floralwhite", 255, 250, 240),
    NamedColor::new("forestgreen", 34, 139, 34),
    NamedColor::new("fuchsia", 255, 0, 255),
    NamedColor::new("gainsboro", 220, 220, 220),
    NamedColor::new("ghostwhite", 248, 248, 255),
    NamedColor::new("gold", 255, 215, 0),
    NamedColor::new("goldenrod", 218, 165, 32),
    NamedColor::new("gray", 128, 128, 128),
    NamedColor::new("green", 0, 128, 0),
    NamedColor::new("greenyellow", 173, 255, 47),
    NamedColor::new("grey", 128, 128, 128),
    NamedColor::new("honeydew", 240, 255, 240),
    NamedColor::new("hotpink", 255, 105, 180),
    NamedColor::new("indianred", 205, 92, 92),
    NamedColor::new("indigo", 75, 0, 130),
    NamedColor::new("ivory", 255, 255, 240),
    NamedColor::new("khaki", 240, 230, 140),
    NamedColor::new("lavender", 230, 230, 250),
    NamedColor::new("lavenderblush", 255, 240, 245),
    NamedColor::new("lawngreen", 124, 252, 0),
    NamedColor::new("lemonchiffon", 255, 250, 205),
    NamedColor::new("lightblue", 173, 216, 230),
    NamedColor::new("lightcoral", 240, 128, 128),
    NamedColor::new("lightcyan", 224, 255, 255),
    NamedColor::new("lightgoldenrodyellow", 250, 250, 210),
    NamedColor::new("lightgray", 211, 211, 211),
    NamedColor::new("lightgreen", 144, 238, 144),
    NamedColor::new("lightgrey", 211, 211, 211),
    NamedColor::new("lightpink", 255, 182, 193),
    NamedColor::new("lightsalmon", 255, 160, 122),
    NamedColor::new("lightseagreen", 32, 178, 170),
    NamedColor::new("lightskyblue", 135, 206, 250),
    NamedColor::new("lightslategray", 119, 136, 153),
    NamedColor::new("lightslategrey", 119, 136, 153),
    NamedColor::new("lightsteelblue", 176, 196, 222),
    NamedColor::new("lightyellow", 255, 255, 224),
    NamedColor::new("lime", 0, 255, 0),
    NamedColor::new("limegreen", 50, 205, 50),
    NamedColor::new("linen", 250, 240, 230),
    NamedColor::new("magenta", 255, 0, 255),
    NamedColor::new("maroon", 128, 0, 0),
    NamedColor::new("mediumaquamarine", 102, 205, 170),
    NamedColor::new("mediumblue", 0, 0, 205),
    NamedColor::new("mediumorchid", 186, 85, 211),
    NamedColor::new("mediumpurple", 147, 112, 219),
    NamedColor::new("mediumseagreen", 60, 179, 113),
    NamedColor::new("mediumslateblue", 123, 104, 238),
    NamedColor::new("mediumspringgreen", 0, 250, 154),
    NamedColor::new("mediumturquoise", 72, 209, 204),
    NamedColor::new("mediumvioletred", 199, 21, 133),
    NamedColor::new("midnightblue", 25, 25, 112),
    NamedColor::new("mintcream", 245, 255, 250),
    NamedColor::new("mistyrose", 255, 228, 225),
    NamedColor::new("moccasin", 255, 228, 181),
    NamedColor::new("navajowhite", 255, 222, 173),
    NamedColor::new("navy", 0, 0, 128),
    NamedColor::new("oldlace", 253, 245, 230),
    NamedColor::new("olive", 128, 128, 0),
    NamedColor::new("olivedrab", 107, 142, 35),
    NamedColor::new("orange", 255, 165, 0),
    NamedColor::new("orangered", 255, 69, 0),
    NamedColor::new("orchid", 218, 112, 214),
    NamedColor::new("palegoldenrod", 238, 232, 170),
    NamedColor::new("palegreen", 152, 251, 152),
    NamedColor::new("paleturquoise", 175, 238, 238),
    NamedColor::new("palevioletred", 219, 112, 147),
    NamedColor::new("papayawhip", 255, 239, 213),
    NamedColor::new("peachpuff", 255, 218, 185),
    NamedColor::new("peru", 205, 133, 63),
    NamedColor::new("pink", 255, 192, 203),
    NamedColor::new("plum", 221, 160, 221),
    NamedColor::new("powderblue", 176, 224, 230),
    NamedColor::new("purple", 128, 0, 128),
    NamedColor::new("rebeccapurple", 102, 51, 153),
    NamedColor::new("red", 255, 0, 0),
    NamedColor::new("rosybrown", 188, 143, 143),
    NamedColor::new("royalblue", 65, 105, 225),
    NamedColor::new("saddlebrown", 139, 69, 19),
    NamedColor::new("salmon", 250, 128, 114),
    NamedColor::new("sandybrown", 244, 164, 96),
    NamedColor::new("seagreen", 46, 139, 87),
    NamedColor::new("seashell", 255, 245, 238),
    NamedColor::new("sienna", 160, 82, 45),
    NamedColor::new("silver", 192, 192, 192),
    NamedColor::new("skyblue", 135, 206, 235),
    NamedColor::new("slateblue", 106, 90, 205),
    NamedColor::new("slategray", 112, 128, 144),
    NamedColor::new("slategrey", 112, 128, 144),
    NamedColor::new("snow", 255, 250, 250),
    NamedColor::new("springgreen", 0, 255, 127),
    NamedColor::new("steelblue", 70, 130, 180),
    NamedColor::new("tan", 210, 180, 140),
    NamedColor::new("teal", 0, 128, 128),
    NamedColor::new("thistle", 216, 191, 216),
    NamedColor::new("tomato", 255, 99, 71),
    NamedColor::new("turquoise", 64, 224, 208),
    NamedColor::new("violet", 238, 130, 238),
    NamedColor::new("wheat", 245, 222, 179),
    NamedColor::new("white", 255, 255, 255),
    NamedColor::new("whitesmoke", 245, 245, 245),
    NamedColor::new("yellow", 255, 255, 0),
    NamedColor::new("yellowgreen", 154, 205, 50),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_color() {
        let actual = NamedColor::new("orange", 255, 165, 0);
        assert_eq!(actual.name, "orange");
        assert_eq!(actual.r, 255);
        assert_eq!(actual.g, 165);
        assert_eq!(actual.b, 0);
    }
}
