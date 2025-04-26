#![feature(array_chunks, coverage_attribute)]

mod algorithm;
mod assert;
pub mod color;
mod error;
mod image;
mod math;
mod palette;
mod swatch;
mod theme;

pub use algorithm::Algorithm;
pub use error::Error;
pub use image::ImageData;
pub use math::FloatNumber;
pub use palette::Palette;
pub use swatch::Swatch;
pub use theme::Theme;
