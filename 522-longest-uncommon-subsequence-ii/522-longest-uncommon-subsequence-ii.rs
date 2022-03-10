// https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/99453/Python-Simple-Explanation

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_unstable_by_key(|s| -(s.len() as i32));
        // println!("{:?}", strs);

        strs.iter()
            .enumerate()
            .find(|(i, word1)| {
                strs.iter()
                    .enumerate()
                    .all(|(j, word2)| *i == j || !is_subseq(word1, word2))
            })
            .map(|(_, s)| s.len() as i32)
            .unwrap_or(-1)
    }
}

fn is_subseq(w1: &str, w2: &str) -> bool {
    // True iff word1 is a subsequence of word2.
    let mut w1 = w1.chars();
    let mut w2 = w2.chars();

    'outer: while let Some(c1) = w1.next() {
        while let Some(c2) = w2.next() {
            if c1 == c2 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}
