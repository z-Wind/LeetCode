use std::mem;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut zeros:Vec<(usize,usize)> = vec![];
        for i in (0..m){
            for j in (0..n){
                if matrix[i][j] == 0 {
                    zeros.push((i,j));
                }
            }
        }
        for (i,j) in zeros{
            for k in (0..m){
                matrix[k][j] = 0;
            }
            for k in (0..n){
                matrix[i][k] = 0;
            }
        }
    }
}