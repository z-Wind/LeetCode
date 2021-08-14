use std::collections::HashMap;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut m = Match::new(s.as_bytes(), p.as_bytes());
        m.is_match()
    }
}

struct Match<'a> {
    mem:HashMap<(usize,usize), bool>,
    s: &'a [u8],
    p: &'a [u8],
}

impl<'a> Match<'a> {
    fn new(s:&'a [u8], p: &'a [u8]) -> Self{
        Match{
            mem: HashMap::new(),
            s,
            p,
        }
    }
    fn is_match(&mut self) -> bool{
        self.dp(0,0)
    }
    
    fn dp(&mut self, i:usize,j:usize) -> bool {
        if let Some(&ans) = self.mem.get(&(i,j)){
            return ans;
        }

        let ans:bool;
        let s = &self.s[i..];
        let p = &self.p[j..];
        
        if p.len() == 0{
            ans = s.len() == 0;
            self.mem.insert((i,j), ans);
            return ans;
        }

        let first_match = s.len() > 0 && (s[0] == p[0] || p[0] == b'*' || p[0] == b'?');
        if p.len() >= 1 && p[0] == b'*'{
            ans = self.dp(i,j+1) ||
            first_match && self.dp(i+1,j);
        } else {
            ans = first_match && self.dp(i+1,j+1);
        }

        self.mem.insert((i,j), ans);
        ans
    }
}