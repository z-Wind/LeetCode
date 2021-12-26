impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        
        let mut i = 0;
        let mut j = 0;
        while j < n {
            let curr = chars[j];
            let mut count = 0;
            while j < n && curr == chars[j] {
                count += 1;
                j += 1;
            }
            
            chars[i] = curr;
            i += 1;
            if count > 1 {
                for c in count.to_string().chars(){
                    chars[i] = c;
                    i += 1;   
                }
            }
        }

        i as i32
    }
}
