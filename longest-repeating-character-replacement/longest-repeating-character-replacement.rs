impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if k == n as i32 - 1{
            return n as i32;
        }
        let mut lines:[Vec<(i32,i32)>;26] = Default::default();
        
        let mut start = 0;
        let mut c = s[0];
        for i in 1..n{
            if c != s[i]{
                let idx = (c-b'A') as usize;
                lines[idx].push((start as i32, (i-1) as i32));
                start = i;
                c = s[i]
            }
        }
        lines[(c-b'A') as usize].push((start as i32, (n-1) as i32));
        
        // println!("{:?}", lines);
        
        let mut ans:i32 = 0;
        for c in 0..26{
            if lines[c].len() == 0{
                continue;
            }
            // println!("{}", c);
            let mut ignore:i32 = k;
            let mut first:usize = 0;
            let mut end:usize = 1;
            let mut rst = (lines[c][first].1 - lines[c][first].0 + 1 + ignore);
            while end < lines[c].len(){
                // println!("1st:{}, k:{}, {:?} -> ... -> {:?} -> {:?}", first, ignore, lines[c][first], lines[c][end-1], lines[c][end]);
                if ignore >= (lines[c][end].0 - lines[c][end-1].1 - 1) {
                    ignore -= (lines[c][end].0 - lines[c][end-1].1 - 1);
                    rst = rst.max(lines[c][end].1 - lines[c][first].0 + 1 + ignore);
                    end += 1;
                } else {
                    ignore += (lines[c][first+1].0 - lines[c][first].1 - 1);
                    first += 1;
                }
                // println!("rst:{}", rst);
            }
            // println!("1st:{}, k:{}", first, ignore);
            if ignore >= 0{
                rst = rst.max(lines[c][end-1].1 - lines[c][first].0 + 1 + ignore);
            }
            ans = ans.max(rst);
            if ans >= n as i32{
                break;
            }
        }
        
        ans.min(n as i32)
    }
}