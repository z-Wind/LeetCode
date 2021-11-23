impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let s = s.as_bytes();
        let mut counts:Vec<i32> = vec![0;26];
        for c in s.iter(){
            let i = (c - b'a') as usize;
            counts[i]+=1;
        }
        for (idx,c) in s.iter().enumerate(){
            let i = (c - b'a') as usize;
            if counts[i] == 1{
                return idx as i32;
            }
        }
        -1
    }
}
