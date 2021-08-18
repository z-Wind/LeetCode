use std::char;
use std::cmp::max;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result:Vec<char> = Vec::with_capacity(max(a.len(),b.len()));
        let mut iter_a = a.chars().rev();
        let mut iter_b = b.chars().rev();
        let mut carry = 0;
        loop{
            match (iter_a.next(),iter_b.next()){
                (Some(x), Some(y)) | (Some(x), Some(y)) => {
                    let x = x as u32 - '0' as u32;
                    let y = y as u32 - '0' as u32;
                    let mut n = x+y;
                    n += carry;
                    
                    result.push(char::from_digit(n%2,2).unwrap());
                    carry = n/2;
                }
                (Some(x), None) | (None, Some(x)) => {
                    let mut n = x as u32 - '0' as u32;
                    n += carry;
                    
                    result.push(char::from_digit(n%2,2).unwrap());
                    carry = n/2;
                }
                (None, None) => {
                    if carry == 1{
                        result.push('1');
                    }
                    break;
                }
                (_, _) => panic!(),
            }
        }
        result.into_iter().rev().collect()
    }
}