use crate::image::RgbaPixel;

/// A trait for filtering pixels in an image.
pub trait Filter: Send + Sync + 'static {
    /// Tests whether a pixel passes the filter.
    ///
    /// # Arguments
    /// * `pixel` - The pixel to apply the filter to.
    ///
    /// # Returns
    /// `true` if the pixel passes the filter; `false` otherwise.
    #[must_use]
    fn test(&self, pixel: &RgbaPixel) -> bool;

    /// Composites this filter with another filter.
    ///
    /// # Type Parameters
    /// * `F` - The type of the filter to compose with.
    ///
    /// # Arguments
    /// * `other` - The other filter to compose with.
    ///
    /// # Returns
    /// A new filter that applies this filter and then the given filter.
    #[must_use]
    fn composite<F>(self, other: F) -> CompositeFilter<Self, F>
    where
        Self: Sized,
        F: Filter,
    {
        CompositeFilter::new(self, other)
    }
}

/// A filter that applies a closure to a pixel.
///
/// This filter is useful for creating custom filters that can be passed to the `apply` method.
///
/// # Type Parameters
/// * `F` - The type of the closure.
impl<F> Filter for F
where
    F: Fn(&RgbaPixel) -> bool + Send + Sync + 'static,
{
    #[inline(always)]
    fn test(&self, pixel: &RgbaPixel) -> bool {
        self(pixel)
    }
}

/// A filter that composites two filters together.
///
/// This filter applies the first filter and then the second filter.
///
/// # Type Parameters
/// * `F1` - The type of the 1st filter.
/// * `F2` - The type of the 2nd filter.
#[derive(Debug)]
pub struct CompositeFilter<F1, F2>
where
    F1: Filter,
    F2: Filter,
{
    first: F1,
    second: F2,
}

impl<F1, F2> CompositeFilter<F1, F2>
where
    F1: Filter,
    F2: Filter,
{
    /// Creates a new `CompositeFilter` instance.
    ///
    /// # Arguments
    /// * `first` - The first filter.
    /// * `second` - The second filter.
    ///
    /// # Returns
    /// A new `CompositeFilter` instance.
    #[must_use]
    pub fn new(first: F1, second: F2) -> Self {
        Self { first, second }
    }
}

impl<F1, F2> Filter for CompositeFilter<F1, F2>
where
    F1: Filter,
    F2: Filter,
{
    #[inline(always)]
    fn test(&self, pixel: &RgbaPixel) -> bool {
        // Apply the first filter and then the second filter
        // https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators
        self.first.test(pixel) && self.second.test(pixel)
    }
}

/// A filter that filters alpha values.
#[derive(Debug)]
pub struct AlphaFilter {
    threshold: u8,
}

impl AlphaFilter {
    /// Creates a new `AlphaFilter` instance.
    ///
    /// # Arguments
    /// * `threshold` - The alpha threshold for the filter.
    ///
    /// # Returns
    /// A new `AlphaFilter` instance.
    #[must_use]
    pub fn new(threshold: u8) -> Self {
        Self { threshold }
    }
}

impl Filter for AlphaFilter {
    #[inline(always)]
    fn test(&self, pixel: &RgbaPixel) -> bool {
        pixel[3] > self.threshold
    }
}

impl Default for AlphaFilter {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_filter() {
        // Act & Assert
        let filter = |pixel: &RgbaPixel| pixel[0] > 128; // Filter for red channel > 128
        assert_eq!(filter.test(&[255, 0, 0, 255]), true);
        assert_eq!(filter.test(&[129, 0, 0, 255]), true);
        assert_eq!(filter.test(&[128, 0, 0, 255]), false);
        assert_eq!(filter.test(&[0, 0, 0, 255]), false);
    }

    #[test]
    fn test_composite_filter() {
        // Arrange
        let alpha_filter = |pixel: &RgbaPixel| pixel[3] != 0;
        let green_filter = |pixel: &RgbaPixel| pixel[1] >= 128;

        // Act
        let filter = alpha_filter.composite(green_filter);

        // Assert
        assert_eq!(filter.test(&[255, 127, 255, 0]), false); // Alpha == 0 && green channel < 128
        assert_eq!(filter.test(&[255, 255, 255, 0]), false); // Alpha == 0 && green channel >= 128
        assert_eq!(filter.test(&[255, 127, 255, 255]), false); // Alpha != 0 && green channel < 128
        assert_eq!(filter.test(&[255, 128, 255, 255]), true); // Alpha != 0 && green channel >= 128
        assert_eq!(filter.test(&[255, 255, 255, 255]), true); // Alpha != 0 && green channel >= 128
    }

    #[test]
    fn test_alpha_filter() {
        // Act & Assert
        let filter = AlphaFilter::new(127);
        assert_eq!(filter.test(&[255, 0, 0, 255]), true);
        assert_eq!(filter.test(&[255, 0, 0, 128]), true);
        assert_eq!(filter.test(&[255, 0, 0, 127]), false);
        assert_eq!(filter.test(&[255, 0, 0, 0]), false);
    }
}
