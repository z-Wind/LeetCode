// https://leetcode.com/problems/find-the-difference/discuss/86825/Java-solution-using-bit-manipulation

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let t = t.as_bytes();
        let n = t.len();
        let mut c:u8 = t[n-1];
        for (ct, cs) in t.iter().zip(s.bytes()){
            c ^= cs;
            c ^= ct;
        }
        c as char
    }
}