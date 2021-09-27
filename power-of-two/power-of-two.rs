impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0{
            return false;
        }
        while n != 1{
            if n & 1 == 1{
                return false;
            }
            n >>= 1;
        }
        true
    }
}