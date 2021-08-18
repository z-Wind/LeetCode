use std::char;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry:i32 = 0;
        for digit in digits.iter_mut().rev(){
            match digit{
                digit @ 0..=8 => {
                    *digit += 1;
                    carry = 0;
                    break;
                }
                digit @ 9 => {
                    *digit = 0;
                    carry = 1;
                },
                _ => panic!(),
            }
        }        
        if carry == 1{
            digits.insert(0, 1);
        }
        digits
    }
}