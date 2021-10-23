impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // 1162261467 is 3^19,  3^20 is bigger than i32::MAX
        n > 0 && 1162261467 % n == 0
    }
}