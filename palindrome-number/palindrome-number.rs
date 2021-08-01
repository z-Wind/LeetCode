impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false }        
        else if x < 10 { return true }
        
        let chars:Vec<char> = x.to_string().chars().collect();
        for i in (0..chars.len()/2){
            if chars[i] != chars[chars.len()-i-1]{
                return false;
            }
        }
        
        true
    }
}