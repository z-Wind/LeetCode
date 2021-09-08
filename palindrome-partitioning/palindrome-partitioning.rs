use std::collections::HashMap;
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut dp:HashMap<usize, Vec<Vec<String>>> = HashMap::new();
        backward(&mut dp, &s, 0)
    }
}

fn backward(dp:&mut HashMap<usize,Vec<Vec<String>>>, s:&str, start:usize) -> Vec<Vec<String>>{
    let mut ans:Vec<Vec<String>> = Vec::new();
    if start >= s.len(){
        return ans;
    }
    if dp.contains_key(&start){
        return dp.get(&start).unwrap().to_vec();
    }
    
    for i in (start..s.len()){
        if is_palindrome(&s.as_bytes()[start..=i]){
            let mut words = backward(dp,s,i+1);
            if words.is_empty(){
                let mut temp:Vec<String> = Vec::new();
                temp.push(String::from(&s[start..=i]));
                ans.push(temp);
                continue;
            }
            for mut word in words{
                let mut temp:Vec<String> = Vec::new();
                temp.push(String::from(&s[start..=i]));
                temp.append(&mut word);
                ans.push(temp);
            };
        }
    }
    
    dp.insert(start, ans.clone());
    // println!("{}: {:?}", &s[start..],ans);
    ans
}

fn is_palindrome(s:&[u8]) -> bool{
    for i in (0..s.len()/2){
        if s[i] != s[s.len()-1-i]{
            return false;
        }
    }
    true
}