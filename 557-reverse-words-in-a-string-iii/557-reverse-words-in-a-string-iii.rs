// https://leetcode.com/problems/reverse-words-in-a-string-iii/discuss/1640310/Rust-0ms-2.2MB
// reverse the entire string: abc def ghi => ihg fed cba
// reverse the order of the words: cba fed ihg

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.chars()
            .rev()
            .collect::<String>()
            .split_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}
