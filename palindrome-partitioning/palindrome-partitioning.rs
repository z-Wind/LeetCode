use std::collections::HashMap;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans:Vec<Vec<String>> = Vec::new();
        let mut temp:Vec<String> = Vec::new();
        backtrack(&mut ans, &mut temp, &s, 0);
        ans
    }
}

fn backtrack(ans:&mut Vec<Vec<String>>,temp: &mut Vec<String>, s:&str, start:usize){
    if start >= s.len(){
        ans.push(temp.clone());
        return;
    }
    
    for i in (start..s.len()){
        if is_palindrome(&s.as_bytes()[start..=i]){
            temp.push(String::from(&s[start..=i]));
            backtrack(ans,temp,s,i+1);
            temp.pop();
        }
    }
}

fn is_palindrome(s:&[u8]) -> bool{
    for i in (0..s.len()/2){
        if s[i] != s[s.len()-1-i]{
            return false;
        }
    }
    true
}