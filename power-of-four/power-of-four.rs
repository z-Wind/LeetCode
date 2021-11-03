impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let zeros = n.trailing_zeros();
        let ones = n.count_ones();
        // println!("{}:{}", n, zeros);
        ones == 1 && zeros & 1 == 0 // zeros % 2 == 0
    }
}