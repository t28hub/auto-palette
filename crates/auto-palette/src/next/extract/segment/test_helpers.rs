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
