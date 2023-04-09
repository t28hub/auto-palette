use crate::color_trait::Color;
use crate::lab::Lab;
use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::point::Point3;
use crate::named::NamedColor;
use crate::rgb::Rgb;
use crate::white_point::D65;
use crate::xyz::XYZ;

/// Struct for searching for the closest named color to a given color.
///
/// # Examples
/// ```
/// use auto_palette::rgb::Rgb;
/// use auto_palette::named::NamedColor;
/// use auto_palette::search::ColorSearch;
///
/// let colors = [
///     NamedColor::new("red", 255, 0, 0),
///     NamedColor::new("green", 0, 255, 0),
///     NamedColor::new("blue", 0, 0, 255),
///     NamedColor::new("yellow", 255, 255, 0),
///     NamedColor::new("cyan", 0, 255, 255),
///     NamedColor::new("magenta", 255, 0, 255),
///     NamedColor::new("white", 255, 255, 255),
///     NamedColor::new("black", 0, 0, 0),
/// ];
/// let search = ColorSearch::new(&colors);
/// let actual = search.search(&Rgb::new(255, 0, 50));
/// assert_eq!(actual, Some(NamedColor::new("red", 255, 0, 0)));
/// ```
pub struct ColorSearch<'a> {
    colors: &'a [NamedColor],
    kdtree: KDTreeSearch<'a, f64, Point3<f64>>,
}

impl<'a> ColorSearch<'a> {
    /// Create a new `ColorSearch` instance.
    ///
    /// # Arguments
    /// * `colors` - A slice of named colors to search.
    ///
    /// # Returns
    /// A new `ColorSearch` instance.
    #[must_use]
    pub fn new(colors: &'a [NamedColor]) -> Self {
        let points: Vec<Point3<f64>> = colors
            .iter()
            .map(|named| {
                let lab = named.to_lab();
                Point3::new(lab.l, lab.a, lab.b)
            })
            .collect();
        let kdtree = KDTreeSearch::new_with_vec(points, Distance::SquaredEuclidean);
        Self { colors, kdtree }
    }

    /// Search for the closest named color to the given color.
    ///
    /// # Arguments
    /// * `color` - The color to search for.
    ///
    /// # Returns
    /// The closest named color to the given color, or `None` if no colors were given.
    #[must_use]
    pub fn search(&self, color: &Rgb) -> Option<NamedColor> {
        let xyz = XYZ::<f64, D65>::from(color);
        let lab = Lab::<f64, D65>::from(&xyz);
        let point = Point3::new(lab.l, lab.a, lab.b);
        self.kdtree.search_nearest(&point).map(|neighbor| {
            let named = &self.colors[neighbor.index];
            named.clone()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rgb::Rgb;

    #[must_use]
    fn basic_colors() -> [NamedColor; 16] {
        [
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
        ]
    }

    #[test]
    fn test_color_search() {
        let colors = basic_colors();
        let actual = ColorSearch::new(&colors);
        assert_eq!(actual.colors.len(), 16);
    }

    #[test]
    fn test_search() {
        let colors = basic_colors();
        let search = ColorSearch::new(&colors);

        let actual = search.search(&Rgb::new(255, 0, 50));
        assert_eq!(actual, Some(NamedColor::new("red", 255, 0, 0)));

        let actual = search.search(&Rgb::new(255, 0, 153));
        assert_eq!(actual, Some(NamedColor::new("purple", 128, 0, 128)));

        let actual = search.search(&Rgb::new(48, 48, 48));
        assert_eq!(actual, Some(NamedColor::new("black", 0, 0, 0)));
    }
}
