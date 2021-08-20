use std::collections::HashSet;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut col_clear = false;
        for i in (0..m){
            if matrix[i][0] == 0{
                col_clear = true;
            } 
            for j in (1..n){
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }
        println!("{:?}",matrix);
        for i in (1..m){
            for j in (1..n){
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        
        for j in (1..n){
            if matrix[0][0] == 0 {
                matrix[0][j] = 0;
            }
        }
        
        if col_clear{
            for i in (0..m){
                    matrix[i][0] = 0;
                
            }
        }
    }
}