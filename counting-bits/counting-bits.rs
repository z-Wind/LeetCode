// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution
// f[i] = f[i / 2] + i % 2.

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans = vec![0;n+1];
        for i in 0..=n{
            ans[i] = ans[i>>1] + (i as i32 & 1);
        }
        ans
    }
}