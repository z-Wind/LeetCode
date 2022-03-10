use std::collections::{BinaryHeap, HashMap, HashSet};

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut map = vec![HashMap::new();11];
        for s in strs {
            *map[s.len()].entry(s).or_insert(0) += 1;
        }
        
        for i in (1..=10).rev() {
            let m = map.pop().unwrap();
            if m.len() == 0 {
                continue;
            }
            
            if m.values().any(|&x| x == 1) {
                return i as i32;
            }
            
            for s in m.keys() {
                for sub in gen_subseq(s) {
                    if sub.len() >= i {
                        continue;
                    }
                    if let Some(count) = map[sub.len()].get_mut(&sub) {
                        *count += 1;
                    }
                }
            }
        }
        
        -1
    }
}

fn gen_subseq(s: &str) -> HashSet<String> {
    let mut ans:HashSet<String> = HashSet::new();
    
    for c in s.chars() {
        let mut tmp = ans.clone();
        for mut ss in tmp.drain() {
            ss.push(c);
            ans.insert(ss);
        }
        
        ans.insert(c.to_string());
    }
    // println!("{}:{:?}", s, ans);
    
    ans
}