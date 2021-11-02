impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut ans:Vec<Vec<i32>> = Vec::new();
        let mut words_rev:Vec<String> = Vec::new();
        for i in 0..words.len(){
            words_rev.push(words[i].chars().rev().collect::<String>())
        }
        palindrome_pairs(&mut ans, &words, &words_rev, 0);
        ans
    }
}

fn palindrome_pairs(ans:&mut Vec<Vec<i32>>, words: &Vec<String>, words_rev: &Vec<String>, start:usize){
    if start == words.len(){
        return;
    }
    for i in start+1..words.len(){
        let (a,b) = if words[start].len() >= words[i].len(){
            (start, i)
        } else {
            (i, start)
        };
        
        match words[a].strip_prefix(&words_rev[b]){
            None => (),
            Some(s) => {
                if is_palindrome(s.as_bytes()){
                    ans.push(vec![a as i32, b as i32]);
                }
            },
        }
        match words[a].strip_suffix(&words_rev[b]){
            None => (),
            Some(s) => {
                if is_palindrome(s.as_bytes()){
                    ans.push(vec![b as i32, a as i32])
                }
            },
        }
    }
    
    palindrome_pairs(ans, words, words_rev, start+1);
}

fn is_palindrome(s:&[u8]) -> bool{
    if s.len() == 0{
        return true;
    }
    let mut i = 0;
    let mut j = s.len()-1;
    while i < j{
        if s[i] != s[j]{
            return false;
        }
        i+=1;
        j-=1;
    }
    return true;
}