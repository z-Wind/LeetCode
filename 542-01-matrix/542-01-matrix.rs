// https://leetcode.com/problems/01-matrix/discuss/1369741/C%2B%2BJavaPython-BFS-DP-solutions-with-Picture-Clean-and-Concise-O(1)-Space
// from zero cells

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());

        let mut deq = VecDeque::new();
        for r in 0..m {
            for c in 0..n {
                if mat[r][c] == 0 {
                    deq.push_back((r, c));
                } else {
                    mat[r][c] = -1; // Marked as not processed yet!
                }
            }
        }
        
        let DIR = vec![0, 1, 0, -1, 0];
        while let Some((r, c)) = deq.pop_front() {
            for i in 0..4 {
                let (nr, nc) = ((r as i32 + DIR[i]) as usize, (c as i32 + DIR[i + 1]) as usize);
                if nr >= m || nc >= n || mat[nr][nc] != -1 {
                    continue;
                }
                mat[nr][nc] = mat[r][c] + 1;
                deq.push_back((nr, nc));
            }
        }
        return mat;
    }
}
