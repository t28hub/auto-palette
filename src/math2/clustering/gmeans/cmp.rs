use std::cmp::Ordering;

/// Struct storing data ordered by size.
///
/// # Type parameters
/// * `T` - The type of the data.
#[derive(Debug)]
pub struct SizeOrdered<T> {
    size: usize,
    data: T,
}

impl<T> SizeOrdered<T> {
    /// Creates a new `SizeOrdered` instance.
    ///
    /// # Arguments
    /// * `size` - The size of the data.
    /// * `data` - The actual data.
    ///
    /// # Returns
    /// A new `SizeOrdered` instance.
    #[must_use]
    pub fn new(size: usize, data: T) -> Self {
        Self { size, data }
    }

    /// Returns the size of the data.
    ///
    /// # Returns
    /// The size of the data.
    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns reference to the actual data.
    ///
    /// # Returns
    /// Reference to the actual data.
    #[inline]
    #[must_use]
    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<T> Eq for SizeOrdered<T> {}

impl<T> PartialEq<Self> for SizeOrdered<T> {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl<T> PartialOrd for SizeOrdered<T> {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl<T> Ord for SizeOrdered<T> {
    #[inline]
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_ordered() {
        let a = SizeOrdered::new(5, "alice");
        let b = SizeOrdered::new(3, "bob");
        let c = SizeOrdered::new(7, "charlie");

        assert!(a > b);
        assert!(a < c);
        assert!(b < c);
    }
}
