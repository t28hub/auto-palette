/// Struct representing a label for DBSCAN clustering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    Assigned(usize),
    Outlier,
    Marked,
    Undefined,
}

impl Label {
    /// Returns whether this label is assigned.
    ///
    /// # Returns
    /// `true` if the label is assigned, otherwise `false`.
    pub fn is_assigned(&self) -> bool {
        matches!(*self, Label::Assigned(_))
    }

    /// Returns whether this label is outlier.
    ///
    /// # Returns
    /// `true` if the label is outlier, otherwise `false`.
    pub fn is_outlier(&self) -> bool {
        matches!(*self, Label::Outlier)
    }

    /// Returns whether this label is undefined.
    ///
    /// # Returns
    /// `true` if the label is undefined, otherwise `false`.
    pub fn is_undefined(&self) -> bool {
        matches!(*self, Label::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_assigned() {
        assert_eq!(Label::Assigned(0).is_assigned(), true);
        assert_eq!(Label::Outlier.is_assigned(), false);
        assert_eq!(Label::Marked.is_assigned(), false);
        assert_eq!(Label::Undefined.is_assigned(), false);
    }

    #[test]
    fn test_is_outlier() {
        assert_eq!(Label::Assigned(0).is_outlier(), false);
        assert_eq!(Label::Outlier.is_outlier(), true);
        assert_eq!(Label::Marked.is_outlier(), false);
        assert_eq!(Label::Undefined.is_outlier(), false);
    }

    #[test]
    fn test_is_undefined() {
        assert_eq!(Label::Assigned(0).is_undefined(), false);
        assert_eq!(Label::Outlier.is_undefined(), false);
        assert_eq!(Label::Marked.is_undefined(), false);
        assert_eq!(Label::Undefined.is_undefined(), true);
    }
}
