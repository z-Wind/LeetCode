// https://leetcode.com/problems/repeated-substring-pattern/discuss/94334/Easy-python-solution-with-explaination

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let ss = s.repeat(2);
        ss[1..ss.len()-1].contains(&s)
    }
}