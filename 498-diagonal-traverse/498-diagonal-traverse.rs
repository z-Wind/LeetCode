impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = Vec::with_capacity(m * n);

        let mut up = true;
        let mut i = 0;
        let mut j = 0;
        for _ in 0..m * n {
            // println!("{},{}", i, j);
            ans.push(mat[i][j]);

            match (up, i, j) {
                (true, ..) if j == n - 1 => {
                    i += 1;
                    up = false;
                }
                (true, ..) if i == 0 => {
                    j += 1;
                    up = false;
                }
                (true, ..) => {
                    i -= 1;
                    j += 1;
                }
                (false, ..) if i == m - 1 => {
                    j += 1;
                    up = true;
                }
                (false, ..) if j == 0 => {
                    i += 1;
                    up = true;
                }
                (false, ..) => {
                    i += 1;
                    j -= 1;
                }
            }
        }

        ans
    }
}
