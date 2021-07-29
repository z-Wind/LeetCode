use std::cmp::Ordering;

fn is_palindrome(s: &str) -> bool {
    if s.len() == 0{
        return false;
    } else if s.len() == 1{
        return true;
    }
    
    let s1 = s.chars();
    let s2 = s.chars().rev();
    
    match s1.cmp(s2) {
        Ordering::Equal => return true,
        _ => return false,
    }
}

fn longest_palindrome(s: &str) -> &str {
    if s.len() < 2{
        return s
    }
    if is_palindrome(s) { return s }
    
    let mut s1 = "";
    for i in (1..s.len()){
        if is_palindrome(&s[..s.len()-i]){
            s1 = &s[..s.len()-i];
            break;
        }
    }
    let mut s2 = "";
    if s1.len() < s.len()-1{
        s2 = longest_palindrome(&s[1..s.len()]);
    }
    
    if s1.len() >= s2.len(){
        return s1;
    } else {
        return s2;
    }
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        String::from(longest_palindrome(&s))
    }
}