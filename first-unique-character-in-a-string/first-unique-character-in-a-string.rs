impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counts:Vec<i32> = vec![-1;26];
        for (idx,c) in s.bytes().enumerate(){
            let i = (c - b'a') as usize;
            if counts[i] == -1{
                counts[i] = idx as i32;    
            } else {
                counts[i] = -2;
            }
            
        }
        
        match counts.iter().filter(|x| **x>=0).min(){
            Some(&i) => i,
            None => -1,
        }
    }
}