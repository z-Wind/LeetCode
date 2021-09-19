use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut map_s:HashMap<u8,u8> = HashMap::new();
        let mut map_t:HashMap<u8,u8> = HashMap::new();
        
        for i in (0..s.len()){
            match (map_s.get(&s[i]), map_t.get(&t[i])){
                (None,None) =>{
                    let diff = s[i] - t[i];
                    map_s.insert(s[i], diff);
                    map_t.insert(t[i], diff);
                },
                (Some(diff),_) =>{
                    if s[i] != diff + t[i]{
                        return false;
                    }
                },
                (None,Some(diff)) =>{
                    if s[i]-diff != t[i]{
                        return false;
                    }
                },
            } 
        }
        true
    }
}