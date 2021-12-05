// https://leetcode.com/problems/trapping-rain-water-ii/discuss/89495/How-to-get-the-solution-to-2-D-%22Trapping-Rain-Water%22-problem-from-1-D-case

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();
        let dirs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut res = 0;
        let mut pq: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];

        // outer line
        for i in 0..m {
            pq.push((Reverse(height_map[i][0]), i, 0));
            visited[i][0] = true;
            pq.push((Reverse(height_map[i][n - 1]), i, n - 1));
            visited[i][n - 1] = true;
        }
        for i in 1..n - 1 {
            pq.push((Reverse(height_map[0][i]), 0, i));
            visited[0][i] = true;
            pq.push((Reverse(height_map[m - 1][i]), m - 1, i));
            visited[m - 1][i] = true;
        }

        while let Some((Reverse(h), x, y)) = pq.pop() {
            for &(offset_x, offset_y) in dirs.iter() {
                let i = x + offset_x as usize;
                let j = y + offset_y as usize;
                if (i >= m || j >= n || visited[i][j]) {
                    continue;
                }
                res += 0.max(h - height_map[i][j]);
                pq.push((Reverse(h.max(height_map[i][j])), i, j));
                visited[i][j] = true;
            }
        }

        return res;
    }
}