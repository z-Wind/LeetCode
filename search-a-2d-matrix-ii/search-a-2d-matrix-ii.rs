// https://leetcode.com/problems/search-a-2d-matrix-ii/discuss/66140/My-concise-O(m%2Bn)-Java-solution

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        
        let mut i = 0;
        let mut j = n-1;
        while i < m && j < n{
            // println!("{},{}",i,j);
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Greater => j -= 1,
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
}