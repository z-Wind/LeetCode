use std::collections::HashSet;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut set1: HashSet<char> = "qwertyuiop".chars().collect();
        let mut set2: HashSet<char> = "asdfghjkl".chars().collect();
        let mut set3: HashSet<char> = "zxcvbnm".chars().collect();

        let mut ans = Vec::new();
        'outer: for word in words {
            let c = word.chars().next().unwrap().to_ascii_lowercase();
            if set1.contains(&c) {
                for c in word.chars() {
                    if !set1.contains(&c.to_ascii_lowercase()) {
                        continue 'outer;
                    }
                }
            } else if set2.contains(&c) {
                for c in word.chars() {
                    if !set2.contains(&c.to_ascii_lowercase()) {
                        continue 'outer;
                    }
                }
            } else if set3.contains(&c) {
                for c in word.chars() {
                    if !set3.contains(&c.to_ascii_lowercase()) {
                        continue 'outer;
                    }
                }
            } else {
                continue;
            }

            ans.push(word);
        }
        ans
    }
}