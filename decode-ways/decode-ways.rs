impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp:Vec<i32> = vec![-1;s.len()];
        backtrack(s.as_bytes(), 0, &mut dp)
    }
}


fn backtrack(s:&[u8], pos:usize, dp:&mut Vec<i32>) -> i32{
    if pos == s.len(){
        return 1;
    }
    //println!("{}: {:?}",pos, dp);
    if dp[pos] != -1{
        return dp[pos];
    }
    dp[pos] = 0;
    
    match s[pos]{
        b'1'..=b'9' => {
            dp[pos] += backtrack(s,pos+1,dp);
        },
        _ => (),
    }
    if pos == s.len() - 1{
        return dp[pos];
    }
    match (s[pos],s[pos+1]){
        (b'1', b'0'..=b'9') | (b'2', b'0'..=b'6') => {
           dp[pos] += backtrack(s,pos+2,dp);
        },
        _ => (),
    }
    return dp[pos];
}