// 2^1234 = 2^1000 * 2^200 * 2^30 * 3^4
// a/b = q...r
// xy mod b => (bq+r)(bq'+r') = b^2qq' + bqr' + bq'r + rr' = b(bqq' + qr' + q'r) + rr'

impl Solution {
    pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
        let mut r = a%1337;
        let mut ans = power_mod(r, b.pop().unwrap());
        while let Some(n) = b.pop(){
            // r = r^10 mod 1337
            r = power_mod(r, 10);
            ans = (ans * power_mod(r, n)) % 1337;
        }
        ans
    }
}

fn power_mod(x:i32, n:i32) -> i32{
    let mut ans = 1;
    for _ in 0..n{
        ans = (ans * x) % 1337;
    }
    ans
}