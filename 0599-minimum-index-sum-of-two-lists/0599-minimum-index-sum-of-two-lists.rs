use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let map1: HashMap<String, usize> =
            list1.into_iter().enumerate().map(|(i, s)| (s, i)).collect();

        let mut result = Vec::new();
        let mut min_sum = usize::MAX;
        for (i, s) in list2.into_iter().enumerate() {
            if let Some(j) = map1.get(&s) {
                let sum = i + j;
                match sum.cmp(&min_sum) {
                    Ordering::Greater => (),
                    Ordering::Less => {
                        result.clear();
                        result.push(s);
                        min_sum = sum;
                    }
                    Ordering::Equal => {
                        result.push(s);
                    }
                }
            }
        }

        result
    }
}
