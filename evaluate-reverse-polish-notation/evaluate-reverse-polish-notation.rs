impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        if tokens.len() == 1{
            return tokens[0].parse::<i32>().unwrap();
        }
        
        let mut stack:Vec<i32> = Vec::new();
        for token in tokens{
            match token.parse::<i32>(){
                Ok(num) => stack.push(num),
                Err(_) => {
                    let b:i32 = stack.pop().unwrap();
                    let a:i32 = stack.pop().unwrap();
                    let cal = match token.as_str(){
                        "+" => a+b,
                        "-" => a-b,
                        "*" => a*b,
                        "/" => a/b,
                        _ => panic!(),
                    };
                    stack.push(cal);
                }
            }
        }
        stack.pop().unwrap()
    }
}