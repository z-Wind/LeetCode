impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let upper = word.chars().fold(0, |count, c| {
            if c.is_ascii_uppercase() {
                count + 1
            } else {
                count
            }
        });

        upper == word.len()
            || upper == 0
            || (upper == 1 && word.chars().next().unwrap().is_ascii_uppercase())
    }
}
