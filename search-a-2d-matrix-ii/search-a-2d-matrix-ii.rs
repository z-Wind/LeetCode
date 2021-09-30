// https://leetcode.com/problems/search-a-2d-matrix-ii/discuss/66140/My-concise-O(m%2Bn)-Java-solution

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut i = 0;
        let mut j = n-1;
        while i < m && j < n{
            // println!("{},{}",i,j);
            if matrix[i][j] == target{
                return true;
            } else if matrix[i][j] > target{
                j-=1;
            } else {
                i+=1;
            }
        }
        false
    }
}