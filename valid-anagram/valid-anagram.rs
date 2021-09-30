use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map:HashMap<char,i32> = HashMap::new();
        for c in s.chars(){
            *map.entry(c).or_insert(0) += 1;
        }
        for c in t.chars(){
            match map.get_mut(&c){
                None => return false,
                Some(x) => {
                    *x -= 1;
                    if *x == 0{
                        map.remove(&c);
                    }
                },
            }
        }
        map.is_empty()
    }
}