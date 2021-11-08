// https://leetcode.com/problems/count-numbers-with-unique-digits/discuss/83041/JAVA-DP-O(1)-solution.
// Let f(n) = count of number with unique digits of length n.
// f(1) = 10. (0, 1, 2, 3, ...., 9)
// f(2) = 9 * 9
// f(3) = f(2) * 8 = 9 * 9 * 8
// f(4) = f(3) * 7 = 9 * 9 * 8 * 7
// ...

// f(10) = 9 * 9 * 8 * 7 * 6 * ... * 1
// f(11) = 0 = f(12) = f(13)....

impl Solution {
    pub fn count_numbers_with_unique_digits(mut n: i32) -> i32 {
        if n == 0 {
            return 1;   
        }
        
        let mut res = 10;
        let mut uniqueDigits = 9;
        let mut availableNumber = 9;
        while n > 1 && availableNumber > 0 {
            uniqueDigits = uniqueDigits * availableNumber;
            res += uniqueDigits;
            availableNumber-=1;
            n -= 1;
        }
        return res;
    }
}