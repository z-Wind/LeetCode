impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = clear(s1, s2.as_bytes());
        
        let s1_len = s1.len();
        let n1 = n1 as usize;
        let s2_len = s2.len();
        let n2 = n2 as usize;
        
        let mut s1_iter = s1.bytes().cycle().take(s1_len * n1);
        let s2 = s2.as_bytes();
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        let mut dp = Vec::new();
        dp.push(0);
        while i < n1{
            for c in s1.bytes(){
                if s2[k] == c {
                    k += 1;
                }
                if k == s2_len {
                    k = 0;
                    j += 1;
                }
            }
            i += 1;
            dp.push(j);
            if k == 0 && j % n2 == 0{
                break;
            }
        }
        
        let n = n1 / i;
        let mut ans = n * (j / n2);
        let idx = n1 - n * i;
        // println!("i:{}, j:{}, idx:{} {:?}", i, j, idx, dp);
        (ans + dp[idx] / n2) as i32
    }
}

fn clear(s1:String, s2:&[u8]) -> String{
    let mut count = vec![0;26];
    for c in s2{
        let idx = (c - b'a') as usize;
        count[idx] += 1;
    }
    
    s1.chars().fold(String::new(), |mut s, c| {
        if count[(c as u8 - b'a') as usize] > 0{
            s.push(c);
        }
        s
    })
    
}
