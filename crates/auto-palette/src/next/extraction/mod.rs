mod centroid;
mod clustering;
mod extractor;
mod nearest;
mod point;
mod position;
mod preprocessor;
mod resampling;
mod saliency;
mod segmentation;
#[cfg(test)]
mod test_helpers;

pub use extractor::Extractor;

/// The result of extracting colors from an image with an [`Extractor`].
#[derive(Debug, Clone)]
pub struct Extraction {
    // Private so that the representation can change without breaking users.
    _private: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_is_send_sync() {
        // Assert: users can share the extraction across threads.
        fn assert_bounds<T: Send + Sync>() {}
        assert_bounds::<Extraction>();
    }
}
