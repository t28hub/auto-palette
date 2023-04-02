#[derive(Debug, PartialEq)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    next_label: usize,
}

impl UnionFind {
    /// Creates a new `UnionFind` instance.
    ///
    /// # Arguments
    /// * `n` - The number of elements.
    ///
    /// # Returns
    /// A new `UnionFind` instance.
    ///
    /// # Panics
    /// Panics if the given `n` less than or equal to 0.
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        let parent: Vec<usize> = (0..2 * n).collect();
        let size: Vec<usize> = (0..2 * n)
            .map(|index| if index < n { 1 } else { 0 })
            .collect();
        Self {
            parent,
            size,
            next_label: n,
        }
    }

    /// Finds the parent node of the given `x`.
    ///
    /// # Arguments
    /// * `x` - The node to find its parent
    ///
    /// # Returns
    /// The the parent node of the given `x`.
    pub fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        let mut parenet = x;
        // Find the root node of the given index.
        while self.parent[root] != root {
            root = self.parent[root];
        }

        while self.parent[parenet] != root {
            parenet = self.parent[parenet];
            self.parent[parenet] = root;
        }
        root
    }

    /// Merges the `x` and `y`.
    ///
    /// # Arguments
    /// * `x` - A node in the first node to merge.
    /// * `y` - A node in the other node to merge.
    ///
    /// # Returns
    /// The size of the merged node.
    pub fn union(&mut self, x: usize, y: usize) -> usize {
        self.parent[x] = self.next_label;
        self.parent[y] = self.next_label;

        let total = self.size[x] + self.size[y];
        self.size[self.next_label] = total;
        self.next_label += 1;
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        assert_eq!(
            UnionFind::new(3),
            UnionFind {
                parent: vec![0, 1, 2, 3, 4, 5],
                size: vec![1, 1, 1, 0, 0, 0],
                next_label: 3,
            }
        )
    }

    #[test]
    #[should_panic(expected = "assertion failed: n > 0")]
    fn test_union_find_panic() {
        let _ = UnionFind::new(0);
    }

    #[test]
    fn test_find() {
        let mut union_find = UnionFind::new(4);
        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 1);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 3);

        union_find.union(0, 1);
        assert_eq!(union_find.find(0), 4);
        assert_eq!(union_find.find(1), 4);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 3);
        assert_eq!(union_find.find(4), 4);

        union_find.union(4, 3);
        assert_eq!(union_find.find(0), 5);
        assert_eq!(union_find.find(1), 5);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 5);
        assert_eq!(union_find.find(4), 5);
        assert_eq!(union_find.find(5), 5);
    }

    #[test]
    fn test_union() {
        let mut union_find = UnionFind::new(4);
        assert_eq!(union_find.union(0, 1), 2);
        assert_eq!(union_find.union(2, 3), 2);
        assert_eq!(union_find.union(4, 5), 4);
    }
}
