use std::collections::HashMap;
use std::cmp::{min,max};

impl Solution {
    pub fn multiply(mut num1: String, mut num2: String) -> String {
        let mut carry = 0;
        let mut prod = String::from("");
        let mut result = String::from("0");
        
        
        let mut mul = Mul::new();
        for (i,c) in num2.chars().rev().enumerate(){
            prod = mul.multiply_str(&num1, c);
            result = mul.add_shift_str(&result,&prod,i);
            //println!("{} => prod:{}, result:{}",c,prod,result);
        }
    
        
        result
    }
}

struct Mul{
    dp:HashMap<char, String>,
}

impl Mul{
    fn new() -> Self{
        Mul{
            dp: HashMap::new(),
        }
    }
    fn add_shift_str(&self, base:&str, s:&str, shift:usize) -> String{
        //println!("base:{}, s:{}, shift:{}",base,s,shift);
        if s == "0"{
            return base.to_string();
        }
        
        let mut base_iter = base.chars().rev();
        let mut s_iter = s.chars().rev();
        
        let mut result = String::from("");
        let mut carry = 0;
        
        for i in (0..max(base.len(),s.len()+shift)){
            if i < shift{
                match base_iter.next(){
                    None => result.push('0'),
                    Some(c) => result.push(c),
                }
            } else if i >= base.len(){
                if carry > 0{
                    let t = self.add_char(carry,'0',s_iter.next().unwrap());
                    carry = t.0;
                    result.push_str(&t.1);
                } else {
                    result.push(s_iter.next().unwrap());    
                }  
            } else if i >= s.len()+shift{
                if carry > 0{
                    let t = self.add_char(carry,'0',base_iter.next().unwrap());
                    carry = t.0;
                    result.push_str(&t.1);
                } else {
                    result.push(base_iter.next().unwrap());    
                }  
            } else {
                let t = self.add_char(carry,base_iter.next().unwrap(),s_iter.next().unwrap());
                carry = t.0;
                result.push_str(&t.1);
            }
            //println!{"result:{}", result};
        }
        if carry > 0{
            result.push_str(&carry.to_string());
        }
        result.chars().rev().collect()
    }
    fn multiply_str(&mut self, s:&str, c:char) -> String{
        if c == '0'{
            return String::from("0");
        }
        if self.dp.contains_key(&c){
            let prod = self.dp.get(&c).unwrap();
            return prod.to_string();
        }
        
        let mut result = String::from("");
        let mut carry = 0;
        let mut prod:String;
        for sc in s.chars().rev(){
            let t = self.multiply_char(carry, sc, c);
            carry = t.0;
            prod = t.1;
            result.push_str(&prod);
        }
        if carry > 0{
            result.push_str(&carry.to_string());
        }
        
        result = result.chars().rev().collect();
        self.dp.insert(c, result.clone());
        result
    }
    fn multiply_char(&self, carry:i32, a:char, b:char) -> (i32, String){
        let num1 = a.to_digit(10).unwrap();
        let num2 = b.to_digit(10).unwrap();
        let result = (num1*num2) as i32 + carry;
        return (result/10, format!("{}",result as u32 % 10));
    }
    fn add_char(&self, carry:i32, a:char, b:char) -> (i32, String){
        let num1 = a.to_digit(10).unwrap();
        let num2 = b.to_digit(10).unwrap();
        let result = (num1+num2) as i32 + carry;
        //println!{"carry:{} a:{} b:{} result:{}", carry,a,b,result};
        return (result/10, format!("{}",result as u32 % 10));
    }
}