// https://leetcode.com/problems/reshape-the-matrix/discuss/2151421/3ms-simple-solution

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;

        if (m == r && r == c) || (m * n != r * c) {
            return mat;
        }

        mat.concat() // flatten
            .chunks(c) // devide
            .map(|row| row.to_vec())
            .collect() // restructure
    }
}
