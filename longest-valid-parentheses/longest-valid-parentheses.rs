use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut maxlen = 0;
        
        for c in s.chars(){
            match c{
                '(' => left+=1,
                ')' => right+=1,
                _=>panic!(),
            }
            if left == right{
                maxlen = max(maxlen, left*2);
            } else if left < right{
                left = 0;
                right = 0;
            }
        }
        
        left = 0;
        right = 0;
        for c in s.chars().rev(){
            match c{
                '(' => left+=1,
                ')' => right+=1,
                _=>panic!(),
            }
            if left == right{
                maxlen = max(maxlen, left*2);
            } else if left > right{
                left = 0;
                right = 0;
            }
        }
        
        maxlen
    }
}