use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut ans = vec![];
        if s.len() < 11{
            return ans;
        }
        let mut map = HashMap::new();
        for i in (0..s.len()-9){
            // println!("{:?}",String::from_utf8(s[i..i+10].to_vec()).unwrap());
            let mut entry = map.entry(&s[i..i+10]).or_insert(0);
            *entry += 1;
            if *entry == 2{
                ans.push(s[i..i+10].to_string());
            }
        }
        ans
    }
}