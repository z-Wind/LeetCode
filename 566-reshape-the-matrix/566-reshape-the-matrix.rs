impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;

        if (m == r && r == c) || (m * n != r * c) {
            return mat;
        }

        let mut result = vec![vec![0; c]; r];
        for i in 0..m {
            for j in 0..n {
                let loc = i * n + j;
                result[loc / c][loc % c] = mat[i][j];
            }
        }

        result
    }
}
