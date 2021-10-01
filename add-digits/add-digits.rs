// Digital Root
impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        match num{
            0 => 0,
            x => 1+(x-1)%9,
        }
    }
}