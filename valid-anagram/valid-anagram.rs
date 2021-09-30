impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut buckets = [0; 26];
        for (i, j) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
            buckets[(i - b'a') as usize] += 1;
            buckets[(j - b'a') as usize] -= 1;
        }
        buckets == [0; 26]
    }
}

use std::collections::HashMap;
fn is_anagram_unicode(s: String, t: String) -> bool {
    if s.len() != t.len(){
        return false;
    }
    let mut map:HashMap<char,i32> = HashMap::with_capacity(s.len());
    let mut s = s.chars();
    let mut t = t.chars();
    while let (Some(cs), Some(ct)) = (s.next(), t.next()){
        *map.entry(cs).or_insert(0) += 1;
        *map.entry(ct).or_insert(0) -= 1;
    }
    for &v in map.values(){
        if v != 0{
            return false;
        }
    }
    true
}