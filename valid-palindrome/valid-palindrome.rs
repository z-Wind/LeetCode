impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len()-1;
        while i < j{
            if !s[i].is_ascii_alphanumeric(){
                i+=1;
                continue
            }
            if !s[j].is_ascii_alphanumeric(){
                j-=1;
                continue
            }
            if !s[i].eq_ignore_ascii_case(&s[j]) {
                return false;
            }
            i+=1;
            j-=1;
        }
        true
    }
}