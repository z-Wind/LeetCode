// https://leetcode.com/problems/diagonal-traverse/discuss/581868/Easy-Python-NO-DIRECTION-CHECKING

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = Vec::with_capacity(m * n);

        let mut i = 0;
        let mut j = 0;
        for _ in 0..m * n {
            // println!("{},{}", i, j);
            ans.push(mat[i][j]);

            // (i + j) % 2 == 0
            if (i + j) & 1 == 0 {
                if j == n - 1 {
                    i += 1;
                } else if i == 0 {
                    j += 1;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else {
                if i == m - 1 {
                    j += 1;
                } else if j == 0 {
                    i += 1;
                } else {
                    i += 1;
                    j -= 1;
                }
            }
        }

        ans
    }
}
