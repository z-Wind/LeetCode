use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0{
            return vec![];
        }
        let mut map:HashMap<usize, Vec<String>> = HashMap::with_capacity(strs.len()/4);
        
        for s in strs{
            let key = cal_key(&s);
            map.entry(key).or_insert(vec![]).push(s);
        }
        
        map.values().cloned().collect()
    }
}

fn cal_key(s: &str) -> usize{
	let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101 ];

	let mut key = 1;
	for c in s.chars() {
        let index = c as u32 - 'a' as u32;
		let prime = primes[index as usize];
		key *= prime;
	}
	return key;
}