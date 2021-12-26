impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        
        let mut i = 0;
        let mut start = 0;
        let mut end = 0;
        let mut count = 0;
        while end < n {
            if chars[start] == chars[end] {
                count += 1;
                end += 1;
            } else {
                chars[i] = chars[start];
                i += 1;
                if count > 1 {
                    for c in count.to_string().chars(){
                        chars[i] = c;
                        i += 1;   
                    }
                }
                count = 0;
                start = end;
            }
        }
        chars[i] = chars[start];
        i += 1;
        if count > 1 {
            for c in count.to_string().chars(){
                chars[i] = c;
                i += 1;   
            }
        }
        // for _ in i..n{
        //     chars.pop();
        // }

        i as i32
    }
}
