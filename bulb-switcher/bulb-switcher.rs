// https://leetcode.com/problems/bulb-switcher/discuss/77104/Math-solution..
// Divisors come in pairs, like i=12 has divisors 1 and 12, 2 and 6, and 3 and 4. Except when i is a square, like 36 has divisors 1 and 36, 2 and 18, 3 and 12, 4 and 9, and double divisor 6. So bulb i ends up on if and only if i is a square.
// For example, given a 36. sqrt(36) = 6. It means we have 1^2, 2^2, 3^2, 4^2, 5^2, 6^2 between 0~36. So the number of square numbers is 6.

impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        (n as f64).sqrt() as i32
    }
}