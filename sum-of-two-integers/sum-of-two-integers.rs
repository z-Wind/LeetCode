impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut ans = 0;
        let mut carry = 0;
        let mut sum = 0;
        for i in 0..32{
            let bit_a = a & 1;
            let bit_b = b & 1;
            let carry_t = (bit_a & bit_b) | (carry & (bit_a ^ bit_b));
            let sum_t = carry ^ bit_a ^ bit_b;
            carry = carry_t;
            sum = sum_t;
            ans |= sum << i;
            a >>= 1;
            b >>= 1;
        }
        ans
    }
}