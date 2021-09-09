// https://leetcode.com/problems/palindrome-partitioning-ii/discuss/42198/My-solution-does-not-need-a-table-for-palindrome-is-it-right-It-uses-only-O(n)-space.
use std::cmp::min;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cut = vec![0;n+1];  // number of cuts for the first k characters
        for i in (0..=n) {
            cut[i] = i-1;
        }
        for i in (0..n) {
            
            // odd length palindrome
            let mut j = 0;
            while i >= j && i+j < n && s[i-j]==s[i+j] {
                cut[i+j+1] = min(cut[i+j+1],1+cut[i-j]);
                j+=1;
            }
            
            // even length palindrome
            j = 1;
            while i+1 >= j && i+j < n && s[i-j+1] == s[i+j] {  
                cut[i+j+1] = min(cut[i+j+1],1+cut[i-j+1]);
                j+=1;
            }
        }
        cut[n] as i32
    }
}