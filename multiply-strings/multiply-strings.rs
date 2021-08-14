use std::char;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero = String::from("0");
        if num1 == zero || num2 == zero {
            return zero;
        }
        let len = num1.len() + num2.len();
        let mut res:Vec<u32> = vec![0;len];
        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                res[i + j + 1] += ((num1.as_bytes()[i] - b'0') as u32) * ((num2.as_bytes()[j] - b'0') as u32);
            }
        }
        for i in (1..len).rev() {
            res[i - 1] += res[i] / 10;
            res[i] %= 10;
        }
        let mut arr:Vec<char> = Vec::new();
        if res[0] != 0 {
            arr.push(char::from_digit(res[0], 10).unwrap());
        }
        for i in 1..len {
            arr.push(char::from_digit(res[i], 10).unwrap());
        }
        arr.into_iter().collect()
    }
}