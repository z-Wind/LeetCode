use std::collections::HashMap;
use std::cmp::min;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ans:Vec<i32> = vec![];
    
        let word_len = words[0].len();
        let words_len = word_len * words.len();
        if words_len > s.len(){
            return ans;
        }

        let mut words_map = HashMap::new();
        for word in words {
            *words_map.entry(word).or_insert(0 as usize)+=1;
        }
        //println!("map:{:#?}", words_map);
        
        // seperated by word_len for shift window
        let mut i = 0;
        for i in (0..min(word_len, s.len()-words_len+1)){
            ans.append(&mut find(&s, i, word_len, words_len, &words_map));
        }

        ans
    }
}


fn find(s: &str, start:usize, word_len:usize, words_len:usize, words: &HashMap<String, usize>) -> Vec<i32>{  
    let mut ans:Vec<i32> = vec![];
    let mut words_s = HashMap::new();
    
    let mut i = start;
    let mut word_start = start;
    //println!("s.len:{}\nword_len:{}\nwords_len:{}", s.len(), word_len, words_len);
    while i+words_len<=s.len(){
        let word = &s[word_start..word_start+word_len];
        *words_s.entry(word).or_insert(0 as usize)+=1;
        word_start+=word_len;
        //println!("i:{}, word_start:{}, {}, {:?}", i,word_start,word,words_s);
        
        match words.get(word){
            None => {
                i = word_start;
                words_s.clear();
                continue;
            },
            Some(num) => {
                while *words_s.entry(word).or_default() > *num{
                    *words_s.entry(&s[i..i+word_len]).or_insert(0 as usize)-=1;
                    i+=word_len;
                    //println!("in while i:{}, word_start:{}, {}, {:?}", i,word_start,word,words_s);
                }       
            },
        }
        
        if word_start-i == words_len{
            //println!("get: {}", i);
            ans.push(i as i32);
            let word = &s[i..i+word_len];
            *words_s.entry(word).or_insert(0 as usize)-=1;
            i += word_len;
        }
    }
    
    ans
}