impl Solution {
    pub fn is_palindrome(mut s: String) -> bool {
        s.make_ascii_lowercase();
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len()-1;
        while i < j{
            match s[i]{
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => (),
                _ => {
                    i+=1;
                    continue;
                },
            }
            match s[j]{
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => (),
                _ => {
                    j-=1;
                    continue;
                },
            }
            
            if s[i] != s[j] {
                return false;
            }
            i+=1;
            j-=1;
        }
        true
    }
}