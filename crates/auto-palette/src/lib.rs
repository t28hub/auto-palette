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
pub use palette::Palette;
pub use swatch::Swatch;
pub use theme::Theme;
