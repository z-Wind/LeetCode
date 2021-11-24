// https://leetcode.com/problems/is-subsequence/discuss/87302/Binary-search-solution-for-follow-up-with-detailed-comments
// Follow-up: O(N) time for pre-processing, O(Mlog?) for each S.
// Eg-1. s="abc", t="bahbgdca"
// idx=[a={1,7}, b={0,3}, c={6}]
//  i=0 ('a'): prev=1
//  i=1 ('b'): prev=3
//  i=2 ('c'): prev=6 (return true)
// Eg-2. s="abc", t="bahgdcb"
// idx=[a={1}, b={0,6}, c={5}]
//  i=0 ('a'): prev=1
//  i=1 ('b'): prev=6
//  i=2 ('c'): prev=? (return false)

use std::collections::HashMap;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut map:HashMap<char,Vec<usize>> = HashMap::with_capacity(26);
        for (i,c) in t.chars().enumerate(){
            map.entry(c).or_insert(Vec::new()).push(i);
        }
        // println!("{:?}", map);
        
        let mut prev = 0;
        for c in s.chars(){
            match map.get(&c){
                None => return false,
                Some(v) => {
                    let idx = v.binary_search(&prev).unwrap_or_else(|x| x);
                    if idx == v.len(){
                        return false;
                    }
                    prev = v[idx] + 1;
                }
            }
        }
        
        true
    }
}