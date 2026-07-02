//! 🎨 A Rust library that extracts prominent color palettes from images
//! automatically.
//!
//! # Overview
//!
//! Extraction runs as a pipeline:
//! 1. [`ImageData`] holds the RGBA pixels (loaded from a file with the
//!    `image` feature, or supplied directly).
//! 2. Large images are downsampled to the configured pixel budget
//!    (see [`PaletteBuilder::max_pixels`]).
//! 3. A segmentation algorithm ([`Algorithm`], tunable via the
//!    [`segmentation`] module) groups pixels into regions.
//! 4. Perceptually similar regions are merged into ranked [`Swatch`]es that
//!    make up the [`Palette`].
//! 5. [`Palette::find_swatches`] and [`Palette::find_swatches_with_theme`]
//!    select a diverse subset of swatches, optionally scored by a [`Theme`].
//!
//! # Quick start
//!
//! ```no_run
//! use auto_palette::{ImageData, Palette};
//!
//! # fn main() -> Result<(), auto_palette::Error> {
//! let image_data = ImageData::load("path/to/image.png")?;
//! let palette: Palette<f64> = Palette::extract(&image_data)?;
//! for swatch in palette.find_swatches(5)? {
//!     println!("{} {:?}", swatch.color().to_hex_string(), swatch.position());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! * `image` (default) - Loading images from files via the `image` crate.
//! * `serde` - `Serialize`/`Deserialize` implementations for [`Palette`],
//!   [`Swatch`], [`color::Color`], and the color space types.

mod algorithm;
mod assert;
pub mod color;
mod error;
mod image;
mod math;
mod palette;
pub mod segmentation;
mod swatch;
mod theme;

pub use algorithm::Algorithm;
pub use error::{
    Error,
    ExtractionError,
    ExtractionErrorKind,
    ImageError,
    SelectionError,
    SelectionErrorKind,
    UnsupportedError,
    UnsupportedErrorKind,
};
pub use image::{filter::Filter, ImageData, Rgba};
pub use math::FloatNumber;
pub use palette::{Palette, PaletteBuilder};
pub use segmentation::SegmentationMethod;
pub use swatch::Swatch;
pub use theme::Theme;
