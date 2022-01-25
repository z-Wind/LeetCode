impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let mut res = Vec::with_capacity(s.len());
        for c in s.chars().rev(){
            match c {
                '-' => continue,
                _ => {
                    if res.len() % (k+1) == k {
                        res.push('-');
                    }
                    res.push(c.to_ascii_uppercase());
                }
            }
        }
        // println!("{:?}", res);
        
        res.into_iter().rev().collect()
    }
}