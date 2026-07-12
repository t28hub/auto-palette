use crate::image::Rgba;

/// A predicate deciding whether a pixel participates in palette extraction.
///
/// Any closure of the form `Fn(&Rgba) -> bool + Send + Sync + 'static` is a
/// `Filter`, so most callers never implement this trait manually.
///
/// # Examples
/// ```
/// use auto_palette::{Filter, Rgba};
///
/// let opaque = |pixel: &Rgba| pixel[3] == 255;
/// assert!(opaque.test(&[255, 0, 0, 255]));
/// assert!(!opaque.test(&[255, 0, 0, 0]));
/// ```
pub trait Filter: Send + Sync + 'static {
    /// Tests whether a pixel passes the filter.
    ///
    /// # Arguments
    /// * `pixel` - The pixel to apply the filter to.
    ///
    /// # Returns
    /// `true` if the pixel passes the filter; `false` otherwise.
    #[must_use]
    fn test(&self, pixel: &Rgba) -> bool;
}

impl<F> Filter for F
where
    F: Fn(&Rgba) -> bool + Send + Sync + 'static,
{
    #[inline(always)]
    fn test(&self, pixel: &Rgba) -> bool {
        self(pixel)
    }
}

impl Filter for Box<dyn Filter> {
    #[inline(always)]
    fn test(&self, pixel: &Rgba) -> bool {
        self.as_ref().test(pixel)
    }
}

/// A filter that accepts pixels whose alpha channel exceeds a threshold.
///
/// This is the default filter used by
/// [`PaletteBuilder`](crate::PaletteBuilder): fully transparent pixels are
/// excluded from palette extraction.
///
/// # Examples
/// ```
/// use auto_palette::{AlphaFilter, Filter};
///
/// let filter = AlphaFilter::new(64);
/// assert!(filter.test(&[255, 0, 0, 65]));
/// assert!(!filter.test(&[255, 0, 0, 64]));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AlphaFilter {
    threshold: u8,
}

impl AlphaFilter {
    /// Creates a new `AlphaFilter` with the given threshold.
    ///
    /// # Arguments
    /// * `threshold` - Pixels with an alpha value greater than this threshold pass the filter.
    ///
    /// # Returns
    /// A new `AlphaFilter` instance.
    #[must_use]
    pub fn new(threshold: u8) -> Self {
        Self { threshold }
    }
}

impl Default for AlphaFilter {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Filter for AlphaFilter {
    #[inline(always)]
    fn test(&self, pixel: &Rgba) -> bool {
        pixel[3] > self.threshold
    }
}

/// A conjunction of filters: a pixel passes only if every filter accepts it.
///
/// This type is internal; [`PaletteBuilder`](crate::PaletteBuilder) uses it to
/// combine the default alpha filter with user-supplied filters.
#[derive(Default)]
pub(crate) struct FilterChain {
    filters: Vec<Box<dyn Filter>>,
}

impl FilterChain {
    /// Appends a filter to the chain.
    ///
    /// # Arguments
    /// * `filter` - The filter to append.
    pub(crate) fn push(&mut self, filter: impl Filter) {
        self.filters.push(Box::new(filter));
    }
}

impl Filter for FilterChain {
    #[inline]
    fn test(&self, pixel: &Rgba) -> bool {
        self.filters.iter().all(|filter| filter.test(pixel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_filter() {
        // Act & Assert
        let filter = |pixel: &Rgba| pixel[0] > 128; // Filter for red channel > 128
        assert!(filter.test(&[255, 0, 0, 255]));
        assert!(filter.test(&[129, 0, 0, 255]));
        assert!(!filter.test(&[128, 0, 0, 255]));
        assert!(!filter.test(&[0, 0, 0, 255]));
    }

    #[test]
    fn test_boxed_filter() {
        // Act & Assert
        let filter: Box<dyn Filter> = Box::new(|pixel: &Rgba| pixel[3] != 0);
        assert!(filter.test(&[0, 0, 0, 1]));
        assert!(!filter.test(&[0, 0, 0, 0]));
    }

    #[test]
    fn test_alpha_filter() {
        // Act & Assert
        let filter = AlphaFilter::new(127);
        assert!(filter.test(&[255, 0, 0, 255]));
        assert!(filter.test(&[255, 0, 0, 128]));
        assert!(!filter.test(&[255, 0, 0, 127]));
        assert!(!filter.test(&[255, 0, 0, 0]));
    }

    #[test]
    fn test_alpha_filter_default() {
        // Act & Assert
        let filter = AlphaFilter::default();
        assert!(filter.test(&[0, 0, 0, 1]));
        assert!(!filter.test(&[0, 0, 0, 0]));
    }

    #[test]
    fn test_filter_chain_empty_accepts_all() {
        // Act & Assert
        let chain = FilterChain::default();
        assert!(chain.test(&[0, 0, 0, 0]));
    }

    #[test]
    fn test_filter_chain_conjunction() {
        // Arrange
        let mut chain = FilterChain::default();
        chain.push(AlphaFilter::default());
        chain.push(|pixel: &Rgba| pixel[1] >= 128);

        // Act & Assert
        assert!(chain.test(&[255, 128, 255, 255]));
        assert!(!chain.test(&[255, 255, 255, 0])); // rejected by the alpha filter
        assert!(!chain.test(&[255, 127, 255, 255])); // rejected by the closure
    }
}
