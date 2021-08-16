use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0{
            return vec![];
        }
        let mut map:HashMap<[usize;26], Vec<String>> = HashMap::new();
        
        for s in strs{
            let mut key = [0;26];
            for c in s.chars(){
                let index = c as u32 - 'a' as u32;
                key[index as usize] += 1;
            }
            (*map.entry(key).or_insert(vec![])).push(s);
        }
        
        let mut ans: Vec<Vec<String>> = vec![];
        for (_,v) in map.into_iter(){
            ans.push(v);
        }
        ans
    }
}