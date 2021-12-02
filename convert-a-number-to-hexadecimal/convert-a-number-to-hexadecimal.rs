use std::char;
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0{
            return String::from("0");
        }
        let mut num = num as u32;
        let mut s:Vec<char> = Vec::new();
        while num > 0{
            s.push(char::from_digit(num & 15, 16).unwrap());
            num>>=4;
        }
        s.into_iter().rev().collect()
    }
}