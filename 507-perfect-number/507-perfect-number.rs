// https://leetcode.com/problems/perfect-number/discuss/98594/Simple-Java-Solution/102848

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        
        let mut sum = 1;
        let max = (num as f64).sqrt() as i32;
        for div in 2..=max {
            if num % div == 0 {
                sum += div;
                sum += num / div;
            }
        }

        sum == num
    }
}
