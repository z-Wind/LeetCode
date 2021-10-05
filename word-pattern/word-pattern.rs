use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern = pattern.as_bytes();
        let mut v:Vec<&str> = s.split(' ').collect();
        if pattern.len() != v.len(){
            return false;
        }
        
        let mut map_p:HashMap<u8,&str> = HashMap::new();
        let mut map_s:HashMap<&str,u8> = HashMap::new();
        
        for i in (0..v.len()){
            let p = pattern[i];
            let s = v[i];
            match (map_p.get(&p), map_s.get(&s)){
                (None,None) =>{
                    map_p.insert(pattern[i], &s);
                    map_s.insert(s, pattern[i]);
                },
                (Some(s2),Some(p2)) => {
                    if &s != s2 || &p != p2{
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