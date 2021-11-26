impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if k==1{
            return s.len() as i32;
        }
        let mut ans = 0;
        let s = s.as_bytes();
        for i in 0..s.len(){
            if ans >= s.len()-i{
                break;
            }
            let mut less = 0;
            let mut counts = vec![0;26];
            for j in i..s.len(){
                let idx = (s[j] - b'a') as usize;
                counts[idx] += 1;
                if counts[idx] == 1{
                    less += 1;
                } else if counts[idx] == k{
                    less -= 1;
                }
                // println!("{} => {}, {:?}", s[j] as char, less, counts);
                
                if less == 0{
                    ans = ans.max(j-i+1);
                }
            }
        }
        ans as i32
    }
}