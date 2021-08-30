use std::collections::HashMap;
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp:HashMap<(usize,usize),bool> = HashMap::new();
        is_interleave(s1,s2,s3,0,0,&mut dp)
    }
}

fn is_interleave(s1: &[u8], s2: &[u8], s3: &[u8], i:usize, j:usize, dp:&mut HashMap<(usize,usize),bool>) -> bool{
    if dp.contains_key(&(i,j)){
        return *dp.get(&(i,j)).unwrap();
    }
    if s1.len() + s2.len() != s3.len(){
        return false;
    } else if i == s1.len() && j == s2.len(){
        return true;
    }
    
    let mut check = false;
    if i<s1.len() && s3[i+j] == s1[i]{
        check = check || is_interleave(s1, s2, s3, i+1, j, dp);
    }
    if j<s2.len() && s3[i+j] == s2[j]{
        check = check || is_interleave(s1, s2, s3, i, j+1, dp);
    }
    
    dp.insert((i,j),check);
    check
}