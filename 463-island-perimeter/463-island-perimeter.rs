// https://leetcode.com/problems/island-perimeter/discuss/95001/clear-and-easy-java-solution

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut res = 0;;
        for i in 0..m {
            for j in 0..n {
                if (grid[i][j] == 1) {
                    res += 4; // count islands
                    if (i + 1 < m && grid[i + 1][j] == 1) {
                        res -= 2; // count down neighbours
                    }
                    if (j + 1 < n && grid[i][j + 1] == 1) {
                        res -= 2; // count right neighbours
                    }
                }
            }
        }

        res
    }
}
