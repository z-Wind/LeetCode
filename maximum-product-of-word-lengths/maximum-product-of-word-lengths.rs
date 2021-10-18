use std::collections::HashSet;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut set:Vec<HashSet<char>> = vec![HashSet::new();n];
        
        let mut max_val = 0;
        for i in 0..n{
            let w1 = words[i].len();
            if set[i].is_empty(){
                for c in words[i].chars(){
                    set[i].insert(c);
                }
            }
            for j in i+1..n{
                if set[j].is_empty(){
                    for c in words[j].chars(){
                        set[j].insert(c);
                    }
                }
                if !set[i].is_disjoint(&set[j]){
                    continue;
                }
                let w2 = words[j].len();
                // println!("{},{} => {}", words[i], words[j], w1*w2);
                max_val = max_val.max((w1*w2) as i32);
            }
        }
        
        max_val
    }
}