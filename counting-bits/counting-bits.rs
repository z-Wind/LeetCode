impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize + 1);
        for i in 0..=n{
            ans.push(i.count_ones() as i32);
        }
        ans
    }
}