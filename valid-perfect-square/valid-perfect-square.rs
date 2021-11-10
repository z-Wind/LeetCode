impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        ((num as f64).sqrt() as i32).pow(2) == num
    }
}