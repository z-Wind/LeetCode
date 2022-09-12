// https://leetcode.com/problems/permutation-in-string/discuss/1761953/Python3-SLIDING-WINDOW-OPTIMIZED-(-)-Explained/1277759

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();
        if n > m {
            return false;
        }

        let mut chars_s1 = vec![0; 26];
        for c in s1.bytes() {
            chars_s1[(c - b'a') as usize] += 1;
        }

        let s2 = s2.as_bytes();
        let mut chars_s2 = vec![0; 26];
        let mut count = 0;
        for i in 0..m {
            chars_s2[(s2[i] - b'a') as usize] += 1;
            if i >= n {
                chars_s2[(s2[i - n] - b'a') as usize] -= 1;
            }

            if chars_s1 == chars_s2 {
                return true;
            }
        }

        false
    }
}
