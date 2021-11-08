// https://github.com/z-Wind/LeetCode/blob/main/range-sum-query-2d-immutable/range-sum-query-2d-immutable.rs

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let num_matrix = NumMatrix::new(matrix);
        
        let mut ans = i32::MIN;
        for row1 in 0..m{
            for col1 in 0..n{
                for row2 in 0..=row1{
                    for col2 in 0..=col1{
                        let sum = num_matrix.sum_region(row2,col2,row1,col1);
                        // println!("{},{} -> {},{}: {}", row2, col2, row1, col1, sum);
                        if sum <= k{
                            ans = ans.max(sum);
                        }
                    }
                }
            }
        }
        ans
    }
}

struct NumMatrix {
    sums: Vec<Vec<i32>>,
}

impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sums:Vec<Vec<i32>> = vec![vec![0;matrix[0].len()+1];matrix.len()+1];
        for i in (0..matrix.len()){
            for j in (0..matrix[0].len()){
                sums[i+1][j+1] = sums[i][j+1] + sums[i+1][j] - sums[i][j] + matrix[i][j]
            }
        }
        
        // println!("{:?}", sums);
        Self { sums }
    }
    
    fn sum_region(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> i32 {    
        self.sums[row2+1][col2+1] - self.sums[row1][col2+1] - self.sums[row2+1][col1] + self.sums[row1][col1]
    }
}