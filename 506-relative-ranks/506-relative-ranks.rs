use std::collections::BTreeMap;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut map = BTreeMap::new();
        for &n in score.iter() {
            map.insert(n, String::from(""));
        }
        
        for (i, value) in map.values_mut().rev().enumerate() {
            *value = match i+1 {
                1 => String::from("Gold Medal"),
                2 => String::from("Silver Medal"),
                3 => String::from("Bronze Medal"),
                x => x.to_string()
            }
        }
        
        score.iter().map(|x| map.get(x).unwrap().clone()).collect()
    }
}