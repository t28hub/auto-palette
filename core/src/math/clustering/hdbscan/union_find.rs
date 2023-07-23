use std::cmp::Ordering;

/// Struct representing a union-find data structure.
#[derive(Debug, PartialEq)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    /// Creates a new `UnionFind` instance.
    ///
    /// # Arguments
    /// * `n` - The number of nodes.
    ///
    /// # Returns
    /// A new `UnionFind` instance.
    ///
    /// # Panics
    /// Panics if the given `n` less than 0.
    #[must_use]
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }

    /// Finds the parent node of the given `x`.
    ///
    /// # Arguments
    /// * `x` - The node to find its parent
    ///
    /// # Returns
    /// The the parent node of the given `x`.
    #[inline]
    #[must_use]
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Merges the `x` and `y`.
    ///
    /// # Arguments
    /// * `x` - A node in the first node to merge.
    /// * `y` - A node in the other node to merge.
    #[inline]
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.parent[x];
        let root_y = self.parent[y];
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            Ordering::Less => {
                self.parent[root_x] = root_y;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let actual = UnionFind::new(8);
        assert_eq!(
            actual,
            UnionFind {
                parent: vec![0, 1, 2, 3, 4, 5, 6, 7],
                rank: vec![0, 0, 0, 0, 0, 0, 0, 0],
            }
        );
    }

    #[test]
    #[should_panic(expected = "assertion failed: n > 0")]
    fn test_union_find_panic() {
        let _ = UnionFind::new(0);
    }

    #[test]
    fn test_operations() {
        let mut union_find = UnionFind::new(4);
        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 1);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 3);

        union_find.union(0, 1);
        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 0);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 3);

        union_find.union(2, 3);
        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 0);
        assert_eq!(union_find.find(2), 2);
        assert_eq!(union_find.find(3), 2);

        union_find.union(1, 3);
        assert_eq!(union_find.find(0), 0);
        assert_eq!(union_find.find(1), 0);
        assert_eq!(union_find.find(2), 0);
        assert_eq!(union_find.find(3), 0);
    }
}
