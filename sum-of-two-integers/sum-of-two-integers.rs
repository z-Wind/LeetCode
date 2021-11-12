// https://leetcode.com/problems/sum-of-two-integers/discuss/132479/Simple-explanation-on-how-to-arrive-at-the-solution

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut carry = a & b;
        let mut sum = a ^ b;
        while carry !=0 {
            let carry_shift = carry<<1;
            carry = sum & carry_shift;
            sum ^= carry_shift;
        }
        sum
    }
}