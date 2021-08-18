use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (grid.len(), grid[0].len());
        let mut cur = vec![0;n as usize];
        for i in (0..m) {
            for j in (0..n) {
                if i == 0 && j == 0 {
                    cur[j] = grid[i][j];
                } else if j == 0{
                    cur[j] += grid[i][j]; 
                } else if i == 0{
                    cur[j] += grid[i][j] + cur[j-1]; 
                } else {
                    cur[j] = (grid[i][j] + min(cur[j], cur[j-1]));
                }   
                //println!("({},{}):{} => {:?}",i, j, grid[i][j], cur);
            }
        }
        return cur[n - 1];
    }
}