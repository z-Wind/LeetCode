// https://leetcode.com/problems/binary-watch/discuss/88458/Simple-Python%2BJava

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut ans = Vec::new();
        for h in 0..12_i32 {
            for m in 0..60_i32 {
                if (h * 64 + m).count_ones() == turned_on {
                    ans.push(format!("{}:{:02}", h, m));
                }
            }
        }
        ans
    }
}
