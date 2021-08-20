impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n -1;
        
        while left <= right {
            let mid = (left + right)/2;
            let (i,j) = to_i_j(mid,n);
            //println!("{} => ({},{})", mid, i, j);
            if matrix[i][j] == target{
                return true;
            } else if matrix[i][j] > target{
                match mid.checked_sub(1){
                    None => break,
                    Some(x) => right = x,
                }
            } else {
                match mid.checked_add(1){
                    None => break,
                    Some(x) => left = x,
                }
            }
        }
        false
    }
}

fn to_i_j(x:usize, n:usize) -> (usize,usize){
    (x/n, x%n)
}