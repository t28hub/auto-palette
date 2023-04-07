use crate::lab::Lab;
use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::point::Point3;
use crate::named::NamedColor;
use crate::rgba::Rgba;
use crate::white_point::D65;
use crate::xyz::XYZ;

/// Struct for searching for the closest named color to a given color.
///
/// # Examples
/// ```
/// use auto_palette::rgba::Rgba;
/// use auto_palette::named::NamedColor;
/// use auto_palette::search::ColorSearch;
///
/// let colors = [
///    NamedColor::new("red", Rgba::new(255, 0, 0, 255)),
///    NamedColor::new("green", Rgba::new(0, 255, 0, 255)),
///    NamedColor::new("blue", Rgba::new(0, 0, 255, 255)),
/// ];
/// let search = ColorSearch::new(&colors);
/// let actual = search.search(&Rgba::new(255, 0, 50, 255));
/// assert_eq!(actual, Some(NamedColor::new("red", Rgba::new(255, 0, 0, 255))));
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
                let xyz = XYZ::<f64, D65>::from(named.color());
                let lab = Lab::<f64, D65>::from(&xyz);
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
    pub fn search(&self, color: &Rgba) -> Option<NamedColor> {
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
    use crate::named::named;
    use crate::rgba::Rgba;

    #[must_use]
    fn basic_colors() -> [NamedColor; 16] {
        [
            named("black", 0, 0, 0),
            named("silver", 192, 192, 192),
            named("gray", 128, 128, 128),
            named("white", 255, 255, 255),
            named("maroon", 128, 0, 0),
            named("red", 255, 0, 0),
            named("purple", 128, 0, 128),
            named("fuchsia", 255, 0, 255),
            named("green", 0, 128, 0),
            named("lime", 0, 255, 0),
            named("olive", 128, 128, 0),
            named("yellow", 255, 255, 0),
            named("navy", 0, 0, 128),
            named("blue", 0, 0, 255),
            named("teal", 0, 128, 128),
            named("aqua", 0, 255, 255),
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

        let actual = search.search(&Rgba::new(255, 0, 50, 255));
        assert_eq!(actual, Some(named("red", 255, 0, 0)));

        let actual = search.search(&Rgba::new(255, 0, 153, 255));
        assert_eq!(actual, Some(named("purple", 128, 0, 128)));

        let actual = search.search(&Rgba::new(48, 48, 48, 255));
        assert_eq!(actual, Some(named("black", 0, 0, 0)));
    }
}
