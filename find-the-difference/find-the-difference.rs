// https://leetcode.com/problems/find-the-difference/discuss/86825/Java-solution-using-bit-manipulation

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = t.len();
        let mut ans:u8 = t[n-1];
        for i in 0..n-1{
            ans ^= s[i];
            ans ^= t[i];
        }
        ans as char
    }
}