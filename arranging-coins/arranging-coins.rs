// (1+k)*k/2 >= n
// (k^2)/2 + k/2 - n = 0
// k = -1/2 + sqrt(1/4 + 2n)

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {        
        let k = -0.5 + (0.25 + n as f64 * 2.0).sqrt();
        
        k as i32
    }
}
