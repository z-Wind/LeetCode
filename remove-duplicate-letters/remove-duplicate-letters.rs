use std::collections::{VecDeque, BTreeMap};
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count_c:BTreeMap<u8, VecDeque<usize>> = BTreeMap::new();
        for (i,c) in s.bytes().enumerate(){
            count_c.entry(c).or_insert(VecDeque::new()).push_back(i);
        }
        
        let mut temp = Vec::new();
        match remove_duplicate_letters(&mut temp, 0, count_c){
            Some(ans) => String::from_utf8(ans).unwrap(),
            _ => String::new(),
        }
    }
}

fn remove_duplicate_letters(temp:&mut Vec<u8>, start:usize, count_c: BTreeMap<u8, VecDeque<usize>>) -> Option<Vec<u8>>{
    if count_c.is_empty(){
        return Some(temp.clone());
    }
    
    'outer: for (c, v1) in count_c.iter(){
        let mut counts = count_c.clone();
        counts.remove(c);
        for v2 in counts.values_mut(){
            while v2[0] < v1[0]{
                v2.pop_front();
                if v2.is_empty(){
                    continue 'outer;   
                }
            }
        }
        temp.push(*c);
        match remove_duplicate_letters(temp, start+1, counts){
            None => (),
            x => return x,
        }
        temp.pop();
    }
    None
}
