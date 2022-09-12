// https://leetcode.com/problems/permutation-in-string/discuss/1761953/Python3-SLIDING-WINDOW-OPTIMIZED-(-)-Explained/1277759

use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n = s1.len();
        let m = s2.len();
        if n > m {
            return false;
        }

        let mut chars: HashMap<u8, i32> = HashMap::new();
        for c in s1.bytes() {
            chars.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        let s2 = s2.as_bytes();
        let mut count = 0;
        for i in 0..m {
            if let Some(x) = chars.get_mut(&s2[i]) {
                *x -= 1;
                if *x == 0 {
                    count += 1;
                }
            }

            if i >= n {
                if let Some(x) = chars.get_mut(&s2[i - n]) {
                    *x += 1;
                    if *x == 1 {
                        count -= 1;
                    }
                }
            }

            if count == chars.len() {
                return true;
            }
        }

        false
    }
}
