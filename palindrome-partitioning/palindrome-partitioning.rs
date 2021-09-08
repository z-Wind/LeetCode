use std::collections::HashMap;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans:Vec<Vec<String>> = Vec::new();
        let mut temp:Vec<String> = Vec::new();
        let mut dp:Vec<Vec<bool>> = vec![vec![false;s.len()];s.len()];
        backtrack(&mut dp, &mut ans, &mut temp, &s, 0);
        ans
    }
}

fn backtrack(dp:&mut Vec<Vec<bool>>, ans:&mut Vec<Vec<String>>,temp: &mut Vec<String>, s:&str, start:usize){
    if start >= s.len(){
        ans.push(temp.clone());
        return;
    }
    
    for i in (start..s.len()){
        if is_palindrome(dp, &s.as_bytes(), start, i){
            temp.push(String::from(&s[start..=i]));
            backtrack(dp,ans,temp,s,i+1);
            temp.pop();
        }
    }
}

fn is_palindrome(dp:&mut Vec<Vec<bool>>, s:&[u8], start:usize, end:usize) -> bool{
    let check = s[start] == s[end] && (end-start <=2 || dp[start+1][end-1]);
    dp[start][end] = check;
    check
}