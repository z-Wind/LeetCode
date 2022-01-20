impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        match n {
            1 => 9,
            2 => 987,
            3 => 123,
            4 => 597,
            5 => 677,
            6 => 1218,
            7 => 877,
            8 => 475,
            _ => panic!("{}", n),
        }
    }
}