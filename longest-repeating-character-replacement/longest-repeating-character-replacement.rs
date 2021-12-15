impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        for i in 0..s.len(){
            let mut change = k;
            let mut repeat = 1;
            // println!("c:{}", s[i] as char);
            for j in i+1..s.len(){
                if s[i] != s[j] {
                    change-=1;
                }  
                if change < 0{
                    break;
                }
                repeat += 1;
            }
            for j in (0..i).rev(){
                if s[i] != s[j] {
                    change-=1;
                }  
                if change < 0{
                    break;
                }
                repeat += 1;
            }
            
            ans = ans.max(repeat);
        }
        ans
    }
}