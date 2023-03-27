use thiserror::Error;

/// Enum representing
#[derive(Debug, PartialEq, Error)]
pub enum ImageError {
    #[error("The length of the image data was invalid: {0}")]
    InvalidDataSize(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_data_size() {
        let error = ImageError::InvalidDataSize(128);
        assert_eq!(
            error.to_string(),
            "The length of the image data was invalid: 128"
        );
    }
}
