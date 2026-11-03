//! Work-in-progress API for auto-palette 1.0.0.
//!
//! Unstable: not covered by semver, and may change or be removed in any release.

mod error;
mod image;

pub use error::{Error, ErrorKind};
pub use image::{ImageRef, PixelFormat};
