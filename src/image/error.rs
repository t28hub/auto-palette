use image::error::UnsupportedError;
use std::io;

/// ImageError represents an error that occurs while processing an image.
#[derive(Debug)]
pub enum ImageError {
    /// An error was encountered when the parameter is invalid.
    InvalidParameter,
    /// An error was encountered when the file format of the image is not supported.
    UnsupportedFile(UnsupportedError),
    /// An error was encountered when the color type of the image is not supported.
    UnsupportedType(image::ColorType),
    /// An error was encountered when an I/O error occurred.
    IoError(io::Error),
    /// An error was encountered when processing the image due to an unknown error.
    Unknown(image::ImageError),
}
