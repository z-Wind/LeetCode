// https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/99453/Python-Simple-Explanation

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_unstable_by_key(|s| -(s.len() as i32));
        // println!("{:?}", strs);
        for (i, word1) in strs.iter().enumerate() {
            if strs
                .iter()
                .enumerate()
                .all(|(j, word2)| i == j || !is_subseq(word1, word2))
            {
                return word1.len() as i32;
            }
        }
        return -1;
    }
}

fn is_subseq(w1: &str, w2: &str) -> bool {
    // True iff word1 is a subsequence of word2.
    let mut i = 0;
    for c in w2.chars() {
        if i < w1.len() && w1[i..i + 1] == c.to_string() {
            i += 1
        }
    }
    i == w1.len()
}
