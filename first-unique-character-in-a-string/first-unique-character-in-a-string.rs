impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let s = s.as_bytes();
        let mut counts:Vec<i32> = vec![0;26];
        for c in s.iter(){
            let i = (c - b'a') as usize;
            counts[i]+=1;
        }
        for idx in 0..s.len(){
            let i = (s[idx] - b'a') as usize;
            if counts[i] == 1{
                return idx as i32;
            }
        }
        -1
    }
}