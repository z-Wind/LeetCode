// https://leetcode.com/problems/power-of-four/discuss/80457/Java-1-line-(cheating-for-the-purpose-of-not-using-loops)

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        //  1: 0b000000001
        //  4: 0b000000100
        // 16: 0b000010000
        // 32: 0b001000000
        // at odd location
        // use 0x55555555 to check
        
        n > 0 && (n&(n-1)) == 0 && (n & 0x55555555) != 0
        //      check only one 1        
    }
}