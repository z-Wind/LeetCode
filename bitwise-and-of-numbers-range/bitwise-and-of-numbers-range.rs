impl Solution {
    pub fn range_bitwise_and(mut left: i32, right: i32) -> i32 {
        if left == 0{
            return 0;
        }
        let mut ans = left & right;
        let mut diff = right-left;
        let bits = 32 - diff.leading_zeros();
        ans >>= bits;
        ans <<= bits;
        ans
    }
}