use std::cmp::Ordering;

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

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
