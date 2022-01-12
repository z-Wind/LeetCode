// https://leetcode.com/problems/count-the-repetitions/discuss/95398/C%2B%2B-solution-inspired-by-%4070664914-with-organized-explanation
// Time: O(|s1| * n1) where |s1| is the length of s1
// Space: O(n1)

use std::collections::HashMap;
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let n1 = n1 as usize;
        let n2 = n2 as usize;
        
        let mut kToRepeatCount:HashMap<usize,usize> = HashMap::new();
        let mut nextIndexToK:HashMap<usize,usize> = HashMap::new();
        kToRepeatCount.insert(0,0);
        nextIndexToK.insert(0,0);
        let mut j = 0;
        let mut cnt = 0;
        for k in 1..=n1 {
            for i in 0..s1.len() {
                if s1[i] == s2[j] {
                    j += 1;
                    if j == s2.len() {
                        j = 0;
                        cnt += 1;
                    }
                }
            }
            if let Some(start) = nextIndexToK.get(&j) {
                let prefixCount = *kToRepeatCount.get(start).unwrap();
                let patternCount = (n1 - start) / (k - start) * (cnt - prefixCount);
                let key = start + (n1 - start) % (k - start);
                let suffixCount = *kToRepeatCount.get(&key).unwrap() - prefixCount;
                return ((prefixCount + patternCount + suffixCount) / n2) as i32;
            }
            kToRepeatCount.insert(k,cnt);
            nextIndexToK.insert(j,k);
        }
        return (*kToRepeatCount.get(&n1).unwrap() / n2) as i32;
    }
}