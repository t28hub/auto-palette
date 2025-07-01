mod algorithm;
mod assert;
pub mod color;
mod error;
mod image;
mod lut;
mod math;
mod palette;
mod swatch;
mod theme;

pub use algorithm::Algorithm;
pub use error::Error;
pub use image::{filter::Filter, ImageData, ImageError, ImageResult, Rgba};
pub use math::FloatNumber;
use once_cell::sync::Lazy;
pub use palette::Palette;
pub use swatch::Swatch;
pub use theme::Theme;

use crate::{color::Gamut, lut::ChromaLookupTable};

static SRGB_GAMUT: Lazy<Gamut> = Lazy::new(|| {
    const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/srgb_table.bin"));
    let table = ChromaLookupTable::from_bytes(BYTES).expect("SRGB lookup table corrupted");
    Gamut::new(table.kind(), table.values::<f32>())
});
