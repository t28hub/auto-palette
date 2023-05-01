extern crate core;

mod algorithm;
mod collection;
mod color;
mod image;
mod math;
mod palette;
mod swatch;

pub use crate::image::error::*;
pub use crate::image::image_data::*;
pub use algorithm::*;
pub use color::*;
pub use palette::*;
pub use swatch::*;
