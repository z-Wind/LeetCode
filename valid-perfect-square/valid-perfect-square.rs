impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // 46340^2 < 2147483647 < 46341^2
        for i in 1..=46340{
            let x = i*i;
            if x == num{
                return true;
            } else if x > num{
                break;
            }
        }
        false
    }
}