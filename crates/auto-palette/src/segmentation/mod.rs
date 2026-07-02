//! Fine-grained configuration of the image segmentation algorithms.
//!
//! Every algorithm listed in [`Algorithm`](crate::Algorithm) has a
//! configuration type in this module that can be passed to
//! [`PaletteBuilder::algorithm`](crate::PaletteBuilder::algorithm)
//! to tune its parameters:
//!
//! ```
//! use auto_palette::{segmentation::DbscanConfig, Palette};
//! # use auto_palette::ImageData;
//!
//! # fn main() -> Result<(), auto_palette::Error> {
//! # let pixels = [255u8; 4 * 4];
//! # let image_data = ImageData::new(2, 2, &pixels)?;
//! let config = DbscanConfig::default().segments(64).min_pixels(4);
//! let palette: Palette<f64> = Palette::builder().algorithm(config).build(&image_data)?;
//! # Ok(())
//! # }
//! ```

mod algorithm;
mod dbscan;
mod error;
mod fastdbscan;
mod helper;
mod input;
mod kmeans;
mod method;
mod result;
mod seed;
pub(crate) mod segment;
mod slic;
mod snic;

pub(crate) use algorithm::Segmentation;
pub use dbscan::DbscanConfig;
pub(crate) use dbscan::DbscanSegmentation;
pub use fastdbscan::FastDbscanConfig;
pub(crate) use fastdbscan::FastDbscanSegmentation;
pub use kmeans::KmeansConfig;
pub(crate) use kmeans::KmeansSegmentation;
pub use method::SegmentationMethod;
pub(crate) use result::SegmentationResult;
pub use slic::SlicConfig;
pub(crate) use slic::SlicSegmentation;
pub use snic::SnicConfig;
pub(crate) use snic::SnicSegmentation;
