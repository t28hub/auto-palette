use std::fmt::{Display, Formatter};

/// Errors that can occur when working with palettes.
#[derive(Debug, Clone, PartialEq)]
pub enum PaletteError {
    /// The image data is empty.
    EmptyImageData,
    /// Palette extraction error.
    ExtractionError(String),
    /// Invalid algorithm error.
    InvalidAlgorithm(String),
    /// Invalid theme error.
    InvalidTheme(String),
}

impl Display for PaletteError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            PaletteError::EmptyImageData => write!(f, "The image data is empty."),
            PaletteError::ExtractionError(s) => write!(f, "Palette extraction error: {}.", s),
            PaletteError::InvalidAlgorithm(s) => write!(f, "Invalid algorithm error: {}.", s),
            PaletteError::InvalidTheme(s) => write!(f, "Invalid theme error: {}.", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_empty_image_data() {
        let error = PaletteError::EmptyImageData;
        assert_eq!(error.to_string(), "The image data is empty.");
    }

    #[test]
    fn test_fmt_extraction_error() {
        let error = PaletteError::ExtractionError("Failed to extract palette".to_string());
        assert_eq!(
            error.to_string(),
            "Palette extraction error: Failed to extract palette."
        );
    }

    #[test]
    fn test_fmt_invalid_algorithm() {
        let error = PaletteError::InvalidAlgorithm("Invalid algorithm".to_string());
        assert_eq!(
            error.to_string(),
            "Invalid algorithm error: Invalid algorithm."
        );
    }

    #[test]
    fn test_fmt_invalid_theme() {
        let error = PaletteError::InvalidTheme("Invalid theme".to_string());
        assert_eq!(error.to_string(), "Invalid theme error: Invalid theme.");
    }
}
