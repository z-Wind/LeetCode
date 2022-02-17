impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        
        let max = (num as f64).sqrt() as i32;
        let mut div = 2;
        let mut sum = 1;
        while div <= max {
            if num % div == 0 {
                sum += div;
                sum += num / div;
            }

            div += 1;
        }

        sum == num
    }
}
