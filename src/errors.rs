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
