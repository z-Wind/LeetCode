use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern = pattern.as_bytes();
        let mut words:Vec<&str> = s.split(' ').collect();
        if pattern.len() != words.len(){
            return false;
        }
        
        let mut map_p:HashMap<u8,usize> = HashMap::new();
        let mut map_s:HashMap<&str,usize> = HashMap::new();
        
        for i in (0..words.len()){
            let p = pattern[i];
            let s = words[i];
            match (map_p.get(&p), map_s.get(&s)){
                (None,None) =>{
                    map_p.insert(pattern[i], i);
                    map_s.insert(s, i);
                },
                (Some(x),Some(y)) => {
                    if x != y{
                        return false;
                    }
                },
                (None,Some(_)) | (Some(_),None) =>{
                    return false;
                },
            }            
        }
        true
    }
}