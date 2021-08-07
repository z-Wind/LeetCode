use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        find_substring(&s, words)
    }
}

fn find_substring(s: &str, words: Vec<String>) -> Vec<i32> {    
    let mut ans:Vec<i32> = vec![];
    
    let word_len = words[0].len();
    let words_len = word_len * words.len();
    if words_len > s.len(){
        return ans;
    }
    
    let mut wordm = HashMap::new();
    for word in words {
        *wordm.entry(word).or_insert(0 as usize)+=1;
    }
    //println!("map:{:#?}", wordm);
    
    let mut i = 0;
    while i+words_len<=s.len(){
        let mut words = wordm.clone();
        if check_continue_substring(&s[i..i+words_len], word_len, &mut words){
            ans.push(i as i32);
        }
        i+=1;
    }

    ans
}

fn check_continue_substring(s: &str, word_len:usize, words: &mut HashMap<String, usize>) -> bool{  
    if words.is_empty(){
        return true;
    }
    
    let word = &s[..word_len];
    match words.get_mut(word){
        None => return false,
        Some(num) =>{
            *num -= 1;
            if *num == 0{
                words.remove(word);
            }
            return check_continue_substring(&s[word_len..], word_len, words);
        },
    }
}