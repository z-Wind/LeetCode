impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let (lower, upper) = word.chars().fold((0, 0), |mut count, c| {
            if c.is_ascii_lowercase() {
                count.0 += 1;
            } else {
                count.1 += 1;
            }

            count
        });

        upper == word.len()
            || lower == word.len()
            || (upper == 1 && word.chars().next().unwrap().is_ascii_uppercase())
    }
}
