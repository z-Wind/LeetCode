// https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m,n) = (m as usize, n as usize);
        let mut cur = vec![1;n as usize];
        for i in (1..m) {
            for j in (1..n) {
                cur[j] += cur[j - 1];
            }
        }
        return cur[n - 1];
    }
}