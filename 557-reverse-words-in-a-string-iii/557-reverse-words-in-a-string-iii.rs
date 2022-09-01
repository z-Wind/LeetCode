impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = String::new();
        for sub in s.split_ascii_whitespace() {
            ans.push_str(&sub.chars().rev().collect::<String>());
            ans.push(' ');
        }
        ans.pop();
        
        ans
    }
}