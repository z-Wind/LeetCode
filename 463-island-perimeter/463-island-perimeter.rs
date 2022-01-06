// https://leetcode.com/problems/island-perimeter/discuss/95001/clear-and-easy-java-solution

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut islands = 0;
        let mut neighbours = 0;

        for i in 0..m {
            for j in 0..n {
                if (grid[i][j] == 1) {
                    islands += 1; // count islands
                    if (i + 1 < m && grid[i + 1][j] == 1) {
                        neighbours += 1; // count down neighbours
                    }
                    if (j + 1 < n && grid[i][j + 1] == 1) {
                        neighbours += 1; // count right neighbours
                    }
                }
            }
        }

        return islands * 4 - neighbours * 2;
    }
}
