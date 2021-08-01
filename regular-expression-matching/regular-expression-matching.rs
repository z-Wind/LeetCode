fn is_match(mut s: Vec<char>, mut p: Vec<char>) -> bool{
    //println!("s:{:?}\np:{:?}",s,p);
    if s.len() == 0 && p.len() == 0{
        return true;
    }
    else if p.len() == 0{
        return false
    }
    
    let pat = &p[0 .. std::cmp::min(2,p.len())];
    match pat{
        ['a' ..= 'z'] => s.len() == 1 && p[0] == s[0],
        ['.'] => s.len() == 1,
        ['a' ..= 'z', 'a' ..= 'z'] | ['a' ..= 'z', '.'] => s.len() > 0 && p[0] == s[0] && is_match(s[1..].to_vec(), p[1..].to_vec()),
        ['.', 'a' ..= 'z'] | ['.', '.'] => s.len() > 0 && is_match(s[1..].to_vec(), p[1..].to_vec()),
        ['a' ..= 'z', '*'] => {
            if s.len() == 0 {return p.len() == 2 || is_match(s[..].to_vec(), p[2..].to_vec())}
            else if p[0] != s[0] {return is_match(s[..].to_vec(), p[2..].to_vec())}
            
            for i in (0..s.len()+1){
                if is_match(s[i..].to_vec(), p[2..].to_vec()){
                    return true;
                }
                if i == s.len() || p[0] != s[i]{
                    break;
                }
            }
            return false;
        },
        ['.', '*'] => {
            if s.len() == 0 {return p.len() == 2 || is_match(s[..].to_vec(), p[2..].to_vec())}
            for i in (0..s.len()+1){
                if is_match(s[i..].to_vec(), p[2..].to_vec()){
                    return true;
                }
            }
            return false;
        },
        _ => {
            println!("{:?}", pat);
            println!("{}", p.into_iter().collect::<String>());
            panic!();
        },
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        //println!("==================");
        if s == p {return true}
        
        is_match(s.chars().collect(), p.chars().collect())
    }
}