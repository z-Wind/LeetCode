use std::collections::HashSet;
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut bank:HashSet<String> = bank.into_iter().collect();
        let mut ans = i32::MAX;
        min_mutation(&mut ans, 0, &start, &end, &mut bank);
        if ans == i32::MAX{
            return -1;
        }
        ans
    }
}

fn min_mutation(ans:&mut i32, count:i32, start: &String, end: &String, bank: &mut HashSet<String>){
    if count >= *ans{
        return;
    }
    if start == end{
        *ans = count;
        return;
    }
    // println!("{} vs {}: {:?}", start, end, bank);
    for pos in (0..start.len()).rev(){        
        for c in vec!["A", "C", "G", "T"]{
            if c == &start[pos..pos+1]{
                continue;
            }
            let mut s = start.clone();
            s.replace_range(pos..pos+1, &c);
            // println!("{}:  {}\n -> {}", pos, start, s);
            if bank.contains(&s){
                bank.remove(&s);
                min_mutation(ans, count+1, &s, end, bank);
                bank.insert(s);
            }
        }
    }
}