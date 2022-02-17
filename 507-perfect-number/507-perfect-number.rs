// https://en.wikipedia.org/wiki/List_of_Mersenne_primes_and_perfect_numbers
// https://zh.wikipedia.org/wiki/%E5%AE%8C%E5%85%A8%E6%95%B0

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        num == 6 || num == 28 || num == 496 || num == 8128 || num == 33550336
    }
}
