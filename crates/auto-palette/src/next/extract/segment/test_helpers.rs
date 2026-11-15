use super::{point::Point, position::Position};
use crate::next::{
    color::{AlphaColor, Oklab},
    image::{ImageBuf, Size},
};

/// Black in OKLab.
pub(super) const BLACK: Oklab = Oklab::new(0.0, 0.0, 0.0);

/// White in OKLab.
pub(super) const WHITE: Oklab = Oklab::new(1.0, 0.0, 0.0);

/// Creates an image from rows of colors, with `None` for a transparent pixel.
///
/// # Arguments
/// * `rows` - The colors of the pixels, row by row.
///
/// # Returns
/// The image with opaque pixels of the given colors.
pub(super) fn image<const W: usize>(rows: &[[Option<Oklab>; W]]) -> ImageBuf<AlphaColor<Oklab>> {
    let pixels = rows
        .iter()
        .flatten()
        .map(|color| match color {
            Some(color) => AlphaColor::opaque(*color),
            None => AlphaColor::transparent(BLACK),
        })
        .collect();
    ImageBuf::new(Size::new(W as u32, rows.len() as u32), pixels)
}

/// Creates a gradient from black to white.
///
/// # Returns
/// `N` colors of evenly spaced lightness, from black to white.
pub(super) fn gradient<const N: usize>() -> [Oklab; N] {
    std::array::from_fn(|index| Oklab::new(index as f64 / (N - 1) as f64, 0.0, 0.0))
}

/// Creates a point of a color at the origin with weight 1.0.
///
/// # Arguments
/// * `color` - The color of the point.
///
/// # Returns
/// The point of `color` at the origin, counting fully in a mean.
pub(super) fn point(color: Oklab) -> Point {
    Point::new(color, Position::new(0.0, 0.0), 1.0)
}

/// Creates a point of a color at the origin with weight 0.0.
///
/// # Arguments
/// * `color` - The color of the point.
///
/// # Returns
/// The point of `color` at the origin, counting for nothing in a mean.
pub(super) fn weightless(color: Oklab) -> Point {
    Point::new(color, Position::new(0.0, 0.0), 0.0)
}

/// Creates a gray point at the origin with weight 1.0.
///
/// # Arguments
/// * `l` - The lightness of the gray, from 0.0 to 1.0.
///
/// # Returns
/// The point of the gray at the origin, counting fully in a mean.
pub(super) fn gray(l: f64) -> Point {
    point(Oklab::new(l, 0.0, 0.0))
}
