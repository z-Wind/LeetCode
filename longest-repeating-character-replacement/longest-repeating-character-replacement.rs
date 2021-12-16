// https://leetcode.com/problems/longest-repeating-character-replacement/discuss/91271/Java-12-lines-O(n)-sliding-window-solution-with-explanation
// make a window , and if it match to expand or not to shift

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let len = s.len();
        let mut count: [usize; 26] = [0; 26];

        let mut start = 0;
        let mut maxCount = 0;
        for end in 0..len {
            let i = (s[end] - b'A') as usize;
            count[i] += 1;
            maxCount = maxCount.max(count[i]);
            // (end - start + 1) means the current longest length, if the (maxCount + k) < (the current longest length), we move whole windows right.
            // otherwise the start stays at current position, but the end moves, means next round the longest length will increase.
            if maxCount + k < end - start + 1 {
                count[(s[start] - b'A') as usize] -= 1;
                start += 1;
            }
        }
        (s.len() - start) as i32
    }
}
