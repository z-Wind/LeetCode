// https://yeefun.github.io/kmp-algorithm-for-beginners/
// KMP(Knuth–Morris–Pratt) algorithm

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let s_rev:String = s.chars().rev().collect();
        let mut s_test = format!("{}#{}",s,s_rev);
        let s_b = s_test.as_bytes();
        let mut f = vec![0;s_test.len()];
        for i in (1..f.len()){
            let mut t = f[i-1];
            if s_b[i] == s_b[t]{
                t+=1;
            } else {
                while t != 0 {
                    t = f[t-1];
                    if s_b[i] == s_b[t]{
                        t+=1;
                        break;
                    }
                }
            }
            f[i] = t;
        }
        // println!("{:?}",f);
        format!("{}{}",&s_rev[..s.len()-f.last().unwrap()],s)
    }
}