impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev()
            .fold(String::new(), |acc, s| acc + s + " ")
            .trim_end()
            .to_string()
    }
}