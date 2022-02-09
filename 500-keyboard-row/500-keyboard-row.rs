use std::collections::HashSet;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows:Vec<HashSet<char>> = vec![
            "qwertyuiop".chars().collect(),
            "asdfghjkl".chars().collect(),
            "zxcvbnm".chars().collect(),
        ];

        words
            .into_iter()
            .filter(|word| {
                rows.iter()
                    .any(|set| word.to_ascii_lowercase().chars().all(|c| set.contains(&c)))
            })
            .collect()
    }
}
