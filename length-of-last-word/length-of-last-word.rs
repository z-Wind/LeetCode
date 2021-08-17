impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut word:Vec<char> = vec![];
        
        for c in s.trim_end().chars().rev(){
            if c == ' '{
                break;
            } else {
                word.push(c);
            }
        }
        
        word.len() as i32      
    }
}