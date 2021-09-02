use std::collections::HashMap;
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut count_s = [0;52];
        for c in s.bytes(){
            if c >= 97{
                count_s[(c-b'a') as usize] += 1;
            } else {
                count_s[26+(c-b'A') as usize] += 1;
            }
        }
        let mut count_t = [0;52];
        for c in t.bytes(){
            if c >= 97{
                count_t[(c-b'a') as usize] += 1;
            } else {
                count_t[26+(c-b'A') as usize] += 1;
            }
        }
        for i in (0..52){
            if count_s[i] < count_t[i]{
                return 0;
            }
        }
        
        let mut dp:HashMap<(usize,usize),i32> = HashMap::new();
        num_distinct(&mut dp, s.as_bytes(), 0, t.as_bytes(), 0)
    }
}

fn num_distinct(dp:&mut HashMap<(usize,usize),i32>, s: &[u8], i:usize, t: &[u8], j:usize) -> i32{
    if dp.contains_key(&(i,j)){
        return *dp.get(&(i,j)).unwrap();
    } else if j == t.len(){
        return 1;
    } else if i == s.len(){
        return 0;
    }
    
    let mut count = 0;
    for k in (i..s.len()){
        if s[k] == t[j]{
            count += num_distinct(dp, &s,k+1, &t,j+1);
        }
    }
    dp.insert((i,j),count);
    count
}