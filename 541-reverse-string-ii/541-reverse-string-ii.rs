impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();

        let mut ans = Vec::with_capacity(s.len());
        for (i, c) in s.chunks(k).enumerate() {
            if i % 2 == 0 {
                ans.append(&mut c.iter().rev().collect());
            } else {
                ans.append(&mut c.iter().collect());
            }
        }

        ans.into_iter().collect()
    }
}
