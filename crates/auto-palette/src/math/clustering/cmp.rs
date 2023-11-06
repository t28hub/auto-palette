use std::cmp::Ordering;

/// Struct representing a priority item.
///
/// # Type Parameters
/// * `T` - The type of the item.
/// * `P` - The type of the priority.
#[derive(Debug)]
pub struct Priority<T, P: PartialOrd>(pub T, pub P);

impl<T, P> Priority<T, P>
where
    P: PartialOrd,
{
    /// Creates a new `Priority` instance.
    ///
    /// # Arguments
    /// * `item` - The item to be prioritized.
    /// * `priority` - The priority of the item.
    ///
    /// # Returns
    /// A new `Priority` instance.
    #[must_use]
    pub fn new(item: T, priority: P) -> Self {
        Self(item, priority)
    }
}

impl<T, P> Eq for Priority<T, P> where P: PartialOrd {}

impl<T, P> PartialEq for Priority<T, P>
where
    P: PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.1.eq(&other.1)
    }
}

impl<T, P> Ord for Priority<T, P>
where
    P: PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.partial_cmp(&other.1).unwrap_or(Ordering::Equal)
    }
}

impl<T, P> PartialOrd for Priority<T, P>
where
    P: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_new() {
        let priority = Priority::new((0, 1), 5.0);

        assert_eq!(priority.0, (0, 1));
        assert_eq!(priority.1, 5.0);
    }

    #[test]
    fn test_priority_eq() {
        let priority1 = Priority::new((0, 1), 5.0);
        let priority2 = Priority::new((0, 2), 5.0);

        assert_eq!(priority1, priority2);
    }

    #[test]
    fn test_priority_ord() {
        let priority1 = Priority::new((0, 1), 5.0);
        let priority2 = Priority::new((0, 2), 3.0);

        assert_eq!(priority1.cmp(&priority2), Ordering::Greater);
        assert_eq!(priority2.cmp(&priority1), Ordering::Less);
    }
}
