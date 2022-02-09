impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = Vec::with_capacity(m*n);
        
        let mut rev = false;
        for i_start in 0..m {
            let mut temp = Vec::new();
            let mut i = i_start;
            let mut j = 0;
            while i < m && j < n {
                temp.push(mat[i][j]);
                i -= 1;
                j += 1;
            }
            if rev {
                temp.reverse();
            }
            ans.extend_from_slice(&temp);
            rev = !rev;
        }
        for j_start in 1..n {
            let mut temp = Vec::new();
            let mut i = m - 1;
            let mut j = j_start;
            while i < m && j < n {
                temp.push(mat[i][j]);
                i -= 1;
                j += 1;
            }
            if rev {
                temp.reverse();
            }
            ans.extend_from_slice(&temp);
            rev = !rev;
        }
        
        ans
    }
}