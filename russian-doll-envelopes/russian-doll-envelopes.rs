// https://leetcode.com/problems/russian-doll-envelopes/discuss/82763/Java-NLogN-Solution-with-Explanation
// https://github.com/z-Wind/LeetCode/blob/main/longest-increasing-subsequence/longest-increasing-subsequence.rs

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_unstable_by(|a, b| 
            if a[0] == b[0]{
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        );
        // println!("{:?}", envelopes);
        
        let mut piles = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            let num = envelope[1];
            let idx = piles.binary_search(&num).unwrap_or_else(|e| e);
            if idx == piles.len(){
                piles.push(num);
            } else {
                piles[idx] = num;
            }
        }
        piles.len() as i32
    }
}