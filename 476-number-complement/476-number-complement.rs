impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let n = 32 - num.leading_zeros();
        let flip = (1 << n) - 1;
        num ^ flip
    }
}