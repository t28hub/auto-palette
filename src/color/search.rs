use crate::color_trait::Color;
use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point3;
use crate::named::NamedColor;
use crate::white_point::D65;

/// Struct for searching for the closest named color to a given color.
///
/// # Type Parameters
/// * `F` - The floating point type.
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
pub struct ColorSearch<'a, F: Float> {
    colors: &'a [NamedColor],
    kdtree: KDTreeSearch<'a, F, Point3<F>>,
}

impl<'a, F> ColorSearch<'a, F>
where
    F: Float,
{
    /// Create a new `ColorSearch` instance.
    ///
    /// # Arguments
    /// * `colors` - A slice of named colors to search.
    ///
    /// # Returns
    /// A new `ColorSearch` instance.
    #[must_use]
    pub fn new(colors: &'a [NamedColor]) -> Self {
        let points: Vec<Point3<F>> = colors
            .iter()
            .map(|named| {
                let lab = named.to_lab();
                Point3::new(F::from_f64(lab.l), F::from_f64(lab.a), F::from_f64(lab.b))
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
    pub fn search<C>(&self, color: &C) -> Option<NamedColor>
    where
        C: Color<F = F, WP = D65>,
    {
        let lab = color.to_lab();
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
    use crate::named::BASIC_COLORS;
    use crate::rgb::Rgb;

    #[test]
    fn test_color_search() {
        let actual: ColorSearch<f64> = ColorSearch::new(&BASIC_COLORS);
        assert_eq!(actual.colors.len(), 16);
    }

    #[test]
    fn test_search() {
        let color_search: ColorSearch<f64> = ColorSearch::new(&BASIC_COLORS);

        let actual = color_search.search(&Rgb::new(255, 0, 50));
        assert_eq!(actual, Some(NamedColor::new("red", 255, 0, 0)));

        let actual = color_search.search(&Rgb::new(255, 0, 153));
        assert_eq!(actual, Some(NamedColor::new("purple", 128, 0, 128)));

        let actual = color_search.search(&Rgb::new(48, 48, 48));
        assert_eq!(actual, Some(NamedColor::new("black", 0, 0, 0)));
    }
}
