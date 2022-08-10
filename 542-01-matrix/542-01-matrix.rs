// https://leetcode.com/problems/01-matrix/discuss/1369741/C%2B%2BJavaPython-BFS-DP-solutions-with-Picture-Clean-and-Concise-O(1)-Space

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());

        for r in 0..m {
            for c in 0..n {
                if mat[r][c] > 0 {
                    let top = if r > 0 { mat[r - 1][c] } else { i32::MAX };
                    let left = if c > 0 { mat[r][c - 1] } else { i32::MAX };
                    mat[r][c] = top.min(left).saturating_add(1);
                }
            }
        }

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if mat[r][c] > 0 {
                    let bottom = if r < m - 1 { mat[r + 1][c] } else { i32::MAX };
                    let right = if c < n - 1 { mat[r][c + 1] } else { i32::MAX };
                    mat[r][c] = mat[r][c].min(bottom.min(right).saturating_add(1));
                }
            }
        }

        return mat;
    }
}
