fn is_match(mut s:&[u8], mut p: &[u8]) -> bool{
    //println!("s:{:?}\np:{:?}",s,p);
    if p.len() == 0{
        return s.len() == 0;
    }
    
    let first_match = s.len() > 0 && (s[0] == p[0] || p[0] == b'.');
    if p.len() >= 2 && p[1] == b'*'{
        is_match(s, &p[2..]) ||
        first_match && is_match(&s[1..], &p[..])
    } else {
        first_match && is_match(&s[1..], &p[1..])
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //println!("==================");
        if s == p {return true}
        
        is_match(s.as_bytes(), p.as_bytes())
    }
}