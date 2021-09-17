use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 11{
            return vec![];
        }
        let mut map = HashMap::new();
        let s = s.as_bytes();
        for i in (0..s.len()-9){
            // println!("{:?}",String::from_utf8(s[i..i+10].to_vec()).unwrap());
            *map.entry(&s[i..i+10]).or_insert(0) += 1;
        }
        map.iter()
            .filter(|(key,val)| **val > 1)
            .map(|(x,_)| String::from_utf8(x.to_vec()).unwrap())
            .collect()
    }
}