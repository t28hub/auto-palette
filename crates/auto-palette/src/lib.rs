//! `auto-palette` extracts prominent color palettes from images.
//!
//! The public API is intentionally small and free of generics and lifetimes,
//! so it can be exposed as-is to WebAssembly, Android, and iOS bindings:
//!
//! * [`ImageData`] - an owned RGBA pixel buffer.
//! * [`Palette`] / [`PaletteBuilder`] - palette extraction.
//! * [`Swatch`] - a prominent color with its position and population.
//! * [`Algorithm`] / [`Theme`] - extraction and selection strategies.
//! * [`color`] - color types and color space conversions.
//!
//! # Examples
//! ```
//! #[cfg(feature = "image")]
//! {
//!     use auto_palette::{ImageData, Palette};
//!
//!     let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
//!     let palette = Palette::extract(&image_data).unwrap();
//!     for swatch in palette.find_swatches(5) {
//!         println!(
//!             "{} {} {:.2}%",
//!             swatch.color().to_hex_string(),
//!             swatch.position(),
//!             swatch.ratio() * 100.0
//!         );
//!     }
//! }
//! ```
mod algorithm;
mod assert;
pub mod color;
mod error;
mod image;
mod math;
mod palette;
mod segmentation;
mod swatch;
mod theme;

pub use algorithm::Algorithm;
pub use error::{Error, Result};
pub use image::{
    filter::{AlphaFilter, Filter},
    ImageData,
    Rgba,
};
pub use palette::{Palette, PaletteBuilder};
pub use swatch::{Position, Swatch};
pub use theme::Theme;

pub(crate) use crate::math::FloatNumber;
