// https://leetcode.com/problems/single-number-ii/discuss/43295/Detailed-explanation-and-generalization-of-the-bitwise-operation-method-for-single-numbers
//
// Given an array of integers, every element appears k (k > 1) times except for one, which appears p times (p >= 1, p % k != 0). Find that single one.
//
// 2^m = k => m=log2(k)
// use m i32 as counter
// cm = cm ^ (cm-1 & .. c1 & num)
// c3 = c3 ^ (c2 & c1 & num)
// c2 = c2 ^ (c1 & num)
// c1 = c1 ^ num
//
// use a mask to clear counter when counter meets k
// mask = ~(b1 & b2 & b3 & ... & bm) to meet k
// ex.
// k = 3 0b11 => mask = ~(c1 & c2)
// k = 5 0b101 => mask = ~(c1 & ~c2 & c3)
// because other numbers who repeat k are clear, the final result is c0 | c1 | c2..
// in other word, it means when the p_bit_k = 1, ck is result
//
impl Solution {
    // 2^m = 3 => m=2
    // k   = 3   => 0b11
    // p   = 1   => 0b01 
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut c1 = 0; 
        let mut c2 = 0;
        let mut mask = 0;
        for num in nums{
            c2 ^= c1 & num;
            c1 ^= num;
            mask = !(c1 & c2);
            c2 &= mask;
            c1 &= mask;
        }
        c1
    }
}