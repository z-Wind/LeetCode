// https://leetcode.com/problems/longest-word-in-dictionary-through-deleting/discuss/99588/Short-Java-Solutions-Sorting-Dictionary-and-Without-Sorting

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s = s.as_bytes();
        let mut longest = Vec::new();
        for dictWord in dictionary {
            if dictWord.len() > s.len() {
                continue;
            }
            
            let dictWord = dictWord.as_bytes();
            let mut i = 0;
            for &c in s.iter() {
                if i < dictWord.len() && c == dictWord[i] {
                    i += 1;
                }
            }

            if i == dictWord.len()
                && dictWord.len() >= longest.len()
                && (dictWord.len() > longest.len() || dictWord < &longest)
            {
                longest = dictWord.to_vec();
            }
        }
        String::from_utf8(longest).unwrap()
    }
}
