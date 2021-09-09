// https://leetcode.com/problems/palindrome-partitioning-ii/discuss/42198/My-solution-does-not-need-a-table-for-palindrome-is-it-right-It-uses-only-O(n)-space.
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cut:Vec<i32> = (-1..=n as i32).collect();  // number of cuts for the first k characters
        for mid in (0..n) {
            
            // odd length palindrome
            let mut shift = 0;
            while mid >= shift && mid+shift < n && s[mid-shift]==s[mid+shift] {
                cut[mid+shift+1] = cut[mid+shift+1].min(1+cut[mid-shift]);
                shift+=1;
            }
            
            // even length palindrome
            shift = 1;
            while mid+1 >= shift && mid+shift < n && s[mid-shift+1] == s[mid+shift] {  
                cut[mid+shift+1] = cut[mid+shift+1].min(1+cut[mid-shift+1]);
                shift+=1;
            }
        }
        cut[n]
    }
}