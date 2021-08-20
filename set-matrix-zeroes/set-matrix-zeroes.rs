use std::mem;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans:Vec<Vec<i32>> = matrix.clone();
        for i in (0..m){
            for j in (0..n){
                if matrix[i][j] == 0 {
                    for k in (0..m){
                        ans[k][j] = 0;
                    }
                    for k in (0..n){
                        ans[i][k] = 0;
                    }
                }
            }
        }
        mem::swap(&mut ans, matrix);
    }
}