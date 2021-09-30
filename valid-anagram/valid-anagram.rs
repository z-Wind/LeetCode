use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
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
}