use std::{error::Error as StdError, fmt};

use crate::math::sampling::SamplingError;

/// The error type for palette operations.
///
/// This enum provides a high-level classification of errors by the operation
/// that failed. The [`Extraction`](Error::Extraction), [`Selection`](Error::Selection),
/// and [`Unsupported`](Error::Unsupported) variants wrap opaque error structs
/// whose [`kind()`] method provides a coarse classification.
/// The [`Image`](Error::Image) variant wraps an [`ImageError`] enum directly.
///
/// Use [`Display`](fmt::Display) for user-facing messages.
/// [`source()`](StdError::source) is available when an underlying cause exists
/// (e.g., [`ImageError::LoadError`]).
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred while loading or processing image data.
    ///
    /// This includes failures such as mismatched pixel data length,
    /// unsupported image formats, or I/O errors during loading.
    Image(ImageError),

    /// An error occurred during palette extraction.
    ///
    /// This includes failures caused by invalid configuration parameters
    /// or mismatched image dimensions.
    Extraction(ExtractionError),

    /// An error occurred during swatch selection.
    ///
    /// This includes failures caused by empty input data or
    /// invalid sampling parameters.
    Selection(SelectionError),

    /// An unsupported value was specified.
    ///
    /// The requested algorithm or theme name is not recognized.
    Unsupported(UnsupportedError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Image(err) => err.fmt(f),
            Error::Extraction(err) => err.fmt(f),
            Error::Selection(err) => err.fmt(f),
            Error::Unsupported(err) => err.fmt(f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Image(err) => err.source(),
            Error::Extraction(err) => err.source(),
            Error::Selection(err) => err.source(),
            Error::Unsupported(err) => err.source(),
        }
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Error::Image(err)
    }
}

impl From<ExtractionError> for Error {
    fn from(err: ExtractionError) -> Self {
        Error::Extraction(err)
    }
}

impl From<SelectionError> for Error {
    fn from(err: SelectionError) -> Self {
        Error::Selection(err)
    }
}

impl From<UnsupportedError> for Error {
    fn from(err: UnsupportedError) -> Self {
        Error::Unsupported(err)
    }
}

/// An error encountered during palette extraction.
///
/// This error is returned when the extraction process fails due to
/// invalid configuration parameters or unexpected input data.
///
/// Use [`kind()`](ExtractionError::kind) for a rough classification
/// or [`Display`](fmt::Display) for a human-readable description.
#[derive(Debug)]
pub struct ExtractionError {
    kind: ExtractionErrorKind,
}

/// Describes what went wrong during palette extraction.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ExtractionErrorKind {
    /// A configuration parameter is invalid (e.g., segments = 0).
    InvalidParameter,
    /// The image dimensions do not match the pixel data.
    DimensionMismatch,
    /// An unexpected internal error occurred.
    Internal,
}

impl ExtractionError {
    /// Returns the corresponding `ExtractionErrorKind`.
    ///
    /// # Returns
    /// The kind of error that occurred during palette extraction.
    #[must_use]
    pub fn kind(&self) -> ExtractionErrorKind {
        self.kind.clone()
    }
}

impl From<ExtractionErrorKind> for ExtractionError {
    fn from(kind: ExtractionErrorKind) -> Self {
        Self { kind }
    }
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ExtractionErrorKind::InvalidParameter => {
                write!(f, "invalid parameter for palette extraction")
            }
            ExtractionErrorKind::DimensionMismatch => {
                write!(f, "image dimensions do not match pixel data")
            }
            ExtractionErrorKind::Internal => {
                write!(f, "internal error during extraction")
            }
        }
    }
}

impl StdError for ExtractionError {}

/// An error encountered during swatch selection.
///
/// This error is returned when the swatch selection process fails due to
/// empty input data or invalid sampling parameters.
///
/// Use [`kind()`](SelectionError::kind) for a rough classification
/// or [`Display`](fmt::Display) for a human-readable description.
#[derive(Debug)]
pub struct SelectionError {
    kind: SelectionErrorKind,
}

/// Describes what went wrong during swatch selection.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum SelectionErrorKind {
    /// The input data is empty.
    EmptyInput,
    /// A parameter value is invalid (e.g., diversity factor out of range).
    InvalidParameter,
    /// The lengths of input arrays do not match.
    LengthMismatch,
}

impl SelectionError {
    /// Returns the corresponding `SelectionErrorKind`.
    ///
    /// # Returns
    /// A `SelectionErrorKind` that classifies the type of selection error.
    #[must_use]
    pub fn kind(&self) -> SelectionErrorKind {
        self.kind.clone()
    }
}

impl From<SelectionErrorKind> for SelectionError {
    fn from(kind: SelectionErrorKind) -> Self {
        Self { kind }
    }
}

impl From<SamplingError> for SelectionError {
    fn from(err: SamplingError) -> Self {
        let kind = match err {
            SamplingError::EmptyPoints | SamplingError::EmptyWeights => {
                SelectionErrorKind::EmptyInput
            }
            SamplingError::DiversityOutOfRange => SelectionErrorKind::InvalidParameter,
            SamplingError::LengthMismatch { .. } => SelectionErrorKind::LengthMismatch,
        };
        Self { kind }
    }
}

impl fmt::Display for SelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            SelectionErrorKind::EmptyInput => {
                write!(f, "cannot select swatches from empty input")
            }
            SelectionErrorKind::InvalidParameter => {
                write!(f, "invalid parameter for swatch selection")
            }
            SelectionErrorKind::LengthMismatch => {
                write!(f, "input array lengths do not match")
            }
        }
    }
}

impl StdError for SelectionError {}

/// An unsupported algorithm or theme was specified.
///
/// Use [`kind()`](UnsupportedError::kind) to determine whether the
/// error is about an algorithm or a theme, and [`value()`](UnsupportedError::value)
/// to get the unsupported value that was specified.
#[derive(Debug)]
pub struct UnsupportedError {
    kind: UnsupportedErrorKind,
    value: String,
}

/// Describes what kind of value was unsupported.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum UnsupportedErrorKind {
    /// The algorithm name is not recognized.
    Algorithm,
    /// The theme name is not recognized.
    Theme,
}

impl UnsupportedError {
    /// Creates a new `UnsupportedError`.
    ///
    /// # Arguments
    /// * `kind` - The kind of unsupported value (algorithm or theme).
    /// * `value` - The unsupported value that was specified.
    ///
    /// # Returns
    /// A new `UnsupportedError` with the specified kind and value.
    #[must_use]
    pub(crate) fn new(kind: UnsupportedErrorKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    /// Returns the corresponding `UnsupportedErrorKind`.
    ///
    /// # Returns
    /// The kind of unsupported value that was specified (algorithm or theme).
    #[must_use]
    pub fn kind(&self) -> UnsupportedErrorKind {
        self.kind.clone()
    }

    /// Returns the unsupported value that was specified.
    ///
    /// # Returns
    /// The unsupported value (e.g., the unrecognized algorithm or theme name).
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            UnsupportedErrorKind::Algorithm => {
                write!(f, "unsupported algorithm: '{}'", self.value)
            }
            UnsupportedErrorKind::Theme => {
                write!(f, "unsupported theme: '{}'", self.value)
            }
        }
    }
}

impl StdError for UnsupportedError {}

/// Represents specific errors encountered during image processing.
///
/// This includes failures such as mismatched pixel data length,
/// unsupported image formats, or I/O errors during loading.
#[derive(Debug)]
#[non_exhaustive]
pub enum ImageError {
    /// The length of the image data is unexpected.
    UnexpectedLength {
        /// The expected length.
        expected: usize,
        /// The actual length.
        actual: usize,
    },

    /// The image failed to load from a file.
    #[cfg(feature = "image")]
    LoadError {
        /// The underlying cause.
        source: image::ImageError,
    },

    /// The image format or color type is not supported.
    #[cfg(feature = "image")]
    UnsupportedFormat,
}

#[cfg(feature = "image")]
impl From<image::ImageError> for ImageError {
    fn from(source: image::ImageError) -> Self {
        ImageError::LoadError { source }
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::UnexpectedLength { expected, actual } => {
                write!(
                    f,
                    "unexpected data length: expected {expected}, got {actual}"
                )
            }
            #[cfg(feature = "image")]
            ImageError::LoadError { source } => {
                write!(f, "failed to load image: {source}")
            }
            #[cfg(feature = "image")]
            ImageError::UnsupportedFormat => {
                write!(f, "unsupported image format or color type")
            }
        }
    }
}

impl StdError for ImageError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            #[cfg(feature = "image")]
            ImageError::LoadError { source } => Some(source),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_satisfies_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }

    #[test]
    fn test_extraction_error_invalid_parameter() {
        // Act
        let actual = ExtractionError::from(ExtractionErrorKind::InvalidParameter);

        // Assert
        assert_eq!(
            actual.to_string(),
            "invalid parameter for palette extraction"
        );
        assert_eq!(actual.kind(), ExtractionErrorKind::InvalidParameter);
    }

    #[test]
    fn test_extraction_error_dimension_mismatch() {
        // Act
        let actual = ExtractionError::from(ExtractionErrorKind::DimensionMismatch);

        // Assert
        assert_eq!(
            actual.to_string(),
            "image dimensions do not match pixel data"
        );
        assert_eq!(actual.kind(), ExtractionErrorKind::DimensionMismatch);
    }

    #[test]
    fn test_extraction_error_internal() {
        // Act
        let actual = ExtractionError::from(ExtractionErrorKind::Internal);

        // Assert
        assert_eq!(actual.to_string(), "internal error during extraction");
        assert_eq!(actual.kind(), ExtractionErrorKind::Internal);
    }

    #[test]
    fn test_selection_error_empty_input() {
        // Act
        let actual = SelectionError::from(SelectionErrorKind::EmptyInput);

        // Assert
        assert_eq!(
            actual.to_string(),
            "cannot select swatches from empty input"
        );
        assert_eq!(actual.kind(), SelectionErrorKind::EmptyInput);
    }

    #[test]
    fn test_selection_error_invalid_parameter() {
        // Act
        let actual = SelectionError::from(SelectionErrorKind::InvalidParameter);

        // Assert
        assert_eq!(actual.to_string(), "invalid parameter for swatch selection");
        assert_eq!(actual.kind(), SelectionErrorKind::InvalidParameter);
    }

    #[test]
    fn test_selection_error_length_mismatch() {
        // Act
        let actual = SelectionError::from(SelectionErrorKind::LengthMismatch);

        // Assert
        assert_eq!(actual.to_string(), "input array lengths do not match");
        assert_eq!(actual.kind(), SelectionErrorKind::LengthMismatch);
    }

    #[test]
    fn test_selection_error_from_sampling_error_empty_points() {
        // Act
        let actual = SelectionError::from(SamplingError::EmptyPoints);

        // Assert
        assert_eq!(actual.kind(), SelectionErrorKind::EmptyInput);
    }

    #[test]
    fn test_selection_error_from_sampling_error_empty_weights() {
        // Act
        let actual = SelectionError::from(SamplingError::EmptyWeights);

        // Assert
        assert_eq!(actual.kind(), SelectionErrorKind::EmptyInput);
    }

    #[test]
    fn test_selection_error_from_sampling_error_diversity_out_of_range() {
        // Act
        let actual = SelectionError::from(SamplingError::DiversityOutOfRange);

        // Assert
        assert_eq!(actual.kind(), SelectionErrorKind::InvalidParameter);
    }

    #[test]
    fn test_selection_error_from_sampling_error_length_mismatch() {
        // Act
        let actual = SelectionError::from(SamplingError::LengthMismatch {
            points_len: 10,
            weights_len: 5,
        });

        // Assert
        assert_eq!(actual.kind(), SelectionErrorKind::LengthMismatch);
    }

    #[test]
    fn test_unsupported_algorithm() {
        // Act
        let actual = UnsupportedError::new(UnsupportedErrorKind::Algorithm, "xmeans");

        // Assert
        assert_eq!(actual.to_string(), "unsupported algorithm: 'xmeans'");
        assert_eq!(actual.kind(), UnsupportedErrorKind::Algorithm);
        assert_eq!(actual.value(), "xmeans");
    }

    #[test]
    fn test_unsupported_theme() {
        // Act
        let actual = UnsupportedError::new(UnsupportedErrorKind::Theme, "pastel");

        // Assert
        assert_eq!(actual.to_string(), "unsupported theme: 'pastel'");
        assert_eq!(actual.kind(), UnsupportedErrorKind::Theme);
        assert_eq!(actual.value(), "pastel");
    }

    #[test]
    fn test_error_from_image_error() {
        // Act
        let image_error = ImageError::UnexpectedLength {
            expected: 100,
            actual: 50,
        };
        let actual = Error::from(image_error);

        // Assert
        assert!(matches!(actual, Error::Image(_)));
    }

    #[test]
    fn test_error_from_extraction_error() {
        // Act
        let extraction_error = ExtractionError::from(ExtractionErrorKind::InvalidParameter);
        let actual = Error::from(extraction_error);

        // Assert
        assert!(matches!(actual, Error::Extraction(_)));
    }

    #[test]
    fn test_error_from_selection_error() {
        // Act
        let selection_error = SelectionError::from(SelectionErrorKind::EmptyInput);
        let actual = Error::from(selection_error);

        // Assert
        assert!(matches!(actual, Error::Selection(_)));
    }

    #[test]
    fn test_error_from_unsupported_error() {
        // Act
        let unsupported_error = UnsupportedError::new(UnsupportedErrorKind::Algorithm, "xmeans");
        let actual = Error::from(unsupported_error);

        // Assert
        assert!(matches!(actual, Error::Unsupported(_)));
    }

    #[test]
    fn test_error_display_delegates_extraction() {
        // Act
        let extraction_error = ExtractionError::from(ExtractionErrorKind::InvalidParameter);
        let actual: Error = extraction_error.into();

        // Assert
        assert_eq!(
            actual.to_string(),
            "invalid parameter for palette extraction"
        );
    }

    #[test]
    fn test_error_display_delegates_unsupported() {
        // Act
        let unsupported_error = UnsupportedError::new(UnsupportedErrorKind::Theme, "pastel");
        let actual: Error = unsupported_error.into();

        // Assert
        assert_eq!(actual.to_string(), "unsupported theme: 'pastel'");
    }
}
