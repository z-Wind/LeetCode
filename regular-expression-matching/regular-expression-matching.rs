fn is_match(mut s: Vec<char>, mut p: Vec<char>) -> bool{
    //println!("s:{:?}\np:{:?}",s,p);
    if p.len() == 0{
        return s.len() == 0;
    }
    
    let first_match = s.len() > 0 && (s[0] == p[0] || p[0] == '.');
    if p.len() >= 2 && p[1] == '*'{
        is_match(s[..].to_vec(), p[2..].to_vec()) ||
        first_match && is_match(s[1..].to_vec(), p[..].to_vec())
    } else {
        first_match && is_match(s[1..].to_vec(), p[1..].to_vec())
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //println!("==================");
        if s == p {return true}
        
        is_match(s.chars().collect(), p.chars().collect())
    }
}