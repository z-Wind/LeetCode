impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut i = 0;
        let mut j = 0;
        while i < m{
            let order = if matrix[i][j] <= target{
                1
            } else {
                -1
            };
            while j < n{
                // println!("{},{}",i,j);
                if matrix[i][j]  == target{
                    return true
                }
                
                match (order, matrix[i][j] < target){
                    (1,true) => j+=1,
                    (-1,false) => j-=1,
                    _ => break,
                }
            }
            i+=1;
            if j == n{
                j = n-1;
            } else if j > n{
                j = 0;
            }
        }
        false
    }
}