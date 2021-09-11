impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut ans:Vec<String> = Vec::new();
        let mut temp:Vec<String> = Vec::new();
        backtrack(&mut ans, &mut temp, &s, 0, &word_dict);
        ans   
    }
}

fn backtrack(ans:&mut Vec<String>, temp:&mut Vec<String>, s: &str, i:usize, word_dict: &Vec<String>){
    if i == s.len(){
        ans.push(temp.join(" "));
        return;
    }
    for word in word_dict{
        if s[i..].starts_with(word){
            temp.push(word.clone());
            backtrack(ans,temp,s,i+word.len(),word_dict);
            temp.pop();
        }
    }
}