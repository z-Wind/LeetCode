// 2^1234 = 2^1000 * 2^200 * 2^30 * 3^4
// a/b = q...r
// xy mod b => (bq+r)(bq'+r') = b^2qq' + bqr' + bq'r + rr' = b(bqq' + qr' + q'r) + rr'

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut n = 0;
        let mut r = a%1337;
        let mut ans = 1;
        for (i,num) in b.into_iter().rev().enumerate(){
            if i > 0{
                // r = r^10 mod 1337
                let mut temp = 1;
                for _ in 0..10{
                    temp = (temp * r) % 1337;
                }
                r = temp;
            }
            for _ in 0..num{
                ans = (ans * r) % 1337;
            }
        }
        ans
    }
}