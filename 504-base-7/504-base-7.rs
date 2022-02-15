impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }
        
        let mut s = String::new();
        let mut n = num.abs();
        while n != 0 {
            s.push_str(&(n % 7).to_string());
            n /= 7;
        }
        if num.is_negative() {
            s.push('-');
        }
        
        s.chars().rev().collect()
    }
}