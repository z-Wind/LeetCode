use std::collections::HashMap;
impl Solution {
    pub fn word_break(s: String, mut word_dict: Vec<String>) -> bool {      
        word_dict.sort_unstable_by_key(|s| s.len());   
        word_dict.reverse();
        let mut dp:HashMap<usize,bool> = HashMap::new();
        // println!("word_dict: {:?}", new_word_dict);
        word_break(&mut dp, &s, 0, &word_dict)
    }
}

fn word_break(dp:&mut HashMap<usize,bool>, s: &str, i:usize, word_dict: &Vec<String>) -> bool{
    if i == s.len(){
        return true;
    }
    if dp.contains_key(&i){
        return *dp.get(&i).unwrap();
    }
    let mut check = false;
    // println!("in: {}", s[i..]);
    for word in word_dict{
        // println!("test: {}", word);
        if s[i..].starts_with(word) {
            if (word_break(dp,s,i+word.len(), word_dict)) {
                check = true;
                break;
            }
        }
    }
    dp.insert(i,check);
    check
}