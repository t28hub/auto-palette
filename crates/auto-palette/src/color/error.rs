use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ColorError {
    #[error("Invalid color code: {0}")]
    InvalidColorCode(u8),

    #[error("Invalid hex color value: {0}")]
    InvalidHexValue(String),
}
