// https://leetcode.com/problems/number-of-provinces/discuss/101336/Java-solution-Union-Find/266043
// Union Find

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<i32>,
    count: i32,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n as i32,
        }
    }

    fn find(&mut self, mut p: usize) -> usize {
        // path compression
        while p != self.parent[p] {
            self.parent[p] = self.parent[self.parent[p]];
            p = self.parent[p];
        }
        return p;
    }

    fn union(&mut self, p: usize, q: usize) {
        let rootP = self.find(p);
        let rootQ = self.find(q);
        if rootP == rootQ {
            return;
        }

        // union by size
        if self.size[rootP] > self.size[rootQ] {
            self.parent[rootQ] = rootP;
            self.size[rootP] += self.size[rootQ];
        } else {
            self.parent[rootP] = rootQ;
            self.size[rootQ] += self.size[rootP];
        }
        self.count -= 1;
    }

    fn count(&self) -> i32 {
        self.count
    }
}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 {
                    uf.union(i, j);
                }
            }
        }

        uf.count
    }
}
