impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n % 2 == 0 {
            Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2)
        } else {
            if n > 0 {
                x * Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2)
            } else {
                (Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2)) / x
            }
        }
    }
}