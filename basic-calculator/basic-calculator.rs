// https://leetcode.com/problems/basic-calculator/discuss/62361/Iterative-Java-solution-with-stack/64037

impl Solution {
    pub fn calculate(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        
        let mut result:i32 = 0;
        let mut sign:i32 = 1;
        let mut num:i32 = 0;

        let mut stack:Vec<i32> = Vec::new();
        stack.push(sign);

        for c in s.as_bytes() {
            match c{
                b'0'..=b'9'=> num = num * 10 + (c - b'0') as i32,
                b'+' => {
                    result += sign * num;
                    sign = *stack.last().unwrap(); 
                    num = 0;
                },
                b'-' => {
                    result += sign * num;
                    sign = -*stack.last().unwrap(); 
                    num = 0;
                },
                b'(' => stack.push(sign),
                b')' => {stack.pop();},
                b' ' => (),
                _ => panic!(),
            }
        }

        result += sign * num;
        result
    }
}
