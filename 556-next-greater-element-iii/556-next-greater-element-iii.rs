// https://leetcode.com/problems/next-greater-element-iii/discuss/101824/Simple-Java-solution-(4ms)-with-explanation.
// https://leetcode.com/problems/next-greater-element-iii/discuss/1769284/rust

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut s: Vec<char> = n.to_string().chars().collect();
        let size = s.len();

        // 找出由右邊開始，違反反序的第一個數字
        let mut i = size - 2;
        while i < size && s[i] >= s[i + 1] {
            i -= 1;
        }
        if i >= size {
            return -1;
        }

        // 基於前面找的方式，右邊會是由大到小排好的狀態
        // 從右邊開始，找出第一個比它大的數字
        let mut k = size - 1;
        while k < size && s[k] <= s[i] {
            k -= 1;
        }

        // 交換兩者，並將反序改為正序，呈現最小值
        s.swap(i, k);
        s[i + 1..size].reverse();

        s.into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(-1)
    }
}
