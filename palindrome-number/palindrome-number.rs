impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) { return false }        
        else if x < 10 { return true }
        
        let mut rev = 0;
        while x > rev{
            rev = rev * 10 + x%10;
            x /= 10;
        
            if rev == x || rev/10 == x{
                return true;
            }
        }
        
        false
    }
}