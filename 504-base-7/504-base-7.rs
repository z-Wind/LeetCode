impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        
        let mut sign = if num.is_negative() {
            num *= -1;
            String::from("-")
        } else {
            String::new()
        };
        
        let mut s = String::new();
        while num != 0 {
            s = (num % 7).to_string() + &s;
            num /= 7;
        }
        
        sign + &s
    }
}