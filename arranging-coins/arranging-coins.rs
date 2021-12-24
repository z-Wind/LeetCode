// (1+k)*k/2 >= n >= k*(k-1)/2

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {        
        let n = n as i64;
        let mut k: i64 = 1;
        
        while (1 + k) * k / 2 <= n {
            k += 1;
        }
        k as i32 - 1
    }
}
