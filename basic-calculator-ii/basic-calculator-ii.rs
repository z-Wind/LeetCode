#[derive(Debug)]
enum Cal{
    Num(i32),
    Op(u8),
}
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack:Vec<Cal> = Vec::new();
        let mut num = 0;
        let mut sign = 1;
        for c in s.as_bytes(){
            match c{
                b'0'..=b'9' => num = num*10 + (c-b'0') as i32,
                b'-' => {
                    match stack.last(){
                        Some(Cal::Op(b'*')) => {
                            stack.pop();
                            if let Cal::Num(x) = stack.pop().unwrap(){
                                num = x*num;   
                            }
                        },
                        Some(Cal::Op(b'/')) => {
                            stack.pop();
                            if let Cal::Num(x) = stack.pop().unwrap(){
                                num = x/num;    
                            }
                        },
                        _ => (),
                    }
                    stack.push(Cal::Num(num*sign));
                    stack.push(Cal::Op(b'+'));
                    num = 0;
                    sign = -1;
                },
                b'+' | b'*' | b'/' => {
                    match stack.last(){
                        Some(Cal::Op(b'*')) => {
                            stack.pop();
                            if let Cal::Num(x) = stack.pop().unwrap(){
                                num = x*num;   
                            }
                        },
                        Some(Cal::Op(b'/')) => {
                            stack.pop();
                            if let Cal::Num(x) = stack.pop().unwrap(){
                                num = x/num;    
                            }
                        },
                        _ => (),
                    }
                    stack.push(Cal::Num(num*sign));
                    stack.push(Cal::Op(*c));
                    num = 0;
                    sign = 1;
                },
                b' '=>(),
                _ => panic!(),
            }
        }
        // println!("{:?}",stack);
        num *= sign;
        while !stack.is_empty(){
            // println!("{}",num);
            match stack.pop(){
                Some(Cal::Op(b'+')) => {
                    if let Cal::Num(x) = stack.pop().unwrap(){
                        num = x+num;   
                    }
                },
                Some(Cal::Op(b'-')) => {
                    if let Cal::Num(x) = stack.pop().unwrap(){
                        num = x-num;   
                    }
                },
                Some(Cal::Op(b'*')) => {
                    if let Cal::Num(x) = stack.pop().unwrap(){
                        num = x*num;   
                    }
                },
                Some(Cal::Op(b'/')) => {
                    if let Cal::Num(x) = stack.pop().unwrap(){
                        num = x/num;   
                    }
                },
                unknown => panic!("{:?}",unknown),
            }
        }
        
        num
    }
}