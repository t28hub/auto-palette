use std::fmt::{Display, Formatter};
use std::io;

use image::error::UnsupportedError;

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

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ImageError::InvalidParameter => write!(f, "Invalid parameter"),
            ImageError::UnsupportedFile(e) => write!(f, "Unsupported file: {}", e),
            ImageError::UnsupportedType(t) => write!(f, "Unsupported type: {:?}", t),
            ImageError::IoError(e) => write!(f, "I/O error: {}", e),
            ImageError::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use image::error::{DecodingError, ImageFormatHint};
    use image::ImageError::Decoding;

    use super::*;

    #[test]
    fn test_fmt_invalid_parameter() {
        let error = ImageError::InvalidParameter;
        assert_eq!(error.to_string(), "Invalid parameter");
    }

    #[test]
    fn test_fmt_unsupported_file() {
        let error = ImageError::UnsupportedFile(UnsupportedError::from(ImageFormatHint::Unknown));
        assert_eq!(
            error.to_string(),
            "Unsupported file: The image format could not be determined"
        );
    }

    #[test]
    fn test_fmt_unsupported_type() {
        let error = ImageError::UnsupportedType(image::ColorType::L8);
        assert_eq!(error.to_string(), "Unsupported type: L8");
    }

    #[test]
    fn test_fmt_io_error() {
        let error = ImageError::IoError(io::Error::new(io::ErrorKind::Other, "I/O error"));
        assert_eq!(error.to_string(), "I/O error: I/O error");
    }

    #[test]
    fn test_fmt_unknown() {
        let error = ImageError::Unknown(Decoding(DecodingError::from_format_hint(
            ImageFormatHint::Unknown,
        )));
        assert_eq!(error.to_string(), "Unknown error: Format error");
    }
}
