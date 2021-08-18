// https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut cur = vec![0;n as usize];
        cur[0] = 1;
        for i in (0..m) {
            for j in (0..n) {
                if obstacle_grid[i][j] == 1{
                    cur[j] = 0;
                } else if j>0{
                    cur[j] += cur[j - 1];
                }
            }
        }
        return cur[n - 1];
    }
}