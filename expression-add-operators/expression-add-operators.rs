use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Result{
    s:Vec<char>,
    val:i64,
    curr_num:Option<i64>,
    last_num:i64,
    op:char,
}
impl Result{
    fn new() -> Self{
        Self{
            s:vec![],
            val:0,
            curr_num:None,
            last_num:0,
            op:'+',
        }
    }
    
    fn cal(&mut self){
        match self.op{
            '+' => {
                self.val += self.last_num;
                self.last_num = self.curr_num.unwrap();
            },
            '-' => {
                self.val += self.last_num;
                self.last_num = -self.curr_num.unwrap();
            },
            '*' => {
                self.last_num *= self.curr_num.unwrap();
            },
            _ => panic!(),
        }
        self.val+=self.last_num;
    }
    
    fn operation(&mut self, c:char){
        self.s.push(c);
        match c{
            '0'..='9' => {
                self.curr_num = match self.curr_num{
                    None => Some( (c as u8 - b'0') as i64 ),
                    Some(x) => Some(x*10 + (c as u8 - b'0') as i64),
                };
            },
            '+' | '-' | '*' => {
                match self.op{
                    '+' => {
                        self.val += self.last_num;
                        self.last_num = self.curr_num.unwrap();
                    },
                    '-' => {
                        self.val += self.last_num;
                        self.last_num = -self.curr_num.unwrap();
                    },
                    '*' => {
                        self.last_num *= self.curr_num.unwrap();
                    },
                    _ => panic!(),
                }
                self.op = c;
                self.curr_num = None;
            },
            _ => panic!(),
        }
    }
}
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut results:VecDeque<Result> = VecDeque::new();
        results.push_back(Result::new());
        for (i, c) in num.chars().enumerate(){
            // println!("{:?}", results);
            if i == 0{
                results[0].operation(c);
                continue;
            }
            for _ in (0..results.len()){
                let result = results.pop_front().unwrap();
                // println!("{}: {:?}", c, result);
                // None
                if !(result.curr_num.is_some() && result.curr_num.unwrap() == 0){
                    let mut temp = result.clone();
                    temp.operation(c);
                    results.push_back(temp);
                }
                // +
                let mut temp = result.clone();
                temp.operation('+');
                temp.operation(c);
                results.push_back(temp);
                // -
                let mut temp = result.clone();
                temp.operation('-');
                temp.operation(c);
                results.push_back(temp);
                // *
                let mut temp = result.clone();
                temp.operation('*');
                temp.operation(c);
                results.push_back(temp);
            }
        }
        let mut ans = vec![];
        for mut result in results{
            result.cal();
            // println!("{}={}",result.s.clone().into_iter().collect::<String>(),result.val);
            if result.val == target as i64{
                ans.push(result.s.into_iter().collect());
            }
        }
    
        ans
    }
}