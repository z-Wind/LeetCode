// https://leetcode.com/problems/fraction-to-recurring-decimal/discuss/51106/My-clean-Java-solution
use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0{
            return 0.to_string();
        }
        
        let mut result = String::new();
        let sign = match (numerator<0,denominator<0){
            (false,false) | (true,true) => (),
            _ => result.push('-'),
        };
        
        let mut numerator = (numerator as i64).abs();
        let mut denominator = (denominator as i64).abs();
        
        result.push_str(&(numerator/denominator).to_string());
        numerator %= denominator;
        if numerator == 0{
            return result;
        }
        result.push('.');
        
        let mut map = HashMap::new();
        let mut i = result.len();
        while numerator != 0{
            numerator *= 10;
            if map.contains_key(&numerator){
                i = *map.get(&numerator).unwrap();
                result.insert(i,'(');
                result.push(')');
                break;
            }
            map.insert(numerator,i);
            result.push_str(&(numerator/denominator).to_string());            
            i+=1;
            numerator %= denominator;
        }
        
        result
    }
}