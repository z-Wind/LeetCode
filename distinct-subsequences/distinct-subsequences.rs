// https://leetcode.com/problems/distinct-subsequences/discuss/37316/7-10-lines-C%2B%2B-Solutions-with-Detailed-Explanations-(O(m*n)-time-and-O(m)-space)
// https://leetcode.com/problems/distinct-subsequences/discuss/37327/Easy-to-understand-DP-in-Java
//
//   S 0123..j..n
// T +----------+
//   |1111111111|
// 0 |0         |
// 1 |0         |
// 2 |0         |
// . |0         |
// . |0         |
// i |0         |
// . |0         |
// . |0         |
// m |0         |
// 
// dp[i][j] = dp[i][j-1]                    when s[j-1] != t[i-1]
// dp[i][j] = dp[i][j-1]   +   dp[i-1][j-1] when s[j-1] == t[i-1]
//           exclude s[j-1]   include s[j-1]
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();
        let mut dp:Vec<Vec<i32>> = vec![vec![0;n+1];m+1];
        for j in (0..=n) { dp[0][j] = 1;}
        for j in (1..=n){
            for i in (1..=m){
                dp[i][j] = match s[j-1] == t[i-1]{
                    false => dp[i][j-1],
                    true => dp[i][j-1] + dp[i-1][j-1],
                }
            }
        }
        return dp[m][n];
    }
}