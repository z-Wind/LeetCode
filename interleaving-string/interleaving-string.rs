// dp[i][j] = (dp[i - 1][j] && s1.charAt(i - 1) == s3.charAt(i + j - 1))
//         || (dp[i][j - 1] && s2.charAt(j - 1) == s3.charAt(i + j - 1))
//
//            dp[i-1][j]
// dp[i][j-1] dp[i][j]
//         
//          dp[j]
// dp[j-1]  dp[j](update)
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
         if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len() + 1]; // include empty string
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                dp[j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[j - 1] && s2[j - 1] == s3[i + j - 1],
                    (_, 0) => dp[j] && s1[i - 1] == s3[i + j - 1],
                    _ => (dp[j - 1] && s2[j - 1] == s3[i + j - 1]) || 
                         (dp[j]     && s1[i - 1] == s3[i + j - 1]),
                }
            }
        }
        dp[s2.len()]
    }
}