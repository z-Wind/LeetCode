use std::collections::HashMap;

fn is_match(mut s:&[u8], mut p: &[u8]) -> bool{
    let mut mem:HashMap<(usize,usize), bool>  = HashMap::new();
    
    //println!("s:{:?}\np:{:?}",s,p);
    if p.len() == 0{
        return s.len() == 0;
    }
    
    let mut dp = |i:usize,j:usize| -> bool {
        if let Some(&ans) = mem.get(&(i,j)){
            return ans;
        }
        
        let ans:bool;
        let s = &s[i..];
        let p = &p[j..];
        
        let first_match = s.len() > 0 && (s[0] == p[0] || p[0] == b'.');
        if p.len() >= 2 && p[1] == b'*'{
            ans = is_match(s, &p[2..]) ||
            first_match && is_match(&s[1..], &p[..]);
        } else {
            ans = first_match && is_match(&s[1..], &p[1..]);
        }
        
        mem.insert((i,j), ans);
        ans
    };
    
    dp(0,0)
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //println!("==================");
        if s == p {return true}
        
        is_match(s.as_bytes(), p.as_bytes())
    }
}