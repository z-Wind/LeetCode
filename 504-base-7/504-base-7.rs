impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        let mut s = String::new();
        let is_neg = num.is_negative();
        let mut num = if is_neg {
            -num as u32
        } else {
            num as u32
        };
        while num != 0 {
            s.push(char::from_digit(num % 7, 10).unwrap());
            num /= 7;
        }
        if is_neg {
            s.push('-');
        }
        
        s.chars().rev().collect()
    }
}