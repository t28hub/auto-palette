//! Work-in-progress API for auto-palette 1.0.0.
//!
//! Unstable: not covered by semver, and may change or be removed in any release.

mod color;
mod error;
mod extraction;
mod image;
mod math;

pub use error::{Error, ErrorKind};
pub use extraction::{Extraction, Extractor};
pub use image::{ImageRef, PixelFormat};
