// get divisor 2 & 5, 5 is rare, so only count 5
impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        if n < 5{
            return 0;
        }
        n/5 + Self::trailing_zeroes(n/5)
    }
}