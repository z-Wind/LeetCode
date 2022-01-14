// https://leetcode.com/problems/concatenated-words/discuss/95652/Java-DP-Solution

use std::collections::HashSet;
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|s| s.len());

        let mut result = Vec::new();
        let mut preWords = HashSet::new();
        for word in words {
            if canForm(&word, &preWords) {
                result.push(word.clone());
            }
            preWords.insert(word);
        }

        result
    }
}

fn canForm(word: &str, dict: &HashSet<String>) -> bool {
    if dict.is_empty() {
        return false;
    }

    let n = word.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && dict.contains(&word[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    return dp[n];
}