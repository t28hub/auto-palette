use std::{convert::Infallible, fmt};

use super::PixelFormat;

/// The error type for the auto-palette 1.0.0 API.
///
/// Use [`Error::kind`] to tell errors apart. The message describes the details.
#[derive(Debug)]
pub struct Error {
    detail: ErrorDetail,
}

/// The category of an [`Error`].
///
/// Check the kind, not the message, to handle an error: the message may change in any release.
/// New kinds may be added, so match with a `_` arm.
///
/// # Examples
/// ```
/// use auto_palette::next::{ErrorKind, ImageRef};
///
/// let pixels = [0; 3];
/// let error = ImageRef::from_rgba8(&pixels, 1, 1).unwrap_err();
/// match error.kind() {
///     ErrorKind::InvalidImage => eprintln!("invalid image: {error}"),
///     _ => eprintln!("error: {error}"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The image data does not match the given size, pixel format, or row stride, or the image
    /// is too large for this platform.
    InvalidImage,
    /// The image uses a pixel format that is not supported, such as 16-bit or floating-point
    /// channels.
    UnsupportedPixelFormat,
}

impl Error {
    /// Returns the category of this error.
    ///
    /// # Returns
    /// The category of this error.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self.detail {
            ErrorDetail::DataLengthMismatch { .. }
            | ErrorDetail::DataTooShort { .. }
            | ErrorDetail::RowStrideTooShort { .. }
            | ErrorDetail::ImageTooLarge { .. } => ErrorKind::InvalidImage,
            #[cfg(feature = "image-0_25")]
            ErrorDetail::UnsupportedColorType { .. } => ErrorKind::UnsupportedPixelFormat,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl std::error::Error for Error {}

impl From<ErrorDetail> for Error {
    fn from(detail: ErrorDetail) -> Self {
        Self { detail }
    }
}

/// Allows inputs that always convert to an `ImageRef`, such as `ImageRef` itself.
impl From<Infallible> for Error {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}

/// The details of an [`Error`], kept private so that they can change without breaking users.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ErrorDetail {
    /// The pixel data is not exactly as long as the image requires.
    #[error(
        "expected {expected} bytes of pixel data for a {width}x{height} {format:?} image, found \
         {actual}"
    )]
    DataLengthMismatch {
        width: u32,
        height: u32,
        format: PixelFormat,
        expected: usize,
        actual: usize,
    },

    /// The pixel data is shorter than the image requires.
    #[error(
        "expected at least {required} bytes of pixel data for a {width}x{height} {format:?} \
         image, found {actual}"
    )]
    DataTooShort {
        width: u32,
        height: u32,
        format: PixelFormat,
        required: usize,
        actual: usize,
    },

    /// The row stride is shorter than one row of pixels.
    #[error("bytes per row ({bytes_per_row}) is less than one row of pixels ({min} bytes)")]
    RowStrideTooShort { bytes_per_row: usize, min: usize },

    /// The image size overflows `usize` on this platform.
    #[error("a {width}x{height} {format:?} image is too large for this platform")]
    ImageTooLarge {
        width: u32,
        height: u32,
        format: PixelFormat,
    },

    /// The `image` crate image has a color type with no matching [`PixelFormat`].
    #[cfg(feature = "image-0_25")]
    #[error("unsupported pixel format {color}, convert the image with to_rgba8() first")]
    // A name, not `image::ColorType`, so every `image-*` feature can share this variant.
    UnsupportedColorType { color: String },
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::data_length_mismatch(
        ErrorDetail::DataLengthMismatch {
            width: 2,
            height: 2,
            format: PixelFormat::Rgba8,
            expected: 16,
            actual: 15,
        },
        ErrorKind::InvalidImage
    )]
    #[case::data_too_short(
        ErrorDetail::DataTooShort {
            width: 2,
            height: 3,
            format: PixelFormat::Rgb8,
            required: 22,
            actual: 21,
        },
        ErrorKind::InvalidImage
    )]
    #[case::row_stride_too_short(
        ErrorDetail::RowStrideTooShort { bytes_per_row: 5, min: 6 },
        ErrorKind::InvalidImage
    )]
    #[case::image_too_large(
        ErrorDetail::ImageTooLarge {
            width: 1,
            height: 3,
            format: PixelFormat::Rgb8,
        },
        ErrorKind::InvalidImage
    )]
    #[cfg_attr(
        feature = "image-0_25",
        case::unsupported_color_type(
            ErrorDetail::UnsupportedColorType { color: "Rgba16".to_string() },
            ErrorKind::UnsupportedPixelFormat
        )
    )]
    fn test_kind(#[case] detail: ErrorDetail, #[case] expected: ErrorKind) {
        // Arrange
        let error = Error::from(detail);

        // Act
        let actual = error.kind();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::data_length_mismatch(
        ErrorDetail::DataLengthMismatch {
            width: 2,
            height: 2,
            format: PixelFormat::Rgba8,
            expected: 16,
            actual: 15,
        },
        "expected 16 bytes of pixel data for a 2x2 Rgba8 image, found 15"
    )]
    #[case::data_too_short(
        ErrorDetail::DataTooShort {
            width: 2,
            height: 3,
            format: PixelFormat::Rgb8,
            required: 22,
            actual: 21,
        },
        "expected at least 22 bytes of pixel data for a 2x3 Rgb8 image, found 21"
    )]
    #[case::row_stride_too_short(
        ErrorDetail::RowStrideTooShort { bytes_per_row: 5, min: 6 },
        "bytes per row (5) is less than one row of pixels (6 bytes)"
    )]
    #[case::image_too_large(
        ErrorDetail::ImageTooLarge {
            width: 1,
            height: 3,
            format: PixelFormat::Rgb8,
        },
        "a 1x3 Rgb8 image is too large for this platform"
    )]
    #[cfg_attr(
        feature = "image-0_25",
        case::unsupported_color_type(
            ErrorDetail::UnsupportedColorType { color: "Rgba16".to_string() },
            "unsupported pixel format Rgba16, convert the image with to_rgba8() first"
        )
    )]
    fn test_display(#[case] detail: ErrorDetail, #[case] expected: &str) {
        // Arrange
        let error = Error::from(detail);

        // Act
        let actual = error.to_string();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_error_is_send_sync_static() {
        // Assert: users can box and send the error, e.g. into `anyhow::Error`.
        fn assert_bounds<T: std::error::Error + Send + Sync + 'static>() {}
        assert_bounds::<Error>();
    }
}
