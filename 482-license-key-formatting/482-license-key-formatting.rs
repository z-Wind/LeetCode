impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut res = Vec::with_capacity(s.len());
        let mut count = 0;
        for c in s.chars().rev(){
            match c {
                '-' => continue,
                _ => {
                    res.push(c.to_ascii_uppercase());
                    count += 1;
                    if count == k{
                        res.push('-');
                        count = 0;
                    }
                }
            }
        }
        // println!("{:?}", res);
        if res.is_empty(){
            return String::new();
        }
        
        if res[res.len()-1] == '-'{
            res[..res.len()-1].into_iter().rev().collect()    
        } else {
            res.into_iter().rev().collect()
        }
    }
}