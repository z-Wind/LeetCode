use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2{
            return s.len() as i32
        }
             
        let (mut start, mut maxlen) = (0, 0);
        let mut record:HashMap<char, usize> = HashMap::new();
        
        for (i,c) in s.chars().enumerate(){
            if let Some(&loc) = record.get(&c){
                if loc >= start{
                    let len = i - start;
                                
                    if len > maxlen{
                        maxlen = len;
                    }
                    
                    start = loc + 1
                }
            }
            let len = i - start + 1;

            if len > maxlen{
                maxlen = len;
            }
            
            record.insert(c,i);
        }
        
        return maxlen as i32
    }
}