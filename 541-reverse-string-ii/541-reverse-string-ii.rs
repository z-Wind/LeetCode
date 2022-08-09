// https://leetcode.com/problems/reverse-string-ii/discuss/848141/Rust-cheapest-and-best

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .enumerate()
            .map(|(i, chunk)| {
                if i % 2 == 0 {
                    chunk.iter().rev().collect::<String>()
                } else {
                    chunk.iter().collect()
                }
            })
            .collect()
    }
}
