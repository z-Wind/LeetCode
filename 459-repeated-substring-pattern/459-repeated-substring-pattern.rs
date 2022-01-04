impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        let mut len = 1;
        while len <= n>>1{
            if n % len == 0 && check(&s, &s[..len]){
                return true;
            }
            len += 1;
        }
        false
    }
}

fn check(s: &str, sub: &str) -> bool {
    // println!("{},{}", s, sub);
    if s.starts_with(sub){
        if s.len() > sub.len(){
            return check(&s[sub.len()..], sub);
        }
        return true;
    }
    false
}